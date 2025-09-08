use axum::{
    extract::{Query, State, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::rag::{
    KnowledgeBase, LLMService,
    types::{SearchQuery, SearchResult, RAGResponse, Document}
};

pub type SharedKnowledgeBase = Arc<RwLock<KnowledgeBase>>;
pub type SharedLLMService = Arc<LLMService>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDocumentRequest {
    pub title: String,
    pub content: String,
    pub source: String,
    pub tags: Vec<String>,
    pub document_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUrlRequest {
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
    pub threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RAGQueryRequest {
    pub query: String,
    pub max_sources: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

pub struct AppState {
    pub knowledge_base: SharedKnowledgeBase,
    pub llm_service: SharedLLMService,
}

pub fn create_rag_router(app_state: AppState) -> Router {
    Router::new()
        .route("/documents", post(add_text_document))
        .route("/documents/url", post(add_url_document))
        .route("/documents/:id", get(get_document).delete(delete_document))
        .route("/documents", get(list_documents))
        .route("/search", post(semantic_search))
        .route("/query", post(rag_query))
        .route("/stats", get(get_stats))
        .with_state(app_state)
}

async fn add_text_document(
    State(state): State<AppState>,
    Json(request): Json<AddDocumentRequest>,
) -> Result<Json<ApiResponse<Uuid>>, StatusCode> {
    let mut kb = state.knowledge_base.write().await;
    
    match kb.add_text_document(&request.content, &request.title, &request.source, request.tags).await {
        Ok(document_id) => Ok(Json(ApiResponse::success(document_id))),
        Err(e) => {
            tracing::error!("Failed to add document: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to add document: {}", e))))
        }
    }
}

async fn add_url_document(
    State(state): State<AppState>,
    Json(request): Json<AddUrlRequest>,
) -> Result<Json<ApiResponse<Uuid>>, StatusCode> {
    let mut kb = state.knowledge_base.write().await;
    
    match kb.add_web_document(&request.url, &request.title, request.tags).await {
        Ok(document_id) => Ok(Json(ApiResponse::success(document_id))),
        Err(e) => {
            tracing::error!("Failed to add URL document: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to add URL document: {}", e))))
        }
    }
}

async fn get_document(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Document>>, StatusCode> {
    let kb = state.knowledge_base.read().await;
    
    match kb.get_document(document_id) {
        Some(document) => Ok(Json(ApiResponse::success(document.clone()))),
        None => Ok(Json(ApiResponse::error("Document not found".to_string()))),
    }
}

async fn delete_document(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let mut kb = state.knowledge_base.write().await;
    
    match kb.remove_document(document_id).await {
        Ok(()) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to delete document: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to delete document: {}", e))))
        }
    }
}

async fn list_documents(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Document>>>, StatusCode> {
    let kb = state.knowledge_base.read().await;
    let documents = kb.list_documents().to_vec();
    Ok(Json(ApiResponse::success(documents)))
}

async fn semantic_search(
    State(state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<ApiResponse<Vec<SearchResult>>>, StatusCode> {
    let mut kb = state.knowledge_base.write().await;
    
    let search_query = SearchQuery {
        query: request.query,
        limit: request.limit,
        filters: None,
        threshold: request.threshold,
    };
    
    match kb.search(&search_query).await {
        Ok(results) => Ok(Json(ApiResponse::success(results))),
        Err(e) => {
            tracing::error!("Search failed: {}", e);
            Ok(Json(ApiResponse::error(format!("Search failed: {}", e))))
        }
    }
}

async fn rag_query(
    State(state): State<AppState>,
    Json(request): Json<RAGQueryRequest>,
) -> Result<Json<ApiResponse<RAGResponse>>, StatusCode> {
    let max_sources = request.max_sources.unwrap_or(5);
    
    // First, get the knowledge base results
    let search_results = {
        let mut kb = state.knowledge_base.write().await;
        let search_query = SearchQuery {
            query: request.query.clone(),
            limit: Some(max_sources),
            filters: None,
            threshold: Some(0.5),
        };
        
        match kb.search(&search_query).await {
            Ok(results) => results,
            Err(e) => {
                tracing::error!("RAG search failed: {}", e);
                return Ok(Json(ApiResponse::error(format!("RAG search failed: {}", e))));
            }
        }
    };
    
    // Then, use LLM service to generate response
    match state.llm_service.generate_rag_response(&request.query, &search_results).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            tracing::error!("RAG response generation failed: {}", e);
            Ok(Json(ApiResponse::error(format!("RAG response generation failed: {}", e))))
        }
    }
}

async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let kb = state.knowledge_base.read().await;
    
    match kb.get_knowledge_base_stats().await {
        Ok(stats) => {
            let stats_json = serde_json::json!({
                "total_documents": stats.total_documents,
                "total_chunks": stats.total_chunks,
                "vector_dimension": stats.vector_dimension,
                "total_size_bytes": stats.total_size_bytes,
            });
            Ok(Json(ApiResponse::success(stats_json)))
        }
        Err(e) => {
            tracing::error!("Failed to get stats: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to get stats: {}", e))))
        }
    }
}

// Health check endpoint for RAG system
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "rag_system",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}