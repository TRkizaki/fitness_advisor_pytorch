use anyhow::Result;
use crate::rag::{
    EmbeddingService, VectorStore, 
    types::{SearchQuery, SearchResult, Chunk, Document}
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct SemanticSearch {
    embedding_service: EmbeddingService,
    vector_store: VectorStore,
    document_cache: HashMap<Uuid, Document>,
}

impl SemanticSearch {
    pub fn new(embedding_service: EmbeddingService, vector_store: VectorStore) -> Self {
        Self {
            embedding_service,
            vector_store,
            document_cache: HashMap::new(),
        }
    }

    pub async fn search(&mut self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // Generate embedding for the search query
        let query_embedding = self.embedding_service.embed_text(&query.query)?;

        // Search vector store
        let scored_points = self.vector_store.search(query_embedding, query).await?;

        // Convert scored points to search results
        let mut results = Vec::new();

        for scored_point in scored_points {
            // Extract chunk information from payload
            let payload = scored_point.payload;
            
            let content = payload
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let document_id_str = payload
                .get("document_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let document_id = Uuid::parse_str(document_id_str)
                .map_err(|_| anyhow::anyhow!("Invalid document ID in vector store"))?;

            let chunk_index = payload
                .get("chunk_index")
                .and_then(|v| v.as_integer())
                .unwrap_or(0) as usize;

            let start_char = payload
                .get("start_char")
                .and_then(|v| v.as_integer())
                .unwrap_or(0) as usize;

            let end_char = payload
                .get("end_char")
                .and_then(|v| v.as_integer())
                .unwrap_or(0) as usize;

            let semantic_level = payload
                .get("semantic_level")
                .and_then(|v| v.as_str())
                .unwrap_or("paragraph")
                .to_string();

            // Create chunk from retrieved data
            let chunk = Chunk {
                id: Uuid::parse_str(&scored_point.id.to_string())
                    .unwrap_or_else(|_| Uuid::new_v4()),
                document_id,
                content,
                embedding: None, // We don't need to store the full embedding for results
                chunk_index,
                metadata: crate::rag::types::ChunkMetadata {
                    start_char,
                    end_char,
                    tokens: None,
                    semantic_level,
                },
            };

            // Get document from cache or placeholder
            let document = self.get_or_create_document_placeholder(document_id);

            let search_result = SearchResult {
                chunk,
                score: scored_point.score,
                document,
            };

            results.push(search_result);
        }

        Ok(results)
    }

    pub fn cache_document(&mut self, document: Document) {
        self.document_cache.insert(document.id, document);
    }

    pub fn cache_documents(&mut self, documents: Vec<Document>) {
        for document in documents {
            self.cache_document(document);
        }
    }

    fn get_or_create_document_placeholder(&self, document_id: Uuid) -> Document {
        self.document_cache
            .get(&document_id)
            .cloned()
            .unwrap_or_else(|| {
                // Create a placeholder document if not in cache
                Document {
                    id: document_id,
                    title: "Document".to_string(),
                    content: String::new(),
                    source: "unknown".to_string(),
                    metadata: crate::rag::types::DocumentMetadata {
                        document_type: crate::rag::types::DocumentType::PlainText,
                        author: None,
                        tags: vec![],
                        language: "en".to_string(),
                        word_count: 0,
                    },
                    created_at: chrono::Utc::now(),
                }
            })
    }

    pub async fn hybrid_search(&mut self, query: &SearchQuery, alpha: f32) -> Result<Vec<SearchResult>> {
        // For now, just do semantic search
        // In the future, this could combine semantic search with keyword-based search
        self.search(query).await
    }

    pub fn compute_similarity_scores(&self, query_embedding: &[f32], results: &mut [SearchResult]) {
        for result in results {
            if let Some(ref chunk_embedding) = result.chunk.embedding {
                let similarity = self.embedding_service.cosine_similarity(query_embedding, chunk_embedding);
                result.score = similarity;
            }
        }

        // Sort by score in descending order
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    }

    pub fn filter_by_threshold(results: Vec<SearchResult>, threshold: f32) -> Vec<SearchResult> {
        results
            .into_iter()
            .filter(|result| result.score >= threshold)
            .collect()
    }

    pub async fn search_with_context(&mut self, query: &SearchQuery, context_window: usize) -> Result<Vec<SearchResult>> {
        let mut results = self.search(query).await?;

        // For each result, try to expand context by including neighboring chunks
        for result in &mut results {
            // This is a simplified version - in practice, you'd want to retrieve
            // neighboring chunks from the vector store or a separate document store
            if context_window > 0 {
                // Add context expansion logic here
                // For now, we just return the original results
            }
        }

        Ok(results)
    }
}