// Simple test for Rust-Python integration
use std::process::Command;
use base64::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Rust-Python ML Integration");
    
    // Create test data (simple base64 encoded string)
    let test_data = b"Hello, this is test video data for ML analysis";
    let video_base64 = BASE64_STANDARD.encode(test_data);
    
    println!("📊 Test data size: {} bytes", test_data.len());
    println!("📋 Base64 length: {} chars", video_base64.len());
    
    // Create JSON input
    let input_json = serde_json::json!({
        "video_base64": video_base64
    });
    
    println!("🐍 Spawning Python process...");
    
    // Spawn Python process
    let mut child = tokio::process::Command::new("python3")
        .arg("ml_analyzer_test.py")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    // Write input to Python process
    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        let input_str = input_json.to_string();
        stdin.write_all(input_str.as_bytes()).await?;
        stdin.flush().await?;
    }

    // Wait for process to complete
    let output = child.wait_with_output().await?;

    if output.status.success() {
        let output_str = String::from_utf8(output.stdout)?;
        println!("✅ Python process completed successfully!");
        println!("📄 Python output:");
        println!("{}", output_str);
        
        // Parse the JSON response
        let result: serde_json::Value = serde_json::from_str(&output_str)?;
        
        println!("\n🎯 Parsed Results:");
        println!("• Overall Score: {}", result["overall_score"]);
        println!("• Exercise Type: {}", result["exercise_type"]);
        println!("• Confidence: {}", result["confidence"]);
        println!("• Test Status: {}", result["python_test"]);
        
        if let Some(recommendations) = result["recommendations"].as_array() {
            println!("• Recommendations:");
            for rec in recommendations {
                println!("  - {}", rec.as_str().unwrap_or(""));
            }
        }
        
        println!("\n🎉 Integration test PASSED!");
        
    } else {
        let error_str = String::from_utf8(output.stderr).unwrap_or_default();
        println!("❌ Python process failed:");
        println!("{}", error_str);
        return Err(format!("Python analysis failed: {}", error_str).into());
    }
    
    Ok(())
}