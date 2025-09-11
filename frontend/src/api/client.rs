// API client for Rust backend integration using Leptos
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit, RequestMode, Response, window};
use wasm_bindgen_futures::JsFuture;

const API_BASE_URL: &str = "/api"; // Proxied to localhost:3000

// User models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub height: u32,
    pub weight: u32,
    pub fitness_level: FitnessLevel,
    pub goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FitnessLevel {
    Beginner,
    Intermediate,
    Advanced,
    Elite,
}

// API Response types
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: String,
}

// Optimization request types
#[derive(Debug, Serialize)]
pub struct OptimizationRequest {
    pub user_id: String,
    pub time_horizon_days: u32,
    pub objectives: Vec<String>,
    pub constraints: OptimizationConstraints,
}

#[derive(Debug, Serialize)]
pub struct OptimizationConstraints {
    pub daily_calories: CalorieConstraints,
    pub macros: MacroConstraints,
}

#[derive(Debug, Serialize)]
pub struct CalorieConstraints {
    pub min: u32,
    pub max: u32,
    pub target: u32,
}

#[derive(Debug, Serialize)]
pub struct MacroConstraints {
    pub protein_g: MacroRange,
    pub carbs_g: MacroRange,
    pub fat_g: MacroRange,
}

#[derive(Debug, Serialize)]
pub struct MacroRange {
    pub min: u32,
    pub max: u32,
}

// Progress data types
#[derive(Debug, Deserialize)]
pub struct ProgressData {
    pub weight_history: Vec<WeightEntry>,
    pub workout_sessions: Vec<WorkoutSession>,
    pub strength_progress: Vec<StrengthEntry>,
}

#[derive(Debug, Deserialize)]
pub struct WeightEntry {
    pub date: String,
    pub weight: f32,
}

#[derive(Debug, Deserialize)]
pub struct WorkoutSession {
    pub date: String,
    pub calories_burned: u32,
    pub duration_minutes: u32,
    pub exercise_type: String,
}

#[derive(Debug, Deserialize)]
pub struct StrengthEntry {
    pub date: String,
    pub exercise: String,
    pub weight: f32,
    pub reps: u32,
    pub sets: u32,
}

// ML Analysis types
#[derive(Debug, Serialize)]
pub struct FrameAnalysisRequest {
    pub frame_base64: String,
    pub analysis_type: String,
}

#[derive(Debug, Deserialize)]
pub struct FrameAnalysisResponse {
    pub exercise_detected: Option<String>,
    pub form_score: f32,
    pub rep_count: u32,
    pub feedback: Vec<String>,
    pub metrics: FormMetrics,
}

#[derive(Debug, Deserialize)]
pub struct FormMetrics {
    pub squat_depth: f32,
    pub knee_alignment: f32,
    pub back_posture: f32,
    pub rep_tempo: f32,
}

// WebSocket message types
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: String,
}

// Main API Client
pub struct FitnessApiClient;

impl FitnessApiClient {
    // Generic HTTP request function
    async fn make_request<T: for<'de> Deserialize<'de>>(
        url: &str,
        method: &str,
        body: Option<String>,
    ) -> Result<T, JsValue> {
        let window = window().ok_or("No global window object")?;
        
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);
        
        if let Some(body_data) = body {
            opts.body(Some(&JsValue::from_str(&body_data)));
            // Set content-type header
            let headers = js_sys::Object::new();
            js_sys::Reflect::set(&headers, &"Content-Type".into(), &"application/json".into())?;
            opts.headers(&headers);
        }

        let request = Request::new_with_str_and_init(url, &opts)?;
        
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;
        
        if !resp.ok() {
            return Err(JsValue::from_str(&format!("HTTP Error: {}", resp.status())));
        }

        let json = JsFuture::from(resp.json()?).await?;
        
        // Convert JsValue to our type
        let data: T = serde_wasm_bindgen::from_value(json)
            .map_err(|e| JsValue::from_str(&format!("JSON parse error: {:?}", e)))?;
        
        Ok(data)
    }

    // User management
    pub async fn get_users() -> Result<Vec<User>, JsValue> {
        let url = format!("{}/users", API_BASE_URL);
        let response: ApiResponse<Vec<User>> = Self::make_request(&url, "GET", None).await?;
        Ok(response.data)
    }

    pub async fn get_user(user_id: &str) -> Result<Option<User>, JsValue> {
        let url = format!("{}/users/{}", API_BASE_URL, user_id);
        match Self::make_request::<ApiResponse<User>>(&url, "GET", None).await {
            Ok(response) => Ok(Some(response.data)),
            Err(_) => Ok(None), // User not found
        }
    }

    pub async fn get_user_recommendations(user_id: &str) -> Result<Vec<String>, JsValue> {
        let url = format!("{}/users/{}/recommendations", API_BASE_URL, user_id);
        let response: ApiResponse<Vec<String>> = Self::make_request(&url, "GET", None).await?;
        Ok(response.data)
    }

    pub async fn get_user_progress(user_id: &str) -> Result<ProgressData, JsValue> {
        let url = format!("{}/users/{}/progress", API_BASE_URL, user_id);
        let response: ApiResponse<ProgressData> = Self::make_request(&url, "GET", None).await?;
        Ok(response.data)
    }

    // Health checks
    pub async fn check_health() -> Result<String, JsValue> {
        let url = format!("{}/health", API_BASE_URL);
        let response: ApiResponse<String> = Self::make_request(&url, "GET", None).await?;
        Ok(response.data)
    }

    pub async fn check_ml_service_status() -> Result<bool, JsValue> {
        let url = format!("{}/ml/status", API_BASE_URL);
        match Self::make_request::<ApiResponse<serde_json::Value>>(&url, "GET", None).await {
            Ok(response) => {
                let available = response.data.get("available")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                Ok(available)
            }
            Err(_) => Ok(false),
        }
    }

    // Menu optimization
    pub async fn optimize_meal_plan(request: OptimizationRequest) -> Result<serde_json::Value, JsValue> {
        let url = format!("{}/menu/optimize", API_BASE_URL);
        let body = serde_json::to_string(&request)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))?;
        
        let response: ApiResponse<serde_json::Value> = Self::make_request(&url, "POST", Some(body)).await?;
        Ok(response.data)
    }

    // ML Analysis
    pub async fn analyze_frame(frame_base64: &str, analysis_type: &str) -> Result<FrameAnalysisResponse, JsValue> {
        let url = format!("{}/ml/analyze-frame", API_BASE_URL);
        let request = FrameAnalysisRequest {
            frame_base64: frame_base64.to_string(),
            analysis_type: analysis_type.to_string(),
        };
        
        let body = serde_json::to_string(&request)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))?;
        
        let response: ApiResponse<FrameAnalysisResponse> = Self::make_request(&url, "POST", Some(body)).await?;
        Ok(response.data)
    }

    // WebSocket connection (placeholder for future implementation)
    pub fn create_websocket() -> Result<web_sys::WebSocket, JsValue> {
        let ws_url = "ws://localhost:3000/api/ai/realtime";
        let ws = web_sys::WebSocket::new(ws_url)?;
        
        // Set up event handlers
        let onopen = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"WebSocket connection opened".into());
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();

        let onerror = Closure::wrap(Box::new(move |e: web_sys::ErrorEvent| {
            web_sys::console::log_1(&format!("WebSocket error: {:?}", e).into());
        }) as Box<dyn FnMut(web_sys::ErrorEvent)>);
        ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        let onclose = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"WebSocket connection closed".into());
        }) as Box<dyn FnMut()>);
        ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget();

        Ok(ws)
    }
}

// Utility functions for common operations
pub mod utils {
    use super::*;

    pub fn format_fitness_level(level: &FitnessLevel) -> &'static str {
        match level {
            FitnessLevel::Beginner => "Beginner",
            FitnessLevel::Intermediate => "Intermediate", 
            FitnessLevel::Advanced => "Advanced",
            FitnessLevel::Elite => "Elite",
        }
    }

    pub fn calculate_bmi(weight_kg: f32, height_cm: f32) -> f32 {
        let height_m = height_cm / 100.0;
        weight_kg / (height_m * height_m)
    }

    pub fn get_bmi_category(bmi: f32) -> &'static str {
        match bmi {
            bmi if bmi < 18.5 => "Underweight",
            bmi if bmi < 25.0 => "Normal weight",
            bmi if bmi < 30.0 => "Overweight",
            _ => "Obese",
        }
    }

    pub fn format_duration(minutes: u32) -> String {
        if minutes < 60 {
            format!("{}m", minutes)
        } else {
            let hours = minutes / 60;
            let remaining_minutes = minutes % 60;
            if remaining_minutes == 0 {
                format!("{}h", hours)
            } else {
                format!("{}h {}m", hours, remaining_minutes)
            }
        }
    }
}