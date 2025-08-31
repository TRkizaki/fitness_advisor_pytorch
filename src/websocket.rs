use std::sync::Arc;
use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use tracing::{info, warn};
use anyhow::Result;

use crate::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    info!("🔗 WebSocket connection established for real-time analysis");
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket, state: Arc<AppState>) {
    use axum::extract::ws::{Message, WebSocket};
    use futures_util::{SinkExt, StreamExt};
    
    let (mut sender, mut receiver) = socket.split();
    
    info!("🎥 Real-time analysis session started");
    
    let welcome = serde_json::json!({
        "type": "welcome",
        "message": "Real-time analysis ready",
        "target_latency_ms": 50
    });
    
    if sender.send(Message::Text(welcome.to_string())).await.is_err() {
        return;
    }
    
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = process_frame_message(&text, &state, &mut sender).await {
                    warn!("Frame processing error: {}", e);
                    let error_msg = serde_json::json!({
                        "type": "error",
                        "message": format!("Processing failed: {}", e)
                    });
                    let _ = sender.send(Message::Text(error_msg.to_string())).await;
                }
            }
            Ok(Message::Binary(data)) => {
                if let Err(e) = process_binary_frame(&data, &state, &mut sender).await {
                    warn!("Binary frame processing error: {}", e);
                }
            }
            Ok(Message::Close(_)) => {
                info!("🔌 WebSocket connection closed");
                break;
            }
            Ok(Message::Ping(data)) => {
                let _ = sender.send(Message::Pong(data)).await;
            }
            Ok(Message::Pong(_)) => {
                // Handle pong
            }
            Err(e) => {
                warn!("WebSocket error: {}", e);
                break;
            }
        }
    }
    
    info!("🏁 Real-time analysis session ended");
}

async fn process_frame_message(
    text: &str,
    state: &Arc<AppState>,
    sender: &mut futures_util::stream::SplitSink<axum::extract::ws::WebSocket, axum::extract::ws::Message>,
) -> Result<()> {
    use axum::extract::ws::Message;
    use futures_util::SinkExt;
    
    let frame_start = std::time::Instant::now();
    
    let request: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| anyhow::anyhow!("Invalid JSON: {}", e))?;
    
    let frame_base64 = request["frame_data"].as_str()
        .ok_or_else(|| anyhow::anyhow!("No frame_data field"))?;
    
    let frame_data = base64::prelude::Engine::decode(&base64::prelude::BASE64_STANDARD, frame_base64)
        .map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))?;
    
    let analysis_result = state.ai_analyzer.analyze_frame_realtime(&frame_data).await?;
    
    let total_latency = frame_start.elapsed().as_millis();
    
    let mut response = analysis_result;
    response["type"] = serde_json::Value::String("analysis".to_string());
    response["total_latency_ms"] = serde_json::Value::Number(serde_json::Number::from(total_latency as u64));
    response["timestamp"] = serde_json::Value::String(chrono::Utc::now().to_rfc3339());
    
    sender.send(Message::Text(response.to_string())).await
        .map_err(|e| anyhow::anyhow!("Failed to send response: {}", e))?;
    
    if total_latency > 50 {
        warn!("⚠️ High latency: {}ms (target: <50ms)", total_latency);
    } else {
        info!("⚡ Analysis completed in {}ms", total_latency);
    }
    
    Ok(())
}

async fn process_binary_frame(
    data: &[u8],
    state: &Arc<AppState>,
    sender: &mut futures_util::stream::SplitSink<axum::extract::ws::WebSocket, axum::extract::ws::Message>,
) -> Result<()> {
    use axum::extract::ws::Message;
    use futures_util::SinkExt;
    
    let frame_start = std::time::Instant::now();
    
    let analysis_result = state.ai_analyzer.analyze_frame_realtime(data).await?;
    
    let total_latency = frame_start.elapsed().as_millis();
    
    let mut response = analysis_result;
    response["type"] = serde_json::Value::String("analysis".to_string());
    response["total_latency_ms"] = serde_json::Value::Number(serde_json::Number::from(total_latency as u64));
    response["timestamp"] = serde_json::Value::String(chrono::Utc::now().to_rfc3339());
    
    sender.send(Message::Text(response.to_string())).await
        .map_err(|e| anyhow::anyhow!("Failed to send response: {}", e))?;
    
    Ok(())
}