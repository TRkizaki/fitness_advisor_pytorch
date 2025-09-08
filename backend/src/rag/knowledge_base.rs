use anyhow::Result;
use crate::rag::{
    EmbeddingService, VectorStore, DocumentProcessor, SemanticSearch,
    types::{Document, Chunk, SearchQuery, SearchResult, RAGResponse}
};
use std::path::Path;
use uuid::Uuid;

pub struct KnowledgeBase {
    embedding_service: EmbeddingService,
    vector_store: VectorStore,
    document_processor: DocumentProcessor,
    semantic_search: SemanticSearch,
    documents: Vec<Document>,
}

impl KnowledgeBase {
    pub fn new(
        embedding_service: EmbeddingService,
        vector_store: VectorStore,
        chunk_size: usize,
    ) -> Self {
        let document_processor = DocumentProcessor::new(chunk_size);
        let semantic_search = SemanticSearch::new(embedding_service.clone(), vector_store.clone());

        Self {
            embedding_service,
            vector_store,
            document_processor,
            semantic_search,
            documents: Vec::new(),
        }
    }

    pub async fn add_pdf_document<P: AsRef<Path>>(&mut self, path: P, title: &str, tags: Vec<String>) -> Result<Uuid> {
        let path_str = path.as_ref().to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;

        let mut document = self.document_processor.process_pdf(path_str, title, path_str)?;
        document.metadata.tags = tags;

        self.add_document_to_knowledge_base(document).await
    }

    pub async fn add_web_document(&mut self, url: &str, title: &str, tags: Vec<String>) -> Result<Uuid> {
        let html_content = self.document_processor.scrape_url(url).await?;
        let mut document = self.document_processor.process_html(&html_content, title, url)?;
        document.metadata.tags = tags;

        self.add_document_to_knowledge_base(document).await
    }

    pub async fn add_text_document(&mut self, content: &str, title: &str, source: &str, tags: Vec<String>) -> Result<Uuid> {
        let mut document = self.document_processor.process_text(
            content, 
            title, 
            source, 
            crate::rag::types::DocumentType::PlainText
        )?;
        document.metadata.tags = tags;

        self.add_document_to_knowledge_base(document).await
    }

    async fn add_document_to_knowledge_base(&mut self, document: Document) -> Result<Uuid> {
        let document_id = document.id;

        // Chunk the document
        let mut chunks = self.document_processor.chunk_document(&document)?;

        // Generate embeddings for each chunk
        let chunk_texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = self.embedding_service.embed_batch(&chunk_texts)?;

        // Add embeddings to chunks
        for (chunk, embedding) in chunks.iter_mut().zip(embeddings) {
            chunk.embedding = Some(embedding);
        }

        // Store chunks in vector database
        self.vector_store.upsert_chunks(chunks).await?;

        // Cache document for search results
        self.semantic_search.cache_document(document.clone());
        self.documents.push(document);

        Ok(document_id)
    }

    pub async fn search(&mut self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        self.semantic_search.search(query).await
    }

    pub async fn generate_rag_response(&mut self, query: &str, max_results: usize) -> Result<RAGResponse> {
        let search_query = SearchQuery {
            query: query.to_string(),
            limit: Some(max_results),
            filters: None,
            threshold: Some(0.5), // Minimum similarity threshold
        };

        let search_results = self.search(&search_query).await?;

        if search_results.is_empty() {
            return Ok(RAGResponse {
                answer: "I don't have enough information to answer that question.".to_string(),
                sources: vec![],
                confidence: 0.0,
            });
        }

        // Combine context from search results
        let context: String = search_results
            .iter()
            .map(|result| format!("Source: {}\n{}\n", result.document.title, result.chunk.content))
            .collect::<Vec<_>>()
            .join("\n---\n");

        // Calculate average confidence from search scores
        let confidence = search_results.iter()
            .map(|r| r.score)
            .sum::<f32>() / search_results.len() as f32;

        // For now, create a simple response based on the context
        // In a full implementation, you would use an LLM to generate the response
        let answer = self.generate_simple_answer(&context, query);

        Ok(RAGResponse {
            answer,
            sources: search_results,
            confidence,
        })
    }

    pub async fn remove_document(&mut self, document_id: Uuid) -> Result<()> {
        // Remove from vector store
        self.vector_store.delete_document(document_id).await?;

        // Remove from local cache
        self.documents.retain(|doc| doc.id != document_id);

        Ok(())
    }

    pub fn get_document(&self, document_id: Uuid) -> Option<&Document> {
        self.documents.iter().find(|doc| doc.id == document_id)
    }

    pub fn list_documents(&self) -> &[Document] {
        &self.documents
    }

    pub async fn get_knowledge_base_stats(&self) -> Result<KnowledgeBaseStats> {
        let collection_info = self.vector_store.get_collection_info().await?;
        
        Ok(KnowledgeBaseStats {
            total_documents: self.documents.len(),
            total_chunks: collection_info.points_count.unwrap_or(0) as usize,
            vector_dimension: collection_info.config
                .and_then(|c| c.params)
                .and_then(|p| match p.vectors_config {
                    Some(qdrant_client::qdrant::vectors_config::Config::Params(params)) => Some(params.size),
                    _ => None,
                })
                .unwrap_or(0) as usize,
            total_size_bytes: self.calculate_total_size(),
        })
    }

    fn generate_simple_answer(&self, context: &str, query: &str) -> String {
        // This is a placeholder for LLM integration
        // In a real implementation, you would send the context and query to an LLM
        
        if context.is_empty() {
            "I don't have enough information to answer that question.".to_string()
        } else {
            format!(
                "Based on the available information, here's what I found related to your query about '{}':\n\n{}",
                query,
                context.chars().take(500).collect::<String>() + if context.len() > 500 { "..." } else { "" }
            )
        }
    }

    fn calculate_total_size(&self) -> usize {
        self.documents
            .iter()
            .map(|doc| doc.content.len())
            .sum()
    }

    pub async fn bulk_import_directory<P: AsRef<Path>>(&mut self, directory: P, supported_extensions: Vec<&str>) -> Result<Vec<Uuid>> {
        let mut imported_ids = Vec::new();
        let dir_path = directory.as_ref();

        if !dir_path.is_dir() {
            return Err(anyhow::anyhow!("Path is not a directory"));
        }

        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    if supported_extensions.contains(&extension.to_lowercase().as_str()) {
                        match extension.to_lowercase().as_str() {
                            "pdf" => {
                                let title = path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                
                                if let Ok(doc_id) = self.add_pdf_document(&path, &title, vec![]).await {
                                    imported_ids.push(doc_id);
                                }
                            },
                            "txt" => {
                                let content = std::fs::read_to_string(&path)?;
                                let title = path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                
                                if let Ok(doc_id) = self.add_text_document(&content, &title, path.to_str().unwrap_or(""), vec![]).await {
                                    imported_ids.push(doc_id);
                                }
                            },
                            _ => {
                                // Skip unsupported file types
                            }
                        }
                    }
                }
            }
        }

        Ok(imported_ids)
    }
}

#[derive(Debug)]
pub struct KnowledgeBaseStats {
    pub total_documents: usize,
    pub total_chunks: usize,
    pub vector_dimension: usize,
    pub total_size_bytes: usize,
}