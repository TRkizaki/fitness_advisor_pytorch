#[cfg(test)]
mod api_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use axum_test::TestServer;
    use serde_json::json;
    use fitness_advisor_ai::rag::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;

    // Mock implementations for API testing
    struct MockKnowledgeBase {
        documents: Vec<Document>,
        search_results: Vec<SearchResult>,
    }

    impl MockKnowledgeBase {
        fn new() -> Self {
            Self {
                documents: Vec::new(),
                search_results: Vec::new(),
            }
        }

        async fn add_text_document(&mut self, content: &str, title: &str, source: &str, tags: Vec<String>) -> anyhow::Result<Uuid> {
            let document = Document {
                id: Uuid::new_v4(),
                title: title.to_string(),
                content: content.to_string(),
                source: source.to_string(),
                metadata: DocumentMetadata {
                    document_type: DocumentType::PlainText,
                    author: None,
                    tags,
                    language: "en".to_string(),
                    word_count: content.split_whitespace().count(),
                },
                created_at: chrono::Utc::now(),
            };

            let id = document.id;
            self.documents.push(document);
            Ok(id)
        }

        async fn add_web_document(&mut self, url: &str, title: &str, tags: Vec<String>) -> anyhow::Result<Uuid> {
            // Mock web content
            let content = format!("This is mock web content from {}", url);
            self.add_text_document(&content, title, url, tags).await
        }

        fn get_document(&self, document_id: Uuid) -> Option<&Document> {
            self.documents.iter().find(|doc| doc.id == document_id)
        }

        async fn remove_document(&mut self, document_id: Uuid) -> anyhow::Result<()> {
            self.documents.retain(|doc| doc.id != document_id);
            Ok(())
        }

        fn list_documents(&self) -> &[Document] {
            &self.documents
        }

        async fn search(&mut self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>> {
            // Mock search implementation
            let mock_results = self.documents
                .iter()
                .filter(|doc| {
                    let query_lower = query.query.to_lowercase();
                    doc.content.to_lowercase().contains(&query_lower) ||
                    doc.title.to_lowercase().contains(&query_lower)
                })
                .take(query.limit.unwrap_or(10))
                .map(|doc| {
                    let chunk = Chunk {
                        id: Uuid::new_v4(),
                        document_id: doc.id,
                        content: doc.content.chars().take(200).collect(),
                        embedding: None,
                        chunk_index: 0,
                        metadata: ChunkMetadata {
                            start_char: 0,
                            end_char: doc.content.len().min(200),
                            tokens: None,
                            semantic_level: "paragraph".to_string(),
                        },
                    };

                    SearchResult {
                        chunk,
                        score: 0.8, // Mock score
                        document: doc.clone(),
                    }
                })
                .collect();

            Ok(mock_results)
        }
    }

    struct MockLLMService;

    impl MockLLMService {
        fn new() -> Self {
            Self
        }

        async fn generate_rag_response(&self, query: &str, sources: &[SearchResult]) -> anyhow::Result<RAGResponse> {
            let answer = if sources.is_empty() {
                "I don't have enough information to answer that question.".to_string()
            } else {
                format!(
                    "Based on the available information, here's what I found about '{}': {}",
                    query,
                    sources.iter()
                        .map(|s| s.chunk.content.chars().take(100).collect::<String>())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            };

            let confidence = if sources.is_empty() { 0.2 } else { 0.8 };

            Ok(RAGResponse {
                answer,
                sources: sources.to_vec(),
                confidence,
            })
        }
    }

    fn create_test_app() -> TestServer {
        let mock_kb = Arc::new(RwLock::new(MockKnowledgeBase::new()));
        let mock_llm = Arc::new(MockLLMService::new());

        let app_state = AppState {
            knowledge_base: mock_kb,
            llm_service: mock_llm,
        };

        let app = create_rag_router(app_state);
        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn test_add_text_document() {
        let server = create_test_app();

        let request_body = json!({
            "title": "Test Fitness Article",
            "content": "Regular exercise is important for maintaining good health and fitness.",
            "source": "test://fitness-article",
            "tags": ["fitness", "health", "exercise"],
            "document_type": "PlainText"
        });

        let response = server
            .post("/documents")
            .json(&request_body)
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);
        assert!(body["data"].is_string()); // Should return UUID
        assert!(body["error"].is_null());
    }

    #[tokio::test]
    async fn test_add_url_document() {
        let server = create_test_app();

        let request_body = json!({
            "url": "https://example.com/nutrition-guide",
            "title": "Nutrition Guide",
            "tags": ["nutrition", "diet", "health"]
        });

        let response = server
            .post("/documents/url")
            .json(&request_body)
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);
        assert!(body["data"].is_string());
    }

    #[tokio::test]
    async fn test_list_documents() {
        let server = create_test_app();

        // First, add a document
        let add_request = json!({
            "title": "Exercise Benefits",
            "content": "Exercise provides numerous health benefits including improved cardiovascular health.",
            "source": "test://exercise-benefits",
            "tags": ["exercise", "benefits"],
            "document_type": "PlainText"
        });

        server.post("/documents").json(&add_request).await;

        // Then list documents
        let response = server.get("/documents").await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);
        assert!(body["data"].is_array());

        let documents = body["data"].as_array().unwrap();
        assert!(!documents.is_empty());

        let first_doc = &documents[0];
        assert_eq!(first_doc["title"], "Exercise Benefits");
    }

    #[tokio::test]
    async fn test_get_document() {
        let server = create_test_app();

        // Add a document first
        let add_request = json!({
            "title": "Strength Training",
            "content": "Strength training helps build muscle mass and increases bone density.",
            "source": "test://strength-training",
            "tags": ["strength", "training"],
            "document_type": "PlainText"
        });

        let add_response = server.post("/documents").json(&add_request).await;
        let add_body: serde_json::Value = add_response.json();
        let document_id = add_body["data"].as_str().unwrap();

        // Get the document
        let response = server
            .get(&format!("/documents/{}", document_id))
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);

        let document = &body["data"];
        assert_eq!(document["title"], "Strength Training");
        assert_eq!(document["id"], document_id);
    }

    #[tokio::test]
    async fn test_get_nonexistent_document() {
        let server = create_test_app();

        let fake_uuid = Uuid::new_v4().to_string();
        let response = server
            .get(&format!("/documents/{}", fake_uuid))
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], false);
        assert!(body["error"].as_str().unwrap().contains("not found"));
    }

    #[tokio::test]
    async fn test_delete_document() {
        let server = create_test_app();

        // Add a document first
        let add_request = json!({
            "title": "Cardio Exercise",
            "content": "Cardiovascular exercise strengthens the heart and improves circulation.",
            "source": "test://cardio-exercise",
            "tags": ["cardio", "heart"],
            "document_type": "PlainText"
        });

        let add_response = server.post("/documents").json(&add_request).await;
        let add_body: serde_json::Value = add_response.json();
        let document_id = add_body["data"].as_str().unwrap();

        // Delete the document
        let response = server
            .delete(&format!("/documents/{}", document_id))
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);

        // Verify document is deleted
        let get_response = server
            .get(&format!("/documents/{}", document_id))
            .await;

        let get_body: serde_json::Value = get_response.json();
        assert_eq!(get_body["success"], false);
    }

    #[tokio::test]
    async fn test_semantic_search() {
        let server = create_test_app();

        // Add documents first
        let documents = vec![
            ("Cardio Benefits", "Cardiovascular exercise improves heart health and endurance."),
            ("Strength Training", "Weight lifting builds muscle mass and increases strength."),
            ("Nutrition Guide", "Proper nutrition supports athletic performance and recovery."),
        ];

        for (title, content) in documents {
            let request = json!({
                "title": title,
                "content": content,
                "source": format!("test://{}", title.to_lowercase()),
                "tags": [],
                "document_type": "PlainText"
            });

            server.post("/documents").json(&request).await;
        }

        // Search for cardio-related content
        let search_request = json!({
            "query": "heart health and cardiovascular benefits",
            "limit": 5,
            "threshold": 0.1
        });

        let response = server
            .post("/search")
            .json(&search_request)
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);

        let results = body["data"].as_array().unwrap();
        assert!(!results.is_empty());

        // Should find cardio-related content
        let first_result = &results[0];
        assert!(first_result["score"].as_f64().unwrap() > 0.0);
    }

    #[tokio::test]
    async fn test_rag_query() {
        let server = create_test_app();

        // Add fitness content
        let add_request = json!({
            "title": "Exercise Guidelines",
            "content": "Adults should aim for at least 150 minutes of moderate-intensity aerobic activity or 75 minutes of vigorous-intensity aerobic activity per week, plus muscle-strengthening activities on 2 or more days per week.",
            "source": "test://exercise-guidelines",
            "tags": ["exercise", "guidelines", "fitness"],
            "document_type": "PlainText"
        });

        server.post("/documents").json(&add_request).await;

        // Ask a RAG question
        let query_request = json!({
            "query": "How much exercise should I do per week?",
            "max_sources": 3
        });

        let response = server
            .post("/query")
            .json(&query_request)
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);

        let rag_response = &body["data"];
        assert!(rag_response["answer"].is_string());
        assert!(rag_response["confidence"].as_f64().unwrap() > 0.0);
        assert!(rag_response["sources"].is_array());

        let answer = rag_response["answer"].as_str().unwrap();
        assert!(!answer.is_empty());
    }

    #[tokio::test]
    async fn test_rag_query_no_results() {
        let server = create_test_app();

        // Query without adding any documents
        let query_request = json!({
            "query": "What is quantum physics?",
            "max_sources": 5
        });

        let response = server
            .post("/query")
            .json(&query_request)
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);

        let rag_response = &body["data"];
        assert!(rag_response["confidence"].as_f64().unwrap() < 0.5);
        
        let sources = rag_response["sources"].as_array().unwrap();
        assert!(sources.is_empty());
    }

    #[tokio::test]
    async fn test_get_stats() {
        let server = create_test_app();

        // Add some documents first
        for i in 1..=3 {
            let request = json!({
                "title": format!("Document {}", i),
                "content": format!("This is the content of document number {}.", i),
                "source": format!("test://doc-{}", i),
                "tags": [],
                "document_type": "PlainText"
            });

            server.post("/documents").json(&request).await;
        }

        let response = server.get("/stats").await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);

        let stats = &body["data"];
        // Note: Our mock implementation doesn't track real stats,
        // but we can verify the structure
        assert!(stats.is_object());
    }

    #[tokio::test]
    async fn test_invalid_json_request() {
        let server = create_test_app();

        let response = server
            .post("/documents")
            .header("content-type", "application/json")
            .text("invalid json")
            .await;

        // Should return an error status for invalid JSON
        assert!(response.status_code().is_client_error());
    }

    #[tokio::test]
    async fn test_missing_required_fields() {
        let server = create_test_app();

        // Request missing required fields
        let incomplete_request = json!({
            "title": "Incomplete Document"
            // Missing content, source, tags
        });

        let response = server
            .post("/documents")
            .json(&incomplete_request)
            .await;

        // Should return an error for incomplete request
        assert!(response.status_code().is_client_error());
    }

    #[tokio::test]
    async fn test_search_with_parameters() {
        let server = create_test_app();

        // Add a document
        let add_request = json!({
            "title": "High-Intensity Training",
            "content": "High-intensity interval training (HIIT) is an effective way to improve cardiovascular fitness and burn calories efficiently.",
            "source": "test://hiit-training",
            "tags": ["hiit", "cardio", "training"],
            "document_type": "PlainText"
        });

        server.post("/documents").json(&add_request).await;

        // Search with different parameters
        let search_request = json!({
            "query": "high intensity training benefits",
            "limit": 1,
            "threshold": 0.5
        });

        let response = server
            .post("/search")
            .json(&search_request)
            .await;

        response.assert_status(StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["success"], true);

        let results = body["data"].as_array().unwrap();
        assert!(results.len() <= 1); // Respects limit
    }
}