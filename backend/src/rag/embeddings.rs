use anyhow::Result;
use ort::{Environment, ExecutionProvider, Session, SessionBuilder, Value};
use std::sync::Arc;
use tokenizers::Tokenizer;

pub struct EmbeddingService {
    session: Session,
    tokenizer: Tokenizer,
    max_length: usize,
}

impl EmbeddingService {
    pub async fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        let environment = Arc::new(Environment::builder().with_name("embeddings").build()?);
        
        let session = SessionBuilder::new(&environment)?
            .with_execution_providers([ExecutionProvider::cpu()])?
            .with_model_from_file(model_path)?;

        let tokenizer = Tokenizer::from_file(tokenizer_path)?;
        let max_length = 512; // Default max sequence length

        Ok(Self {
            session,
            tokenizer,
            max_length,
        })
    }

    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        // Tokenize the input text
        let encoding = self.tokenizer
            .encode(text, true)
            .map_err(|e| anyhow::anyhow!("Tokenization failed: {}", e))?;

        let mut tokens = encoding.get_ids().to_vec();
        
        // Truncate or pad to max_length
        tokens.resize(self.max_length, 0);
        
        // Create attention mask
        let attention_mask: Vec<i64> = tokens
            .iter()
            .map(|&token| if token != 0 { 1 } else { 0 })
            .collect();

        let input_ids: Vec<i64> = tokens.iter().map(|&x| x as i64).collect();

        // Create ONNX input tensors
        let input_ids_array = ndarray::Array::from_shape_vec(
            (1, self.max_length),
            input_ids,
        )?;
        
        let attention_mask_array = ndarray::Array::from_shape_vec(
            (1, self.max_length),
            attention_mask,
        )?;

        let inputs = vec![
            ("input_ids", Value::from_array(self.session.allocator(), &input_ids_array)?),
            ("attention_mask", Value::from_array(self.session.allocator(), &attention_mask_array)?),
        ];

        // Run inference
        let outputs = self.session.run(inputs)?;
        
        // Extract embeddings (assuming the model outputs pooled embeddings)
        if let Some(embedding_tensor) = outputs.get("last_hidden_state").or_else(|| outputs.get("pooler_output")) {
            let embedding_array = embedding_tensor.try_extract::<f32>()?;
            let embedding_vec = embedding_array.as_slice()?.to_vec();
            
            // If we got sequence outputs, we might need to pool them
            if embedding_vec.len() > 768 { // Common embedding dimension
                // Simple mean pooling for demonstration
                let embedding_dim = 768; // Adjust based on your model
                let pooled: Vec<f32> = (0..embedding_dim)
                    .map(|i| {
                        let start = i;
                        let mut sum = 0.0;
                        let mut count = 0;
                        for j in (start..embedding_vec.len()).step_by(embedding_dim) {
                            sum += embedding_vec[j];
                            count += 1;
                        }
                        sum / count as f32
                    })
                    .collect();
                Ok(pooled)
            } else {
                Ok(embedding_vec)
            }
        } else {
            Err(anyhow::anyhow!("No embedding output found"))
        }
    }

    pub fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        texts.iter()
            .map(|text| self.embed_text(text))
            .collect()
    }

    pub fn cosine_similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> f32 {
        let dot_product: f32 = embedding1.iter()
            .zip(embedding2.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm1: f32 = embedding1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = embedding2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            dot_product / (norm1 * norm2)
        }
    }
}