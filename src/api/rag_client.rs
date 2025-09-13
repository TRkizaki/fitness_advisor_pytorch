use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchRequest {
    pub query: String,
    pub document_types: Option<Vec<DocumentType>>,
    pub limit: Option<u32>,
    pub similarity_threshold: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchResult {
    pub document_id: String,
    pub title: String,
    pub content: String,
    pub document_type: DocumentType,
    pub similarity_score: f32,
    pub metadata: Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUploadRequest {
    pub title: String,
    pub content: String,
    pub document_type: DocumentType,
    pub metadata: Option<Value>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    pub document_type: DocumentType,
    pub metadata: Value,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub embedding_status: EmbeddingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRequest {
    pub user_context: UserContext,
    pub recommendation_type: RecommendationType,
    pub preferences: Option<Vec<String>>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRecommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub recommendation_type: RecommendationType,
    pub relevance_score: f32,
    pub supporting_documents: Vec<String>,
    pub action_items: Vec<String>,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeInsight {
    pub category: String,
    pub insight: String,
    pub confidence: f32,
    pub supporting_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub total_documents: u32,
    pub documents_by_type: Value,
    pub search_metrics: SearchMetrics,
    pub popular_topics: Vec<PopularTopic>,
    pub knowledge_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetrics {
    pub total_searches: u32,
    pub avg_results_per_search: f32,
    pub top_queries: Vec<TopQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopQuery {
    pub query: String,
    pub count: u32,
    pub avg_relevance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularTopic {
    pub topic: String,
    pub document_count: u32,
    pub relevance_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    FitnessGuide,
    NutritionInfo,
    ExerciseDescription,
    ResearchPaper,
    UserManual,
    FAQ,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingStatus {
    Pending,
    Processing,
    Complete,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationType {
    WorkoutPlan,
    NutritionAdvice,
    ExerciseForm,
    RecoveryTips,
    ProgressOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: String,
    pub fitness_goals: Vec<String>,
    pub current_stats: Value,
    pub preferences: Vec<String>,
    pub workout_history: Option<Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum RagApiError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parsing error: {0}")]
    Parse(String),
    #[error("API error: {0}")]
    Api(String),
}

impl From<JsValue> for RagApiError {
    fn from(err: JsValue) -> Self {
        RagApiError::Network(format!("{:?}", err))
    }
}

pub struct RagApiClient;

impl RagApiClient {
    const BASE_URL: &'static str = "http://localhost:3000/api/rag";

    async fn make_request<T: for<'de> Deserialize<'de>>(
        endpoint: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<T, RagApiError> {
        let window = web_sys::window().unwrap();
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);

        if let Some(body_str) = body {
            opts.body(Some(&JsValue::from_str(body_str)));
        }

        let url = format!("{}{}", Self::BASE_URL, endpoint);
        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|e| RagApiError::Network(format!("{:?}", e)))?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|e| RagApiError::Network(format!("{:?}", e)))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| RagApiError::Network(format!("{:?}", e)))?;

        let resp: Response = resp_value.dyn_into().unwrap();

        if !resp.ok() {
            let status = resp.status();
            let text = JsFuture::from(resp.text().unwrap()).await
                .map_err(|e| RagApiError::Network(format!("{:?}", e)))?;
            let error_text = text.as_string().unwrap_or_default();
            return Err(RagApiError::Api(format!("HTTP {}: {}", status, error_text)));
        }

        let json = JsFuture::from(resp.json().unwrap())
            .await
            .map_err(|e| RagApiError::Parse(format!("{:?}", e)))?;

        let response_str = js_sys::JSON::stringify(&json)
            .map_err(|e| RagApiError::Parse(format!("{:?}", e)))?
            .as_string()
            .unwrap();

        serde_json::from_str(&response_str)
            .map_err(|e| RagApiError::Parse(e.to_string()))
    }

    pub async fn semantic_search(
        request: SemanticSearchRequest,
    ) -> Result<Vec<SemanticSearchResult>, RagApiError> {
        let body = serde_json::to_string(&request)
            .map_err(|e| RagApiError::Parse(e.to_string()))?;

        Self::make_request("/search/semantic", "POST", Some(&body)).await
    }

    pub async fn upload_document(
        request: DocumentUploadRequest,
    ) -> Result<Document, RagApiError> {
        let body = serde_json::to_string(&request)
            .map_err(|e| RagApiError::Parse(e.to_string()))?;

        Self::make_request("/documents", "POST", Some(&body)).await
    }

    pub async fn get_documents(
        document_type: Option<DocumentType>,
    ) -> Result<Vec<Document>, RagApiError> {
        let endpoint = match document_type {
            Some(doc_type) => {
                let type_str = serde_json::to_string(&doc_type)
                    .map_err(|e| RagApiError::Parse(e.to_string()))?
                    .trim_matches('"')
                    .to_string();
                format!("/documents?type={}", type_str)
            }
            None => "/documents".to_string(),
        };

        Self::make_request(&endpoint, "GET", None).await
    }

    pub async fn get_document(id: &str) -> Result<Document, RagApiError> {
        let endpoint = format!("/documents/{}", id);
        Self::make_request(&endpoint, "GET", None).await
    }

    pub async fn delete_document(id: &str) -> Result<(), RagApiError> {
        let endpoint = format!("/documents/{}", id);
        let _: Value = Self::make_request(&endpoint, "DELETE", None).await?;
        Ok(())
    }

    pub async fn bulk_delete_documents(ids: Vec<String>) -> Result<(), RagApiError> {
        let body = serde_json::to_string(&ids)
            .map_err(|e| RagApiError::Parse(e.to_string()))?;

        let _: Value = Self::make_request("/documents/bulk-delete", "DELETE", Some(&body)).await?;
        Ok(())
    }

    pub async fn get_smart_recommendations(
        request: RecommendationRequest,
    ) -> Result<Vec<SmartRecommendation>, RagApiError> {
        let body = serde_json::to_string(&request)
            .map_err(|e| RagApiError::Parse(e.to_string()))?;

        Self::make_request("/recommendations", "POST", Some(&body)).await
    }

    pub async fn get_knowledge_insights(
        topics: Option<Vec<String>>,
    ) -> Result<Vec<KnowledgeInsight>, RagApiError> {
        let endpoint = match topics {
            Some(topic_list) => {
                let topics_str = topic_list.join(",");
                format!("/insights?topics={}", topics_str)
            }
            None => "/insights".to_string(),
        };

        Self::make_request(&endpoint, "GET", None).await
    }

    pub async fn get_analytics() -> Result<AnalyticsData, RagApiError> {
        Self::make_request("/analytics", "GET", None).await
    }

    pub async fn reindex_documents() -> Result<(), RagApiError> {
        let _: Value = Self::make_request("/admin/reindex", "POST", None).await?;
        Ok(())
    }
}