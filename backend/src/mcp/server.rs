use anyhow::{Result, anyhow};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, error, warn, debug};

use crate::mcp::{
    protocol::MCPProtocol,
    types::*,
    transport::Transport,
    auth::AuthManager,
};

pub struct MCPServer {
    protocol: Arc<RwLock<MCPProtocol>>,
    transport: Box<dyn Transport + Send + Sync>,
    auth_manager: Arc<AuthManager>,
    shutdown_tx: Option<mpsc::Sender<()>>,
    is_running: Arc<RwLock<bool>>,
}

impl MCPServer {
    pub fn new(
        transport: Box<dyn Transport + Send + Sync>,
        auth_manager: AuthManager,
    ) -> Self {
        Self {
            protocol: Arc::new(RwLock::new(MCPProtocol::new())),
            transport,
            auth_manager: Arc::new(auth_manager),
            shutdown_tx: None,
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Fitness Advisor MCP Server");
        
        // Check if already running
        {
            let mut running = self.is_running.write().await;
            if *running {
                return Err(anyhow!("Server is already running"));
            }
            *running = true;
        }

        // Set up shutdown channel
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        self.shutdown_tx = Some(shutdown_tx);

        // Initialize transport
        self.transport.initialize().await?;
        info!("Transport initialized successfully");

        // Clone necessary components for the message handling loop
        let protocol = Arc::clone(&self.protocol);
        let auth_manager = Arc::clone(&self.auth_manager);
        let transport = &self.transport;
        let is_running = Arc::clone(&self.is_running);

        // Main message handling loop
        loop {
            tokio::select! {
                // Handle incoming messages
                message_result = transport.receive_message() => {
                    match message_result {
                        Ok(raw_message) => {
                            let protocol_clone = Arc::clone(&protocol);
                            let auth_clone = Arc::clone(&auth_manager);
                            
                            // Process message in background task
                            tokio::spawn(async move {
                                if let Err(e) = Self::handle_raw_message(
                                    raw_message, 
                                    protocol_clone, 
                                    auth_clone
                                ).await {
                                    error!("Error handling message: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            error!("Error receiving message: {}", e);
                            // Don't break on receive errors, just log and continue
                            continue;
                        }
                    }
                }
                
                // Handle shutdown signal
                _ = shutdown_rx.recv() => {
                    info!("Received shutdown signal");
                    break;
                }
            }
        }

        // Cleanup
        self.transport.shutdown().await?;
        {
            let mut running = self.is_running.write().await;
            *running = false;
        }

        info!("MCP Server shut down successfully");
        Ok(())
    }

    async fn handle_raw_message(
        raw_message: String,
        protocol: Arc<RwLock<MCPProtocol>>,
        auth_manager: Arc<AuthManager>,
    ) -> Result<()> {
        debug!("Received raw message: {}", raw_message);

        // Parse JSON-RPC message
        let message: JsonRpcMessage = match serde_json::from_str(&raw_message) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Failed to parse JSON-RPC message: {}", e);
                let error_response = MCPProtocol::create_error_response(
                    None,
                    error_codes::PARSE_ERROR,
                    format!("Parse error: {}", e),
                );
                // TODO: Send error response back through transport
                return Err(anyhow!("Parse error: {}", e));
            }
        };

        // Authenticate if required
        if Self::requires_authentication(&message) {
            if let Err(e) = auth_manager.authenticate(&message).await {
                warn!("Authentication failed: {}", e);
                let error_response = MCPProtocol::create_error_response(
                    message.id.clone(),
                    error_codes::AUTHENTICATION_ERROR,
                    "Authentication failed".to_string(),
                );
                // TODO: Send error response back through transport
                return Err(e);
            }
        }

        // Handle the message through protocol
        let mut protocol_guard = protocol.write().await;
        match protocol_guard.handle_message(message) {
            Ok(Some(response)) => {
                let response_json = serde_json::to_string(&response)?;
                debug!("Sending response: {}", response_json);
                // TODO: Send response back through transport
            }
            Ok(None) => {
                // No response needed (notification handled)
                debug!("Message handled successfully (no response)");
            }
            Err(e) => {
                error!("Error handling message: {}", e);
                let error_response = MCPProtocol::create_error_response(
                    None, // We've lost the original ID at this point
                    error_codes::INTERNAL_ERROR,
                    format!("Internal error: {}", e),
                );
                let error_json = serde_json::to_string(&error_response)?;
                debug!("Sending error response: {}", error_json);
                // TODO: Send error response back through transport
            }
        }

        Ok(())
    }

    fn requires_authentication(message: &JsonRpcMessage) -> bool {
        match &message.content {
            MessageContent::Request(request) => {
                // Most tool calls and resource access require authentication
                matches!(request.method.as_str(), 
                    "tools/call" | "resources/read" | "prompts/get"
                )
            }
            _ => false,
        }
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Initiating MCP Server shutdown");

        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            if let Err(e) = shutdown_tx.send(()).await {
                warn!("Failed to send shutdown signal: {}", e);
            }
        }

        // Wait for shutdown to complete
        let mut attempts = 0;
        while attempts < 50 { // 5 seconds max wait
            {
                let running = self.is_running.read().await;
                if !*running {
                    break;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            attempts += 1;
        }

        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    // Health check endpoint
    pub async fn health_check(&self) -> Result<serde_json::Value> {
        let is_running = self.is_running().await;
        let protocol = self.protocol.read().await;
        
        Ok(serde_json::json!({
            "status": if is_running { "healthy" } else { "stopped" },
            "server_info": protocol.server_info,
            "capabilities": protocol.capabilities,
            "active_sessions": protocol.sessions.len(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }))
    }

    // Get server statistics
    pub async fn get_stats(&self) -> Result<serde_json::Value> {
        let protocol = self.protocol.read().await;
        
        let active_sessions = protocol.sessions.len();
        let session_info: Vec<_> = protocol.sessions.values()
            .map(|session| serde_json::json!({
                "id": session.id,
                "client": session.client_info.name,
                "created_at": session.created_at,
                "last_activity": session.last_activity,
                "is_active": session.is_active,
            }))
            .collect();

        Ok(serde_json::json!({
            "server_info": protocol.server_info,
            "capabilities": protocol.capabilities,
            "sessions": {
                "active_count": active_sessions,
                "sessions": session_info,
            },
            "uptime": chrono::Utc::now().to_rfc3339(),
        }))
    }

    // Send a log message to all connected clients
    pub async fn broadcast_log(&self, level: LogLevel, message: String, data: Option<Value>) -> Result<()> {
        let protocol = self.protocol.read().await;
        let log_message = protocol.send_log(level, message, data);
        let log_json = serde_json::to_string(&log_message)?;
        
        // TODO: Broadcast to all connected clients through transport
        debug!("Broadcasting log: {}", log_json);
        
        Ok(())
    }

    // Integration points for existing fitness advisor systems
    pub async fn integrate_rag_system(&mut self, rag_endpoint: String) -> Result<()> {
        info!("Integrating with RAG system at: {}", rag_endpoint);
        // TODO: Set up RAG system integration
        // This would connect to the existing RAG API endpoints
        Ok(())
    }

    pub async fn integrate_workout_planner(&mut self, planner_config: serde_json::Value) -> Result<()> {
        info!("Integrating workout planner with config: {}", planner_config);
        // TODO: Set up workout planner integration
        // This would connect to the existing menu optimization and workout systems
        Ok(())
    }

    pub async fn integrate_nutrition_analyzer(&mut self, analyzer_config: serde_json::Value) -> Result<()> {
        info!("Integrating nutrition analyzer with config: {}", analyzer_config);
        // TODO: Set up nutrition analyzer integration
        // This would connect to the existing nutrition analysis systems
        Ok(())
    }
}

// Builder pattern for easier server construction
pub struct MCPServerBuilder {
    transport: Option<Box<dyn Transport + Send + Sync>>,
    auth_manager: Option<AuthManager>,
}

impl MCPServerBuilder {
    pub fn new() -> Self {
        Self {
            transport: None,
            auth_manager: None,
        }
    }

    pub fn with_transport(mut self, transport: Box<dyn Transport + Send + Sync>) -> Self {
        self.transport = Some(transport);
        self
    }

    pub fn with_auth(mut self, auth_manager: AuthManager) -> Self {
        self.auth_manager = Some(auth_manager);
        self
    }

    pub fn build(self) -> Result<MCPServer> {
        let transport = self.transport.ok_or_else(|| anyhow!("Transport is required"))?;
        let auth_manager = self.auth_manager.unwrap_or_else(AuthManager::new);

        Ok(MCPServer::new(transport, auth_manager))
    }
}

impl Default for MCPServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}