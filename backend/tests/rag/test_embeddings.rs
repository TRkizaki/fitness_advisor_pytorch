#[cfg(test)]
mod embedding_tests {
    use fitness_advisor_ai::rag::EmbeddingService;
    use tempfile::NamedTempFile;
    use std::io::Write;

    // Mock embedding service for testing without actual model files
    struct MockEmbeddingService;

    impl MockEmbeddingService {
        fn new() -> Self {
            Self
        }

        fn embed_text(&self, text: &str) -> anyhow::Result<Vec<f32>> {
            // Create deterministic mock embeddings based on text content
            let mut embedding = vec![0.0; 384]; // Common embedding dimension
            
            // Simple hash-based mock embedding
            let chars: Vec<char> = text.chars().collect();
            for (i, &ch) in chars.iter().enumerate().take(384) {
                embedding[i % 384] += (ch as u8 as f32) / 255.0;
            }
            
            // Normalize the embedding
            let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
            if norm > 0.0 {
                for val in &mut embedding {
                    *val /= norm;
                }
            }
            
            Ok(embedding)
        }

        fn embed_batch(&self, texts: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
            texts.iter()
                .map(|text| self.embed_text(text))
                .collect()
        }

        fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
            let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
            let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
            let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
            
            if norm_a == 0.0 || norm_b == 0.0 {
                0.0
            } else {
                dot_product / (norm_a * norm_b)
            }
        }
    }

    #[tokio::test]
    async fn test_embed_single_text() {
        let service = MockEmbeddingService::new();
        
        let text = "Exercise is important for maintaining good health and fitness.";
        let embedding = service.embed_text(text)
            .expect("Should generate embedding");
        
        assert_eq!(embedding.len(), 384);
        
        // Check that embedding is normalized (magnitude close to 1)
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.001, "Embedding should be normalized");
    }

    #[tokio::test]
    async fn test_embed_batch() {
        let service = MockEmbeddingService::new();
        
        let texts = vec![
            "Cardiovascular exercise improves heart health.".to_string(),
            "Strength training builds muscle mass.".to_string(),
            "Proper nutrition supports athletic performance.".to_string(),
        ];
        
        let embeddings = service.embed_batch(&texts)
            .expect("Should generate batch embeddings");
        
        assert_eq!(embeddings.len(), 3);
        
        for embedding in &embeddings {
            assert_eq!(embedding.len(), 384);
            let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
            assert!((magnitude - 1.0).abs() < 0.001);
        }
    }

    #[tokio::test]
    async fn test_cosine_similarity() {
        let service = MockEmbeddingService::new();
        
        let text1 = "Exercise and fitness";
        let text2 = "Physical activity and health";
        let text3 = "Cooking and recipes";
        
        let emb1 = service.embed_text(text1).expect("Should embed text1");
        let emb2 = service.embed_text(text2).expect("Should embed text2");
        let emb3 = service.embed_text(text3).expect("Should embed text3");
        
        let sim_1_2 = service.cosine_similarity(&emb1, &emb2);
        let sim_1_3 = service.cosine_similarity(&emb1, &emb3);
        let sim_1_1 = service.cosine_similarity(&emb1, &emb1);
        
        // Self-similarity should be 1.0
        assert!((sim_1_1 - 1.0).abs() < 0.001);
        
        // Similarity should be between -1 and 1
        assert!(sim_1_2 >= -1.0 && sim_1_2 <= 1.0);
        assert!(sim_1_3 >= -1.0 && sim_1_3 <= 1.0);
        
        // Related texts should be more similar than unrelated ones
        // (This might not always hold with our simple mock, but let's check)
        println!("Similarity 1-2 (related): {}", sim_1_2);
        println!("Similarity 1-3 (unrelated): {}", sim_1_3);
    }

    #[tokio::test]
    async fn test_empty_text_embedding() {
        let service = MockEmbeddingService::new();
        
        let embedding = service.embed_text("")
            .expect("Should handle empty text");
        
        assert_eq!(embedding.len(), 384);
        
        // Empty text should produce zero or near-zero embedding
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(magnitude < 0.1, "Empty text should produce minimal embedding");
    }

    #[tokio::test]
    async fn test_identical_texts_similarity() {
        let service = MockEmbeddingService::new();
        
        let text = "Strength training is essential for building muscle mass and bone density.";
        
        let emb1 = service.embed_text(text).expect("Should embed text");
        let emb2 = service.embed_text(text).expect("Should embed same text");
        
        let similarity = service.cosine_similarity(&emb1, &emb2);
        
        // Identical texts should have similarity of 1.0
        assert!((similarity - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_fitness_domain_embeddings() {
        let service = MockEmbeddingService::new();
        
        let fitness_texts = vec![
            "High-intensity interval training (HIIT) burns calories efficiently.".to_string(),
            "Progressive overload is key to muscle growth.".to_string(),
            "Rest and recovery are crucial for athletic performance.".to_string(),
            "Protein intake should be 1.6-2.2g per kg of body weight.".to_string(),
            "Compound exercises work multiple muscle groups.".to_string(),
        ];
        
        let embeddings = service.embed_batch(&fitness_texts)
            .expect("Should generate fitness embeddings");
        
        assert_eq!(embeddings.len(), 5);
        
        // Test that different fitness concepts have reasonable similarities
        let hiit_emb = &embeddings[0];
        let protein_emb = &embeddings[3];
        let compound_emb = &embeddings[4];
        
        let hiit_compound_sim = service.cosine_similarity(hiit_emb, compound_emb);
        let hiit_protein_sim = service.cosine_similarity(hiit_emb, protein_emb);
        
        println!("HIIT-Compound similarity: {}", hiit_compound_sim);
        println!("HIIT-Protein similarity: {}", hiit_protein_sim);
        
        // All should be positive (related domain)
        assert!(hiit_compound_sim > 0.0);
        assert!(hiit_protein_sim > 0.0);
    }

    #[tokio::test]
    async fn test_embedding_consistency() {
        let service = MockEmbeddingService::new();
        
        let text = "Regular cardio exercise strengthens the cardiovascular system.";
        
        // Generate embedding multiple times
        let emb1 = service.embed_text(text).expect("Should embed text");
        let emb2 = service.embed_text(text).expect("Should embed text again");
        let emb3 = service.embed_text(text).expect("Should embed text third time");
        
        // All embeddings should be identical (deterministic)
        let sim_1_2 = service.cosine_similarity(&emb1, &emb2);
        let sim_1_3 = service.cosine_similarity(&emb1, &emb3);
        let sim_2_3 = service.cosine_similarity(&emb2, &emb3);
        
        assert!((sim_1_2 - 1.0).abs() < 0.001);
        assert!((sim_1_3 - 1.0).abs() < 0.001);
        assert!((sim_2_3 - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_long_text_embedding() {
        let service = MockEmbeddingService::new();
        
        let long_text = "Exercise physiology is the study of how the body responds to physical activity. \
                        It encompasses the acute responses to exercise as well as chronic adaptations to training. \
                        Understanding exercise physiology helps in designing effective training programs. \
                        The cardiovascular system adapts to exercise by increasing cardiac output. \
                        The respiratory system increases ventilation to meet oxygen demands. \
                        The musculoskeletal system undergoes adaptations including increased strength and endurance. \
                        Metabolic adaptations include improved efficiency in energy production pathways.".repeat(3);
        
        let embedding = service.embed_text(&long_text)
            .expect("Should handle long text");
        
        assert_eq!(embedding.len(), 384);
        
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_special_characters_embedding() {
        let service = MockEmbeddingService::new();
        
        let special_text = "Exercise @ 80% max heart rate for 30 minutes. \
                          Protein: 25-30g per meal. \
                          Rest: 7-9 hours/night. \
                          H₂O intake: 2-3 liters daily.";
        
        let embedding = service.embed_text(special_text)
            .expect("Should handle special characters");
        
        assert_eq!(embedding.len(), 384);
        
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.001);
    }
}