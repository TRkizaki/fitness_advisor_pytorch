use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct FormAnalysis {
    pub overall_score: f32,
    pub recommendations: Vec<String>,
    pub detected_errors: Vec<String>,
    pub confidence: f32,
}

pub struct AIMotionAnalyzer {
    python_script_path: String,
    realtime_script_path: String,
}

impl AIMotionAnalyzer {
    pub fn new() -> Self {
        Self {
            python_script_path: "ml_analyzer_test.py".to_string(),
            realtime_script_path: "realtime_analyzer.py".to_string(),
        }
    }

    pub async fn analyze_form(&self, video_data: &[u8]) -> Result<FormAnalysis> {
        use tokio::process::Command;
        use tokio::io::{AsyncWriteExt, AsyncReadExt};
        
        info!("🎥 Starting MediaPipe pose analysis...");
        
        let video_base64 = base64::prelude::Engine::encode(&base64::prelude::BASE64_STANDARD, video_data);
        
        let input_json = serde_json::json!({
            "video_base64": video_base64
        });
        
        let mut child = Command::new("python3")
            .arg(&self.python_script_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn Python process: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            let input_str = input_json.to_string();
            stdin.write_all(input_str.as_bytes()).await
                .map_err(|e| anyhow::anyhow!("Failed to write to Python process: {}", e))?;
            stdin.flush().await
                .map_err(|e| anyhow::anyhow!("Failed to flush stdin: {}", e))?;
        }

        let output = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            child.wait_with_output()
        ).await
        .map_err(|_| anyhow::anyhow!("Python process timed out after 30 seconds"))?
        .map_err(|e| anyhow::anyhow!("Python process failed: {}", e))?;

        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)
                .map_err(|e| anyhow::anyhow!("Invalid UTF-8 from Python: {}", e))?;
            
            info!("✅ Python analysis completed successfully");
            
            let python_result: serde_json::Value = serde_json::from_str(&output_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse Python output: {}", e))?;
            
            let form_analysis = FormAnalysis {
                overall_score: python_result["overall_score"].as_f64().unwrap_or(0.0) as f32,
                recommendations: python_result["recommendations"].as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect(),
                detected_errors: python_result["detected_errors"].as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect(),
                confidence: python_result["confidence"].as_f64().unwrap_or(0.0) as f32,
            };
            
            Ok(form_analysis)
        } else {
            let error_str = String::from_utf8(output.stderr).unwrap_or_default();
            warn!("❌ Python process failed with error: {}", error_str);
            Err(anyhow::anyhow!("Python analysis failed: {}", error_str))
        }
    }
    
    pub async fn analyze_frame_realtime(&self, frame_data: &[u8]) -> Result<serde_json::Value> {
        use tokio::process::Command;
        use tokio::io::AsyncWriteExt;
        
        let frame_base64 = base64::prelude::Engine::encode(&base64::prelude::BASE64_STANDARD, frame_data);
        
        let input_json = serde_json::json!({
            "frame_data": frame_base64
        });
        
        let mut child = Command::new("python3")
            .arg(&self.realtime_script_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn real-time analyzer: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            let input_str = input_json.to_string();
            stdin.write_all(input_str.as_bytes()).await
                .map_err(|e| anyhow::anyhow!("Failed to write to analyzer: {}", e))?;
            stdin.flush().await
                .map_err(|e| anyhow::anyhow!("Failed to flush stdin: {}", e))?;
        }

        let output = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            child.wait_with_output()
        ).await
        .map_err(|_| anyhow::anyhow!("Real-time analyzer timed out"))?
        .map_err(|e| anyhow::anyhow!("Real-time analyzer failed: {}", e))?;

        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)
                .map_err(|e| anyhow::anyhow!("Invalid UTF-8 from analyzer: {}", e))?;
            
            let result: serde_json::Value = serde_json::from_str(&output_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse analyzer output: {}", e))?;
            
            Ok(result)
        } else {
            let error_str = String::from_utf8(output.stderr).unwrap_or_default();
            Err(anyhow::anyhow!("Real-time analysis failed: {}", error_str))
        }
    }
}