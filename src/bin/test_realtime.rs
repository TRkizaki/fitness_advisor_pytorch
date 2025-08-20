// Test real-time analysis performance
use base64::prelude::*;
use std::time::Instant;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Real-time Analysis Performance Test");
    println!("=====================================");
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --bin test_realtime <image_path>");
        println!("Example: cargo run --bin test_realtime test_image.jpg");
        std::process::exit(1);
    }
    
    let image_path = &args[1];
    
    // Read test image
    let image_data = match std::fs::read(image_path) {
        Ok(data) => data,
        Err(e) => {
            println!("❌ Failed to read image {}: {}", image_path, e);
            std::process::exit(1);
        }
    };
    
    println!("📷 Testing with image: {} ({} bytes)", image_path, image_data.len());
    
    // Test multiple frames for performance measurement
    const NUM_FRAMES: usize = 10;
    let mut latencies = Vec::new();
    
    println!("\n🎬 Processing {} frames...", NUM_FRAMES);
    
    for i in 1..=NUM_FRAMES {
        let start_time = Instant::now();
        
        // Encode frame data
        let frame_base64 = BASE64_STANDARD.encode(&image_data);
        
        // Create input JSON
        let input_json = serde_json::json!({
            "frame_data": frame_base64
        });
        
        // Spawn Python real-time analyzer
        let mut child = tokio::process::Command::new("python3")
            .arg("realtime_analyzer.py")
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
        let total_latency = start_time.elapsed().as_millis();
        
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            let result: serde_json::Value = serde_json::from_str(&output_str)?;
            
            let processing_time = result["processing_time_ms"].as_f64().unwrap_or(0.0);
            let score = result["score"].as_u64().unwrap_or(0);
            let exercise = result["exercise"].as_str().unwrap_or("unknown");
            let within_target = result["performance"]["within_target"].as_bool().unwrap_or(false);
            
            let status_icon = if within_target { "✅" } else { "⚠️" };
            
            println!("Frame {}: {}ms total, {:.1}ms processing {} (Score: {}, Exercise: {})", 
                    i, total_latency, processing_time, status_icon, score, exercise);
            
            latencies.push(total_latency as f64);
        } else {
            let error_str = String::from_utf8(output.stderr).unwrap_or_default();
            println!("❌ Frame {} failed: {}", i, error_str);
        }
    }
    
    // Calculate statistics
    if !latencies.is_empty() {
        let avg_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
        let min_latency = latencies.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_latency = latencies.iter().fold(0.0f64, |a, &b| a.max(b));
        
        let within_target_count = latencies.iter().filter(|&&lat| lat < 50.0).count();
        let success_rate = (within_target_count as f64 / latencies.len() as f64) * 100.0;
        
        println!("\n📊 PERFORMANCE RESULTS");
        println!("======================");
        println!("Average Latency: {:.1}ms", avg_latency);
        println!("Min Latency: {:.1}ms", min_latency);
        println!("Max Latency: {:.1}ms", max_latency);
        println!("Success Rate (<50ms): {:.1}% ({}/{})", success_rate, within_target_count, latencies.len());
        
        if avg_latency < 50.0 {
            println!("🎯 ✅ Target achieved! Average latency under 50ms");
        } else {
            println!("🎯 ⚠️ Target missed. Average latency: {:.1}ms (target: <50ms)", avg_latency);
        }
        
        // Estimate max FPS
        let max_fps = 1000.0 / avg_latency;
        println!("📹 Estimated max FPS: {:.1}", max_fps);
        
        // Real-time streaming capability
        if avg_latency <= 33.0 {
            println!("🚀 Real-time streaming: Excellent (30+ FPS capable)");
        } else if avg_latency <= 50.0 {
            println!("⚡ Real-time streaming: Good (20+ FPS capable)");
        } else if avg_latency <= 100.0 {
            println!("🐌 Real-time streaming: Limited (10+ FPS capable)");
        } else {
            println!("❌ Real-time streaming: Not suitable for live analysis");
        }
    }
    
    Ok(())
}