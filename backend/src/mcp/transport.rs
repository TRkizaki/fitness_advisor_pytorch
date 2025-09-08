use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, error, debug, warn};

#[async_trait]
pub trait Transport {
    async fn initialize(&self) -> Result<()>;
    async fn receive_message(&self) -> Result<String>;
    async fn send_message(&self, message: String) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
}

// STDIO Transport for command-line integration
pub struct StdioTransport {
    tx: Arc<RwLock<Option<mpsc::Sender<String>>>>,
    rx: Arc<RwLock<Option<mpsc::Receiver<String>>>>,
    shutdown_tx: Arc<RwLock<Option<mpsc::Sender<()>>>>,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            tx: Arc::new(RwLock::new(None)),
            rx: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn initialize(&self) -> Result<()> {
        let (message_tx, message_rx) = mpsc::channel::<String>(100);
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

        // Store channels
        {
            let mut tx_guard = self.tx.write().await;
            *tx_guard = Some(message_tx);
        }
        {
            let mut rx_guard = self.rx.write().await;
            *rx_guard = Some(message_rx);
        }
        {
            let mut shutdown_guard = self.shutdown_tx.write().await;
            *shutdown_guard = Some(shutdown_tx);
        }

        let message_tx_clone = {
            let tx_guard = self.tx.read().await;
            tx_guard.as_ref().unwrap().clone()
        };

        // Spawn STDIN reader task
        tokio::spawn(async move {
            let stdin = tokio::io::stdin();
            let mut reader = BufReader::new(stdin);
            let mut buffer = String::new();

            loop {
                tokio::select! {
                    // Read from STDIN
                    result = reader.read_line(&mut buffer) => {
                        match result {
                            Ok(0) => {
                                debug!("STDIN closed");
                                break;
                            }
                            Ok(_) => {
                                let line = buffer.trim().to_string();
                                if !line.is_empty() {
                                    debug!("Received STDIN: {}", line);
                                    if let Err(e) = message_tx_clone.send(line).await {
                                        error!("Failed to send message to handler: {}", e);
                                        break;
                                    }
                                }
                                buffer.clear();
                            }
                            Err(e) => {
                                error!("Error reading from STDIN: {}", e);
                                break;
                            }
                        }
                    }

                    // Handle shutdown
                    _ = shutdown_rx.recv() => {
                        info!("STDIO transport shutting down");
                        break;
                    }
                }
            }
        });

        info!("STDIO transport initialized");
        Ok(())
    }

    async fn receive_message(&self) -> Result<String> {
        let mut rx_guard = self.rx.write().await;
        if let Some(rx) = rx_guard.as_mut() {
            match rx.recv().await {
                Some(message) => Ok(message),
                None => Err(anyhow!("Channel closed")),
            }
        } else {
            Err(anyhow!("Transport not initialized"))
        }
    }

    async fn send_message(&self, message: String) -> Result<()> {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(message.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
        debug!("Sent to STDOUT: {}", message);
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        let shutdown_tx = {
            let mut shutdown_guard = self.shutdown_tx.write().await;
            shutdown_guard.take()
        };

        if let Some(tx) = shutdown_tx {
            let _ = tx.send(()).await;
        }

        // Clear channels
        {
            let mut tx_guard = self.tx.write().await;
            *tx_guard = None;
        }
        {
            let mut rx_guard = self.rx.write().await;
            *rx_guard = None;
        }

        info!("STDIO transport shut down");
        Ok(())
    }
}

// WebSocket Transport for web-based integration
pub struct WebSocketTransport {
    port: u16,
    listener: Arc<RwLock<Option<TcpListener>>>,
    connections: Arc<RwLock<Vec<Arc<RwLock<TcpStream>>>>>,
    message_rx: Arc<RwLock<Option<mpsc::Receiver<String>>>>,
    message_tx: Arc<RwLock<Option<mpsc::Sender<String>>>>,
    shutdown_tx: Arc<RwLock<Option<mpsc::Sender<()>>>>,
}

impl WebSocketTransport {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            listener: Arc::new(RwLock::new(None)),
            connections: Arc::new(RwLock::new(Vec::new())),
            message_rx: Arc::new(RwLock::new(None)),
            message_tx: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait]
impl Transport for WebSocketTransport {
    async fn initialize(&self) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        info!("WebSocket transport listening on port {}", self.port);

        {
            let mut listener_guard = self.listener.write().await;
            *listener_guard = Some(listener);
        }

        let (message_tx, message_rx) = mpsc::channel::<String>(100);
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

        {
            let mut tx_guard = self.message_tx.write().await;
            *tx_guard = Some(message_tx.clone());
        }
        {
            let mut rx_guard = self.message_rx.write().await;
            *rx_guard = Some(message_rx);
        }
        {
            let mut shutdown_guard = self.shutdown_tx.write().await;
            *shutdown_guard = Some(shutdown_tx);
        }

        let connections = Arc::clone(&self.connections);
        let listener_clone = Arc::clone(&self.listener);

        // Spawn connection acceptor task
        tokio::spawn(async move {
            loop {
                let listener = {
                    let listener_guard = listener_clone.read().await;
                    match listener_guard.as_ref() {
                        Some(l) => l.try_clone(),
                        None => {
                            warn!("Listener not available");
                            break;
                        }
                    }
                };

                tokio::select! {
                    // Accept new connections
                    result = async {
                        if let Ok(listener) = listener {
                            listener.accept().await
                        } else {
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            Err(std::io::Error::new(std::io::ErrorKind::Other, "No listener"))
                        }
                    } => {
                        match result {
                            Ok((stream, addr)) => {
                                info!("New connection from: {}", addr);
                                let stream_arc = Arc::new(RwLock::new(stream));
                                
                                {
                                    let mut connections_guard = connections.write().await;
                                    connections_guard.push(stream_arc.clone());
                                }

                                let message_tx_clone = message_tx.clone();
                                
                                // Spawn handler for this connection
                                tokio::spawn(async move {
                                    Self::handle_connection(stream_arc, message_tx_clone).await;
                                });
                            }
                            Err(e) => {
                                error!("Error accepting connection: {}", e);
                            }
                        }
                    }

                    // Handle shutdown
                    _ = shutdown_rx.recv() => {
                        info!("WebSocket transport shutting down");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    async fn receive_message(&self) -> Result<String> {
        let mut rx_guard = self.message_rx.write().await;
        if let Some(rx) = rx_guard.as_mut() {
            match rx.recv().await {
                Some(message) => Ok(message),
                None => Err(anyhow!("Channel closed")),
            }
        } else {
            Err(anyhow!("Transport not initialized"))
        }
    }

    async fn send_message(&self, message: String) -> Result<()> {
        let connections = self.connections.read().await;
        
        if connections.is_empty() {
            warn!("No connections available to send message");
            return Ok(());
        }

        // Send to all connected clients
        for connection in connections.iter() {
            let mut stream_guard = connection.write().await;
            if let Err(e) = stream_guard.write_all(message.as_bytes()).await {
                error!("Failed to send message to connection: {}", e);
                continue;
            }
            if let Err(e) = stream_guard.write_all(b"\n").await {
                error!("Failed to send newline to connection: {}", e);
                continue;
            }
            if let Err(e) = stream_guard.flush().await {
                error!("Failed to flush connection: {}", e);
                continue;
            }
        }

        debug!("Sent message to {} connections", connections.len());
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        // Send shutdown signal
        let shutdown_tx = {
            let mut shutdown_guard = self.shutdown_tx.write().await;
            shutdown_guard.take()
        };

        if let Some(tx) = shutdown_tx {
            let _ = tx.send(()).await;
        }

        // Close all connections
        {
            let mut connections = self.connections.write().await;
            connections.clear();
        }

        // Close listener
        {
            let mut listener_guard = self.listener.write().await;
            *listener_guard = None;
        }

        info!("WebSocket transport shut down");
        Ok(())
    }
}

impl WebSocketTransport {
    async fn handle_connection(
        connection: Arc<RwLock<TcpStream>>,
        message_tx: mpsc::Sender<String>,
    ) {
        let stream = {
            let connection_guard = connection.read().await;
            match connection_guard.try_clone() {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to clone stream: {}", e);
                    return;
                }
            }
        };

        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if line.trim().is_empty() {
                continue;
            }

            debug!("Received WebSocket message: {}", line);

            if let Err(e) = message_tx.send(line).await {
                error!("Failed to forward message: {}", e);
                break;
            }
        }

        debug!("Connection handler terminated");
    }
}

// HTTP Transport for REST-like integration
pub struct HttpTransport {
    port: u16,
    listener: Arc<RwLock<Option<TcpListener>>>,
    message_queue: Arc<RwLock<Vec<String>>>,
    response_queue: Arc<RwLock<Vec<String>>>,
    shutdown_tx: Arc<RwLock<Option<mpsc::Sender<()>>>>,
}

impl HttpTransport {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            listener: Arc::new(RwLock::new(None)),
            message_queue: Arc::new(RwLock::new(Vec::new())),
            response_queue: Arc::new(RwLock::new(Vec::new())),
            shutdown_tx: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn initialize(&self) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        info!("HTTP transport listening on port {}", self.port);

        {
            let mut listener_guard = self.listener.write().await;
            *listener_guard = Some(listener);
        }

        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        {
            let mut shutdown_guard = self.shutdown_tx.write().await;
            *shutdown_guard = Some(shutdown_tx);
        }

        let listener_clone = Arc::clone(&self.listener);
        let message_queue = Arc::clone(&self.message_queue);
        let response_queue = Arc::clone(&self.response_queue);

        // Spawn HTTP server task
        tokio::spawn(async move {
            loop {
                let listener = {
                    let listener_guard = listener_clone.read().await;
                    match listener_guard.as_ref() {
                        Some(l) => l.try_clone(),
                        None => break,
                    }
                };

                tokio::select! {
                    result = async {
                        if let Ok(listener) = listener {
                            listener.accept().await
                        } else {
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            Err(std::io::Error::new(std::io::ErrorKind::Other, "No listener"))
                        }
                    } => {
                        match result {
                            Ok((stream, addr)) => {
                                info!("HTTP request from: {}", addr);
                                let message_queue_clone = Arc::clone(&message_queue);
                                let response_queue_clone = Arc::clone(&response_queue);
                                
                                tokio::spawn(async move {
                                    Self::handle_http_request(stream, message_queue_clone, response_queue_clone).await;
                                });
                            }
                            Err(e) => {
                                error!("Error accepting HTTP connection: {}", e);
                            }
                        }
                    }

                    _ = shutdown_rx.recv() => {
                        info!("HTTP transport shutting down");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    async fn receive_message(&self) -> Result<String> {
        // Poll message queue
        loop {
            {
                let mut queue = self.message_queue.write().await;
                if !queue.is_empty() {
                    return Ok(queue.remove(0));
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }

    async fn send_message(&self, message: String) -> Result<()> {
        let mut queue = self.response_queue.write().await;
        queue.push(message);
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        let shutdown_tx = {
            let mut shutdown_guard = self.shutdown_tx.write().await;
            shutdown_guard.take()
        };

        if let Some(tx) = shutdown_tx {
            let _ = tx.send(()).await;
        }

        {
            let mut listener_guard = self.listener.write().await;
            *listener_guard = None;
        }

        info!("HTTP transport shut down");
        Ok(())
    }
}

impl HttpTransport {
    async fn handle_http_request(
        mut stream: TcpStream,
        message_queue: Arc<RwLock<Vec<String>>>,
        response_queue: Arc<RwLock<Vec<String>>>,
    ) {
        let mut buffer = vec![0; 1024];
        
        match stream.read(&mut buffer).await {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]);
                debug!("HTTP request: {}", request);

                // Simple HTTP request parsing
                if request.starts_with("POST") {
                    if let Some(body_start) = request.find("\r\n\r\n") {
                        let body = request[body_start + 4..].trim_end_matches('\0');
                        if !body.is_empty() {
                            let mut queue = message_queue.write().await;
                            queue.push(body.to_string());
                        }
                    }

                    // Wait for response
                    for _ in 0..100 { // Wait up to 1 second
                        {
                            let mut queue = response_queue.write().await;
                            if !queue.is_empty() {
                                let response = queue.remove(0);
                                let http_response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                                    response.len(),
                                    response
                                );
                                let _ = stream.write_all(http_response.as_bytes()).await;
                                return;
                            }
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                }

                // Send 404 for other requests
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                let _ = stream.write_all(response.as_bytes()).await;
            }
            Err(e) => {
                error!("Failed to read HTTP request: {}", e);
            }
        }
    }
}

// Factory function for creating transports
pub fn create_transport(transport_type: &str, config: Option<Value>) -> Result<Box<dyn Transport + Send + Sync>> {
    match transport_type {
        "stdio" => Ok(Box::new(StdioTransport::new())),
        "websocket" => {
            let port = config
                .and_then(|c| c.get("port"))
                .and_then(|p| p.as_u64())
                .unwrap_or(8080) as u16;
            Ok(Box::new(WebSocketTransport::new(port)))
        }
        "http" => {
            let port = config
                .and_then(|c| c.get("port"))
                .and_then(|p| p.as_u64())
                .unwrap_or(8081) as u16;
            Ok(Box::new(HttpTransport::new(port)))
        }
        _ => Err(anyhow!("Unknown transport type: {}", transport_type)),
    }
}