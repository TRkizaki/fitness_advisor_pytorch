// Batch Workout Session Processor - Rust Integration
use serde_json;
use std::path::Path;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏋️ Batch Workout Session Processor");
    println!("==================================");
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --bin batch_processor <video_path>");
        std::process::exit(1);
    }
    
    let video_path = &args[1];
    
    if !Path::new(video_path).exists() {
        println!("❌ Error: Video file {} not found", video_path);
        std::process::exit(1);
    }
    
    println!("🎬 Processing video: {}", video_path);
    let start_time = Instant::now();
    
    // Spawn Python batch analyzer
    let mut child = tokio::process::Command::new("python3")
        .arg("batch_analyzer.py")
        .arg(video_path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    // Wait for process to complete
    let output = child.wait_with_output().await?;
    let processing_time = start_time.elapsed();

    if output.status.success() {
        let output_str = String::from_utf8(output.stdout)?;
        let result: serde_json::Value = serde_json::from_str(&output_str)?;
        
        // Display summary
        if let Some(summary) = result["session_summary"].as_object() {
            println!("\n📊 WORKOUT SESSION ANALYSIS");
            println!("============================");
            
            if let Some(duration) = summary["total_duration"].as_f64() {
                println!("⏱️  Total Duration: {:.1} minutes", duration / 60.0);
            }
            
            if let Some(exercise_time) = summary["total_exercise_time"].as_f64() {
                println!("💪 Exercise Time: {:.1} minutes", exercise_time / 60.0);
            }
            
            if let Some(rest_time) = summary["rest_time"].as_f64() {
                println!("😴 Rest Time: {:.1} minutes", rest_time / 60.0);
            }
            
            if let Some(total_reps) = summary["total_reps"].as_u64() {
                println!("🔢 Total Reps: {}", total_reps);
            }
            
            if let Some(exercises) = summary["exercises_performed"].as_array() {
                println!("🎯 Exercises: {}", 
                    exercises.iter()
                        .map(|e| e.as_str().unwrap_or("unknown"))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            
            if let Some(breakdown) = summary["exercise_breakdown"].as_object() {
                println!("\n📋 Exercise Breakdown:");
                for (exercise, reps) in breakdown {
                    println!("  • {}: {} reps", exercise, reps.as_u64().unwrap_or(0));
                }
            }
        }
        
        // Display detailed analysis
        if let Some(detailed) = result["detailed_analysis"].as_array() {
            println!("\n🔍 DETAILED ANALYSIS");
            println!("====================");
            
            for (i, segment) in detailed.iter().enumerate() {
                if let Some(exercise) = segment["exercise"].as_str() {
                    let duration = segment["duration"].as_f64().unwrap_or(0.0);
                    let reps = segment["reps_counted"].as_u64().unwrap_or(0);
                    let start_time = segment["start_time"].as_f64().unwrap_or(0.0);
                    
                    println!("\n{}. {} ({:.1}s at {:.1}s)", 
                        i + 1, 
                        exercise.to_uppercase(), 
                        duration,
                        start_time
                    );
                    println!("   Reps: {}", reps);
                    
                    // Display fatigue analysis
                    if let Some(fatigue) = segment["fatigue_analysis"].as_object() {
                        let score = fatigue["fatigue_score"].as_f64().unwrap_or(0.0);
                        println!("   Fatigue Score: {:.2}", score);
                        
                        if let Some(indicators) = fatigue["indicators"].as_array() {
                            if !indicators.is_empty() {
                                println!("   ⚠️  Fatigue Indicators:");
                                for indicator in indicators {
                                    if let Some(text) = indicator.as_str() {
                                        println!("      • {}", text);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Performance stats
        println!("\n⚡ PROCESSING STATS");
        println!("==================");
        println!("Processing Time: {:.2}s", processing_time.as_secs_f64());
        
        if let Some(video_stats) = result["video_stats"].as_object() {
            if let Some(total_frames) = video_stats["total_frames"].as_u64() {
                println!("Total Frames: {}", total_frames);
            }
            if let Some(analyzed_frames) = video_stats["analyzed_frames"].as_u64() {
                println!("Analyzed Frames: {}", analyzed_frames);
            }
            if let Some(detection_rate) = video_stats["pose_detection_rate"].as_f64() {
                println!("Pose Detection Rate: {:.1}%", detection_rate * 100.0);
            }
        }
        
        // Save full results to file
        let output_file = format!("{}_analysis.json", 
            Path::new(video_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("workout")
        );
        
        std::fs::write(&output_file, serde_json::to_string_pretty(&result)?)?;
        println!("\n💾 Full analysis saved to: {}", output_file);
        
    } else {
        let error_str = String::from_utf8(output.stderr).unwrap_or_default();
        println!("❌ Batch processing failed:");
        println!("{}", error_str);
        std::process::exit(1);
    }
    
    Ok(())
}