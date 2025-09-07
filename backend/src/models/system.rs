use serde::{Deserialize, Serialize};

// AppState will be defined in main.rs since it needs complex imports

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
        }
    }
}

#[derive(Serialize)]
pub struct GpuStatus {
    pub gpu_available: bool,
    pub gpu_name: String,
    pub compute_capability: String,
    pub vram_total_mb: u32,
    pub vram_used_mb: u32,
    pub cuda_version: String,
    pub ready_for_ai: bool,
    pub features: Vec<String>,
}