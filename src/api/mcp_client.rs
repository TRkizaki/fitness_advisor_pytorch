use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub status: McpServerStatus,
    pub capabilities: Vec<McpCapability>,
    pub endpoint: String,
    pub last_heartbeat: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCall {
    pub id: String,
    pub server_id: String,
    pub tool_name: String,
    pub parameters: Value,
    pub context: Option<McpContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolResponse {
    pub call_id: String,
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceRequest {
    pub server_id: String,
    pub resource_uri: String,
    pub context: Option<McpContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub content_type: String,
    pub content: String,
    pub metadata: Option<Value>,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpContext {
    pub user_id: String,
    pub session_id: String,
    pub fitness_goals: Vec<String>,
    pub current_workout: Option<Value>,
    pub nutrition_preferences: Vec<String>,
    pub environment_context: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSessionRequest {
    pub server_id: String,
    pub context: McpContext,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSession {
    pub id: String,
    pub server_id: String,
    pub status: McpSessionStatus,
    pub context: McpContext,
    pub created_at: String,
    pub last_activity: String,
    pub tools_available: Vec<McpTool>,
    pub resources_available: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub category: McpToolCategory,
    pub required_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerStats {
    pub server_id: String,
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time_ms: f64,
    pub active_sessions: u32,
    pub tool_usage_stats: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerRegistration {
    pub name: String,
    pub description: String,
    pub endpoint: String,
    pub capabilities: Vec<McpCapability>,
    pub authentication: Option<McpAuthentication>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpAuthentication {
    pub auth_type: McpAuthType,
    pub credentials: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpServerStatus {
    Online,
    Offline,
    Maintenance,
    Error,
    Starting,
    Stopping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpCapability {
    Tools,
    Resources,
    Prompts,
    Sampling,
    Logging,
    Progress,
    Notifications,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpSessionStatus {
    Active,
    Inactive,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpToolCategory {
    FitnessTracking,
    NutritionAnalysis,
    WorkoutGeneration,
    ProgressMonitoring,
    DataAnalysis,
    Integration,
    Utility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpAuthType {
    None,
    ApiKey,
    Bearer,
    OAuth2,
    Certificate,
}

#[derive(Debug, thiserror::Error)]
pub enum McpApiError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parsing error: {0}")]
    Parse(String),
    #[error("MCP server error: {0}")]
    Server(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Session error: {0}")]
    Session(String),
}

impl From<JsValue> for McpApiError {
    fn from(err: JsValue) -> Self {
        McpApiError::Network(format!("{:?}", err))
    }
}

pub struct McpApiClient;

impl McpApiClient {
    const BASE_URL: &'static str = "http://localhost:3000/api/mcp";

    async fn make_request<T: for<'de> Deserialize<'de>>(
        endpoint: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<T, McpApiError> {
        let window = web_sys::window().unwrap();
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);

        if let Some(body_str) = body {
            opts.body(Some(&JsValue::from_str(body_str)));
        }

        let url = format!("{}{}", Self::BASE_URL, endpoint);
        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|e| McpApiError::Network(format!("{:?}", e)))?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|e| McpApiError::Network(format!("{:?}", e)))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| McpApiError::Network(format!("{:?}", e)))?;

        let resp: Response = resp_value.dyn_into().unwrap();

        if !resp.ok() {
            let status = resp.status();
            let text = JsFuture::from(resp.text().unwrap()).await
                .map_err(|e| McpApiError::Network(format!("{:?}", e)))?;
            let error_text = text.as_string().unwrap_or_default();
            return Err(McpApiError::Server(format!("HTTP {}: {}", status, error_text)));
        }

        let json = JsFuture::from(resp.json().unwrap())
            .await
            .map_err(|e| McpApiError::Parse(format!("{:?}", e)))?;

        let response_str = js_sys::JSON::stringify(&json)
            .map_err(|e| McpApiError::Parse(format!("{:?}", e)))?
            .as_string()
            .unwrap();

        serde_json::from_str(&response_str)
            .map_err(|e| McpApiError::Parse(e.to_string()))
    }

    // Server Management
    pub async fn get_servers() -> Result<Vec<McpServerInfo>, McpApiError> {
        Self::make_request("/servers", "GET", None).await
    }

    pub async fn get_server(server_id: &str) -> Result<McpServerInfo, McpApiError> {
        let endpoint = format!("/servers/{}", server_id);
        Self::make_request(&endpoint, "GET", None).await
    }

    pub async fn register_server(
        registration: McpServerRegistration,
    ) -> Result<McpServerInfo, McpApiError> {
        let body = serde_json::to_string(&registration)
            .map_err(|e| McpApiError::Parse(e.to_string()))?;
        
        Self::make_request("/servers", "POST", Some(&body)).await
    }

    pub async fn unregister_server(server_id: &str) -> Result<(), McpApiError> {
        let endpoint = format!("/servers/{}", server_id);
        let _: Value = Self::make_request(&endpoint, "DELETE", None).await?;
        Ok(())
    }

    pub async fn get_server_stats(server_id: &str) -> Result<McpServerStats, McpApiError> {
        let endpoint = format!("/servers/{}/stats", server_id);
        Self::make_request(&endpoint, "GET", None).await
    }

    // Session Management
    pub async fn create_session(
        request: McpSessionRequest,
    ) -> Result<McpSession, McpApiError> {
        let body = serde_json::to_string(&request)
            .map_err(|e| McpApiError::Parse(e.to_string()))?;
        
        Self::make_request("/sessions", "POST", Some(&body)).await
    }

    pub async fn get_session(session_id: &str) -> Result<McpSession, McpApiError> {
        let endpoint = format!("/sessions/{}", session_id);
        Self::make_request(&endpoint, "GET", None).await
    }

    pub async fn get_user_sessions(user_id: &str) -> Result<Vec<McpSession>, McpApiError> {
        let endpoint = format!("/sessions?user_id={}", user_id);
        Self::make_request(&endpoint, "GET", None).await
    }

    pub async fn terminate_session(session_id: &str) -> Result<(), McpApiError> {
        let endpoint = format!("/sessions/{}", session_id);
        let _: Value = Self::make_request(&endpoint, "DELETE", None).await?;
        Ok(())
    }

    // Tool Execution
    pub async fn call_tool(tool_call: McpToolCall) -> Result<McpToolResponse, McpApiError> {
        let body = serde_json::to_string(&tool_call)
            .map_err(|e| McpApiError::Parse(e.to_string()))?;
        
        Self::make_request("/tools/call", "POST", Some(&body)).await
    }

    pub async fn get_available_tools(server_id: &str) -> Result<Vec<McpTool>, McpApiError> {
        let endpoint = format!("/servers/{}/tools", server_id);
        Self::make_request(&endpoint, "GET", None).await
    }

    // Resource Access
    pub async fn get_resource(
        request: McpResourceRequest,
    ) -> Result<McpResource, McpApiError> {
        let body = serde_json::to_string(&request)
            .map_err(|e| McpApiError::Parse(e.to_string()))?;
        
        Self::make_request("/resources", "POST", Some(&body)).await
    }

    pub async fn list_resources(server_id: &str) -> Result<Vec<String>, McpApiError> {
        let endpoint = format!("/servers/{}/resources", server_id);
        Self::make_request(&endpoint, "GET", None).await
    }

    // Health and Monitoring
    pub async fn health_check() -> Result<Value, McpApiError> {
        Self::make_request("/health", "GET", None).await
    }

    pub async fn ping_server(server_id: &str) -> Result<Value, McpApiError> {
        let endpoint = format!("/servers/{}/ping", server_id);
        Self::make_request(&endpoint, "POST", None).await
    }

    // Utility methods
    pub fn create_fitness_context(
        user_id: String,
        session_id: String,
        goals: Vec<String>,
        workout: Option<Value>,
        nutrition_prefs: Vec<String>,
    ) -> McpContext {
        McpContext {
            user_id,
            session_id,
            fitness_goals: goals,
            current_workout: workout,
            nutrition_preferences: nutrition_prefs,
            environment_context: serde_json::json!({
                "app_version": "1.0.0",
                "platform": "web",
                "timestamp": js_sys::Date::now()
            }),
        }
    }

    pub fn create_tool_call(
        server_id: String,
        tool_name: String,
        parameters: Value,
        context: Option<McpContext>,
    ) -> McpToolCall {
        McpToolCall {
            id: format!("call_{}", js_sys::Math::random()),
            server_id,
            tool_name,
            parameters,
            context,
        }
    }
}