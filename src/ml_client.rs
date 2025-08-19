// src/ml_client.rs - HTTP client for Python ML service integration

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn, error};

#[derive(Debug, Clone)]
pub struct MLServiceClient {
    client: Client,
    base_url: String,
}

// Request/Response structures matching Python ML service
#[derive(Debug, Serialize)]
pub struct FrameAnalysisRequest {
    pub frame_data: String,  // Base64 encoded
    pub analysis_type: String,  // "realtime" or "detailed"
}

#[derive(Debug, Serialize)]
pub struct VideoAnalysisRequest {
    pub video_data: String,  // Base64 encoded
    pub analysis_type: String,  // "detailed" or "batch"
}

#[derive(Debug, Serialize)]
pub struct BatchAnalysisRequest {
    pub video_path: String,
}

#[derive(Debug, Deserialize)]
pub struct MLAnalysisResponse {
    pub success: bool,
    pub processing_time_ms: f64,
    pub result: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub timestamp: f64,
    pub models_loaded: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModelsStatusResponse {
    pub motion_analyzer: bool,
    pub realtime_analyzer: bool,
    pub batch_analyzer: bool,
    pub mediapipe_available: bool,
    pub pytorch_available: bool,
}

impl MLServiceClient {
    /// Create new ML service client
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))  // Default timeout
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }

    /// Create client with custom configuration
    pub fn with_config(base_url: String, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }

    /// Check if ML service is healthy
    pub async fn health_check(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Health check request failed: {}", e))?;

        if response.status().is_success() {
            let health: HealthResponse = response.json().await
                .map_err(|e| anyhow!("Failed to parse health response: {}", e))?;
            Ok(health)
        } else {
            Err(anyhow!("Health check failed with status: {}", response.status()))
        }
    }

    /// Get ML models status
    pub async fn models_status(&self) -> Result<ModelsStatusResponse> {
        let url = format!("{}/models/status", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Models status request failed: {}", e))?;

        if response.status().is_success() {
            let status: ModelsStatusResponse = response.json().await
                .map_err(|e| anyhow!("Failed to parse models status: {}", e))?;
            Ok(status)
        } else {
            Err(anyhow!("Models status failed with status: {}", response.status()))
        }
    }

    /// Analyze single frame for real-time feedback
    pub async fn analyze_frame_realtime(&self, frame_base64: String) -> Result<MLAnalysisResponse> {
        let request = FrameAnalysisRequest {
            frame_data: frame_base64,
            analysis_type: "realtime".to_string(),
        };

        self.analyze_frame_internal(request).await
    }

    /// Analyze single frame with detailed analysis
    pub async fn analyze_frame_detailed(&self, frame_base64: String) -> Result<MLAnalysisResponse> {
        let request = FrameAnalysisRequest {
            frame_data: frame_base64,
            analysis_type: "detailed".to_string(),
        };

        self.analyze_frame_internal(request).await
    }

    /// Internal frame analysis method
    async fn analyze_frame_internal(&self, request: FrameAnalysisRequest) -> Result<MLAnalysisResponse> {
        let url = format!("{}/analyze/frame", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Frame analysis request failed: {}", e))?;

        if response.status().is_success() {
            let result: MLAnalysisResponse = response.json().await
                .map_err(|e| anyhow!("Failed to parse frame analysis response: {}", e))?;
            Ok(result)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!("Frame analysis failed with status {}: {}", response.status(), error_text))
        }
    }

    /// Analyze video data
    pub async fn analyze_video(&self, video_base64: String, analysis_type: &str) -> Result<MLAnalysisResponse> {
        let request = VideoAnalysisRequest {
            video_data: video_base64,
            analysis_type: analysis_type.to_string(),
        };

        let url = format!("{}/analyze/video", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Video analysis request failed: {}", e))?;

        if response.status().is_success() {
            let result: MLAnalysisResponse = response.json().await
                .map_err(|e| anyhow!("Failed to parse video analysis response: {}", e))?;
            Ok(result)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!("Video analysis failed with status {}: {}", response.status(), error_text))
        }
    }

    /// Analyze batch video file
    pub async fn analyze_batch(&self, video_path: String) -> Result<MLAnalysisResponse> {
        let request = BatchAnalysisRequest { video_path };

        let url = format!("{}/analyze/batch", self.base_url);
        
        // Increase timeout for batch processing
        let response = self.client
            .post(&url)
            .timeout(Duration::from_secs(300))  // 5 minutes for batch analysis
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Batch analysis request failed: {}", e))?;

        if response.status().is_success() {
            let result: MLAnalysisResponse = response.json().await
                .map_err(|e| anyhow!("Failed to parse batch analysis response: {}", e))?;
            Ok(result)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!("Batch analysis failed with status {}: {}", response.status(), error_text))
        }
    }

    /// Utility method to check if ML service is available
    pub async fn is_available(&self) -> bool {
        match self.health_check().await {
            Ok(health) => {
                info!("ML service is healthy: {}", health.status);
                health.models_loaded
            }
            Err(e) => {
                warn!("ML service not available: {}", e);
                false
            }
        }
    }

    /// Get service information
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

// Helper functions for common ML operations
impl MLServiceClient {
    /// Quick pose analysis for WebSocket real-time feedback
    pub async fn quick_pose_analysis(&self, frame_base64: String) -> Option<QuickPoseResult> {
        match self.analyze_frame_realtime(frame_base64).await {
            Ok(response) => {
                if response.success {
                    // Extract key information for real-time feedback
                    let result = &response.result;
                    Some(QuickPoseResult {
                        score: result.get("score").and_then(|s| s.as_f64()).unwrap_or(0.0),
                        exercise: result.get("exercise").and_then(|e| e.as_str()).unwrap_or("unknown").to_string(),
                        feedback: result.get("feedback")
                            .and_then(|f| f.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                            .unwrap_or_default(),
                        warnings: result.get("warnings")
                            .and_then(|w| w.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                            .unwrap_or_default(),
                        processing_time_ms: response.processing_time_ms,
                    })
                } else {
                    error!("ML analysis failed: {:?}", response.error);
                    None
                }
            }
            Err(e) => {
                error!("Failed to get pose analysis: {}", e);
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuickPoseResult {
    pub score: f64,
    pub exercise: String,
    pub feedback: Vec<String>,
    pub warnings: Vec<String>,
    pub processing_time_ms: f64,
}

// Configuration for ML service
#[derive(Debug, Clone)]
pub struct MLServiceConfig {
    pub base_url: String,
    pub timeout_secs: u64,
    pub retry_attempts: u32,
    pub health_check_interval_secs: u64,
}

impl Default for MLServiceConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8001".to_string(),
            timeout_secs: 30,
            retry_attempts: 3,
            health_check_interval_secs: 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ml_client_creation() {
        let client = MLServiceClient::new("http://localhost:8001".to_string());
        assert_eq!(client.get_base_url(), "http://localhost:8001");
    }
    
    // Additional tests would require running ML service
    // For integration tests, see tests/integration_test.rs
}