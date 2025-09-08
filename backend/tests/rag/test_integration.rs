#[cfg(test)]
mod integration_tests {
    use fitness_advisor_ai::rag::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;

    // Mock implementations for integration testing
    
    struct MockEmbeddingService;
    
    impl MockEmbeddingService {
        fn new() -> Self {
            Self
        }
        
        fn embed_text(&self, text: &str) -> anyhow::Result<Vec<f32>> {
            let mut embedding = vec![0.0; 384];
            let chars: Vec<char> = text.chars().collect();
            for (i, &ch) in chars.iter().enumerate().take(384) {
                embedding[i % 384] += (ch as u8 as f32) / 255.0;
            }
            
            let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
            if norm > 0.0 {
                for val in &mut embedding {
                    *val /= norm;
                }
            }
            
            Ok(embedding)
        }
        
        fn embed_batch(&self, texts: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
            texts.iter().map(|text| self.embed_text(text)).collect()
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

    // Mock vector store that stores vectors in memory
    struct MockVectorStore {
        points: Arc<RwLock<Vec<(String, Vec<f32>, std::collections::HashMap<String, String>)>>>,
        dimension: u64,
    }
    
    impl MockVectorStore {
        async fn new(_url: &str, _collection_name: &str, dimension: u64) -> anyhow::Result<Self> {
            Ok(Self {
                points: Arc::new(RwLock::new(Vec::new())),
                dimension,
            })
        }
        
        async fn upsert_chunks(&self, chunks: Vec<Chunk>) -> anyhow::Result<()> {
            let mut points = self.points.write().await;
            
            for chunk in chunks {
                if let Some(embedding) = chunk.embedding {
                    let mut payload = std::collections::HashMap::new();
                    payload.insert("content".to_string(), chunk.content);
                    payload.insert("document_id".to_string(), chunk.document_id.to_string());
                    payload.insert("chunk_index".to_string(), chunk.chunk_index.to_string());
                    
                    points.push((chunk.id.to_string(), embedding, payload));
                }
            }
            
            Ok(())
        }
        
        async fn search(&self, query_embedding: Vec<f32>, query: &SearchQuery) -> anyhow::Result<Vec<MockScoredPoint>> {
            let points = self.points.read().await;
            let mut results = Vec::new();
            
            for (id, embedding, payload) in points.iter() {
                let score = self.cosine_similarity(&query_embedding, embedding);
                
                if let Some(threshold) = query.threshold {
                    if score < threshold {
                        continue;
                    }
                }
                
                results.push(MockScoredPoint {
                    id: id.clone(),
                    score,
                    payload: payload.clone(),
                });
            }
            
            results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
            
            if let Some(limit) = query.limit {
                results.truncate(limit);
            }
            
            Ok(results)
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
    
    #[derive(Debug)]
    struct MockScoredPoint {
        id: String,
        score: f32,
        payload: std::collections::HashMap<String, String>,
    }

    async fn setup_test_knowledge_base() -> (MockEmbeddingService, MockVectorStore, DocumentProcessor) {
        let embedding_service = MockEmbeddingService::new();
        let vector_store = MockVectorStore::new("mock://localhost", "test_collection", 384)
            .await
            .expect("Should create mock vector store");
        let document_processor = DocumentProcessor::new(200); // Small chunks for testing
        
        (embedding_service, vector_store, document_processor)
    }

    #[tokio::test]
    async fn test_full_document_pipeline() {
        let (embedding_service, vector_store, document_processor) = setup_test_knowledge_base().await;
        
        // Create a fitness document
        let fitness_content = "
        Cardiovascular exercise, also known as cardio, is any exercise that raises your heart rate. 
        It includes activities like running, cycling, swimming, and dancing. Regular cardio exercise 
        strengthens your heart muscle, improves circulation, and helps maintain a healthy weight.
        
        Strength training involves exercises that improve muscular strength and endurance. This includes 
        weight lifting, resistance band exercises, and bodyweight exercises like push-ups and squats. 
        Strength training helps build muscle mass, increases bone density, and boosts metabolism.
        
        Proper nutrition is essential for athletic performance and recovery. A balanced diet should 
        include adequate protein for muscle repair, carbohydrates for energy, and healthy fats for 
        hormone production. Hydration is also crucial for optimal performance.
        ";
        
        let document = document_processor.process_text(
            fitness_content,
            "Complete Fitness Guide",
            "test://fitness-guide",
            DocumentType::Exercise
        ).expect("Should create document");
        
        // Chunk the document
        let mut chunks = document_processor.chunk_document(&document)
            .expect("Should chunk document");
        
        assert!(!chunks.is_empty());
        
        // Generate embeddings for chunks
        let chunk_texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = embedding_service.embed_batch(&chunk_texts)
            .expect("Should generate embeddings");
        
        // Add embeddings to chunks
        for (chunk, embedding) in chunks.iter_mut().zip(embeddings) {
            chunk.embedding = Some(embedding);
        }
        
        // Store in vector database
        vector_store.upsert_chunks(chunks.clone())
            .await
            .expect("Should store chunks");
        
        // Test semantic search
        let cardio_query = SearchQuery {
            query: "What are the benefits of cardiovascular exercise?".to_string(),
            limit: Some(5),
            filters: None,
            threshold: Some(0.1),
        };
        
        let cardio_query_embedding = embedding_service.embed_text(&cardio_query.query)
            .expect("Should generate query embedding");
        
        let search_results = vector_store.search(cardio_query_embedding, &cardio_query)
            .await
            .expect("Should search successfully");
        
        assert!(!search_results.is_empty());
        
        // Verify that cardio-related content scores higher
        let top_result = &search_results[0];
        assert!(top_result.score > 0.0);
        
        let content = top_result.payload.get("content").expect("Should have content");
        assert!(content.to_lowercase().contains("cardio") || 
                content.to_lowercase().contains("heart") ||
                content.to_lowercase().contains("exercise"));
    }

    #[tokio::test]
    async fn test_multiple_documents_search() {
        let (embedding_service, vector_store, document_processor) = setup_test_knowledge_base().await;
        
        // Create multiple fitness documents
        let documents = vec![
            ("Cardio Benefits", "Cardiovascular exercise improves heart health, increases endurance, and burns calories efficiently. Running, cycling, and swimming are excellent cardio exercises."),
            ("Strength Training", "Resistance training builds muscle mass, increases bone density, and improves metabolic rate. Focus on compound movements like squats, deadlifts, and bench press."),
            ("Nutrition Basics", "Proper nutrition includes balanced macronutrients: proteins for muscle repair, carbohydrates for energy, and healthy fats for hormone production and nutrient absorption."),
            ("Recovery Importance", "Rest and recovery are essential for muscle growth and performance improvement. Aim for 7-9 hours of quality sleep and include rest days in your training schedule."),
        ];
        
        let mut all_chunks = Vec::new();
        
        // Process each document
        for (title, content) in documents {
            let doc = document_processor.process_text(
                content,
                title,
                &format!("test://{}", title.to_lowercase()),
                DocumentType::Exercise
            ).expect("Should create document");
            
            let mut chunks = document_processor.chunk_document(&doc)
                .expect("Should chunk document");
            
            // Generate embeddings
            let chunk_texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
            let embeddings = embedding_service.embed_batch(&chunk_texts)
                .expect("Should generate embeddings");
            
            // Add embeddings to chunks
            for (chunk, embedding) in chunks.iter_mut().zip(embeddings) {
                chunk.embedding = Some(embedding);
            }
            
            all_chunks.extend(chunks);
        }
        
        // Store all chunks
        vector_store.upsert_chunks(all_chunks)
            .await
            .expect("Should store all chunks");
        
        // Test different types of queries
        let queries = vec![
            ("How to build muscle?", vec!["strength", "muscle", "resistance"]),
            ("What should I eat for energy?", vec!["nutrition", "carbohydrate", "energy"]),
            ("How to improve heart health?", vec!["cardio", "heart", "cardiovascular"]),
            ("Why is sleep important?", vec!["recovery", "sleep", "rest"]),
        ];
        
        for (query_text, expected_keywords) in queries {
            let query = SearchQuery {
                query: query_text.to_string(),
                limit: Some(3),
                filters: None,
                threshold: Some(0.1),
            };
            
            let query_embedding = embedding_service.embed_text(&query.query)
                .expect("Should generate query embedding");
            
            let results = vector_store.search(query_embedding, &query)
                .await
                .expect("Should search successfully");
            
            assert!(!results.is_empty(), "Query '{}' should return results", query_text);
            
            // Check that top result contains relevant keywords
            let top_content = results[0].payload.get("content")
                .expect("Should have content").to_lowercase();
            
            let has_relevant_keyword = expected_keywords.iter()
                .any(|keyword| top_content.contains(keyword));
            
            assert!(has_relevant_keyword, 
                "Query '{}' should return content with keywords {:?}, but got: {}", 
                query_text, expected_keywords, top_content);
        }
    }

    #[tokio::test]
    async fn test_search_with_threshold() {
        let (embedding_service, vector_store, document_processor) = setup_test_knowledge_base().await;
        
        // Create documents with very different content
        let documents = vec![
            ("Fitness", "Exercise regularly to maintain good health and build strength through consistent training."),
            ("Cooking", "Prepare delicious meals using fresh ingredients and traditional cooking techniques for family dinner."),
            ("Technology", "Programming languages like Rust provide memory safety and performance for system development."),
        ];
        
        let mut all_chunks = Vec::new();
        
        for (title, content) in documents {
            let doc = document_processor.process_text(
                content,
                title,
                &format!("test://{}", title),
                DocumentType::PlainText
            ).expect("Should create document");
            
            let mut chunks = document_processor.chunk_document(&doc)
                .expect("Should chunk document");
            
            let chunk_texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
            let embeddings = embedding_service.embed_batch(&chunk_texts)
                .expect("Should generate embeddings");
            
            for (chunk, embedding) in chunks.iter_mut().zip(embeddings) {
                chunk.embedding = Some(embedding);
            }
            
            all_chunks.extend(chunks);
        }
        
        vector_store.upsert_chunks(all_chunks)
            .await
            .expect("Should store chunks");
        
        // Search for fitness-related content with different thresholds
        let fitness_query = "exercise and training for health";
        let query_embedding = embedding_service.embed_text(fitness_query)
            .expect("Should generate query embedding");
        
        // Low threshold - should return more results
        let low_threshold_query = SearchQuery {
            query: fitness_query.to_string(),
            limit: Some(10),
            filters: None,
            threshold: Some(0.01), // Very low threshold
        };
        
        let low_threshold_results = vector_store.search(query_embedding.clone(), &low_threshold_query)
            .await
            .expect("Should search with low threshold");
        
        // High threshold - should return fewer, more relevant results
        let high_threshold_query = SearchQuery {
            query: fitness_query.to_string(),
            limit: Some(10),
            filters: None,
            threshold: Some(0.7), // High threshold
        };
        
        let high_threshold_results = vector_store.search(query_embedding, &high_threshold_query)
            .await
            .expect("Should search with high threshold");
        
        // High threshold should return fewer or equal results
        assert!(high_threshold_results.len() <= low_threshold_results.len());
        
        // If high threshold returns results, they should be more relevant
        if !high_threshold_results.is_empty() {
            assert!(high_threshold_results[0].score >= 0.7);
        }
    }

    #[tokio::test]
    async fn test_search_result_ranking() {
        let (embedding_service, vector_store, document_processor) = setup_test_knowledge_base().await;
        
        // Create documents with varying relevance to a query
        let documents = vec![
            ("Exact Match", "High intensity interval training HIIT is excellent for cardiovascular fitness and fat burning."),
            ("Related", "Cardio exercises like running and cycling improve heart health and endurance significantly."),
            ("Somewhat Related", "Physical activity and regular movement contribute to overall wellness and health benefits."),
            ("Unrelated", "Computer programming requires logical thinking and problem-solving skills in software development."),
        ];
        
        let mut all_chunks = Vec::new();
        
        for (title, content) in documents {
            let doc = document_processor.process_text(
                content,
                title,
                &format!("test://{}", title),
                DocumentType::PlainText
            ).expect("Should create document");
            
            let mut chunks = document_processor.chunk_document(&doc)
                .expect("Should chunk document");
            
            let chunk_texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
            let embeddings = embedding_service.embed_batch(&chunk_texts)
                .expect("Should generate embeddings");
            
            for (chunk, embedding) in chunks.iter_mut().zip(embeddings) {
                chunk.embedding = Some(embedding);
            }
            
            all_chunks.extend(chunks);
        }
        
        vector_store.upsert_chunks(all_chunks)
            .await
            .expect("Should store chunks");
        
        // Search for HIIT-related content
        let query = SearchQuery {
            query: "high intensity interval training benefits".to_string(),
            limit: Some(4),
            filters: None,
            threshold: Some(0.0),
        };
        
        let query_embedding = embedding_service.embed_text(&query.query)
            .expect("Should generate query embedding");
        
        let results = vector_store.search(query_embedding, &query)
            .await
            .expect("Should search successfully");
        
        assert!(!results.is_empty());
        
        // Verify results are ranked by similarity (descending order)
        for i in 1..results.len() {
            assert!(results[i-1].score >= results[i].score,
                "Results should be ranked by similarity score");
        }
        
        // The most relevant content should rank highest
        let top_content = results[0].payload.get("content")
            .expect("Should have content").to_lowercase();
        
        assert!(top_content.contains("hiit") || top_content.contains("interval"));
    }
}