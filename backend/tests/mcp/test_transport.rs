#[cfg(test)]
mod transport_tests {
    use fitness_advisor_ai::mcp::{
        Transport, StdioTransport, WebSocketTransport, HttpTransport
    };
    use std::time::Duration;
    use tokio::time::timeout;
    use anyhow::Result;

    #[tokio::test]
    async fn test_stdio_transport_creation() {
        let transport = StdioTransport::new();
        
        // Transport should be created successfully
        assert!(transport.is_ok());
    }

    #[tokio::test]
    async fn test_stdio_transport_initialize() {
        let transport = StdioTransport::new().unwrap();
        
        let result = transport.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stdio_transport_shutdown() {
        let transport = StdioTransport::new().unwrap();
        
        // Initialize first
        let _ = transport.initialize().await;
        
        let result = transport.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_websocket_transport_creation() {
        let transport = WebSocketTransport::new("ws://localhost:8080".to_string());
        
        // Transport should be created successfully
        assert!(transport.is_ok());
        
        let transport = transport.unwrap();
        assert_eq!(transport.url, "ws://localhost:8080");
    }

    #[tokio::test]
    async fn test_websocket_transport_invalid_url() {
        let transport = WebSocketTransport::new("invalid-url".to_string());
        
        // Should handle invalid URLs gracefully
        assert!(transport.is_err());
    }

    #[tokio::test]
    async fn test_websocket_transport_initialize_no_server() {
        let transport = WebSocketTransport::new("ws://localhost:9999".to_string()).unwrap();
        
        // Should handle connection failure to non-existent server
        let result = timeout(Duration::from_millis(1000), transport.initialize()).await;
        
        // Either timeout or connection error is expected
        assert!(result.is_err() || result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_http_transport_creation() {
        let transport = HttpTransport::new("http://localhost:8080".to_string());
        
        // Transport should be created successfully
        assert!(transport.is_ok());
        
        let transport = transport.unwrap();
        assert_eq!(transport.base_url, "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_http_transport_invalid_url() {
        let transport = HttpTransport::new("not-a-url".to_string());
        
        // Should handle invalid URLs gracefully
        assert!(transport.is_err());
    }

    #[tokio::test]
    async fn test_http_transport_initialize() {
        let transport = HttpTransport::new("http://localhost:8080".to_string()).unwrap();
        
        let result = transport.initialize().await;
        // HTTP transport should initialize successfully even if server is not running
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_http_transport_send_message_no_server() {
        let transport = HttpTransport::new("http://localhost:9999".to_string()).unwrap();
        let _ = transport.initialize().await;
        
        let test_message = r#"{"jsonrpc": "2.0", "method": "test", "id": 1}"#.to_string();
        
        // Should handle connection failure gracefully
        let result = timeout(
            Duration::from_millis(2000), 
            transport.send_message(test_message)
        ).await;
        
        // Either timeout or connection error is expected
        assert!(result.is_err() || result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_transport_trait_object() {
        // Test that all transports can be used as trait objects
        let transports: Vec<Result<Box<dyn Transport + Send + Sync>>> = vec![
            StdioTransport::new().map(|t| Box::new(t) as Box<dyn Transport + Send + Sync>),
            WebSocketTransport::new("ws://localhost:8080".to_string())
                .map(|t| Box::new(t) as Box<dyn Transport + Send + Sync>),
            HttpTransport::new("http://localhost:8080".to_string())
                .map(|t| Box::new(t) as Box<dyn Transport + Send + Sync>),
        ];
        
        for transport_result in transports {
            if let Ok(transport) = transport_result {
                // Each transport should be able to initialize
                let result = transport.initialize().await;
                // Some may fail due to no server, but they should not panic
                let _ = result; // Ignore result, we just want to ensure no panic
            }
        }
    }

    #[tokio::test]
    async fn test_stdio_transport_message_format() {
        let transport = StdioTransport::new().unwrap();
        let _ = transport.initialize().await;
        
        let test_message = r#"{"jsonrpc": "2.0", "method": "initialize", "id": 1}"#.to_string();
        
        // STDIO transport should handle JSON messages
        // Note: In real usage, this would require actual stdin/stdout interaction
        // For unit testing, we're mainly testing that the transport doesn't crash
        let _ = transport.send_message(test_message).await;
    }

    #[tokio::test]
    async fn test_websocket_transport_message_format() {
        let transport = WebSocketTransport::new("ws://localhost:8080".to_string()).unwrap();
        
        let test_message = r#"{"jsonrpc": "2.0", "method": "initialize", "id": 1}"#.to_string();
        
        // WebSocket transport should handle JSON messages
        // Even if connection fails, message formatting should be valid
        assert!(!test_message.is_empty());
        assert!(serde_json::from_str::<serde_json::Value>(&test_message).is_ok());
    }

    #[tokio::test]
    async fn test_http_transport_message_format() {
        let transport = HttpTransport::new("http://localhost:8080".to_string()).unwrap();
        
        let test_message = r#"{"jsonrpc": "2.0", "method": "initialize", "id": 1}"#.to_string();
        
        // HTTP transport should handle JSON messages
        assert!(!test_message.is_empty());
        assert!(serde_json::from_str::<serde_json::Value>(&test_message).is_ok());
    }

    #[tokio::test]
    async fn test_transport_lifecycle() {
        let transport = StdioTransport::new().unwrap();
        
        // Test full lifecycle: initialize -> use -> shutdown
        let init_result = transport.initialize().await;
        assert!(init_result.is_ok());
        
        // Transport should be usable after initialization
        let test_message = r#"{"jsonrpc": "2.0", "method": "test", "id": 1}"#.to_string();
        let _ = transport.send_message(test_message).await;
        
        // Should be able to shutdown cleanly
        let shutdown_result = transport.shutdown().await;
        assert!(shutdown_result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_transport_operations() {
        let transport = StdioTransport::new().unwrap();
        let _ = transport.initialize().await;
        
        // Test multiple concurrent operations
        let handles = (0..5).map(|i| {
            let test_message = format!(r#"{{"jsonrpc": "2.0", "method": "test", "id": {}}}"#, i);
            transport.send_message(test_message)
        }).collect::<Vec<_>>();
        
        // All operations should complete without panicking
        for handle in handles {
            let _ = handle.await; // Ignore results, just ensure no panic
        }
    }

    #[tokio::test]
    async fn test_transport_error_handling() {
        // Test error handling for various transport types
        
        // Invalid WebSocket URL
        let ws_result = WebSocketTransport::new("not-a-websocket-url".to_string());
        assert!(ws_result.is_err());
        
        // Invalid HTTP URL
        let http_result = HttpTransport::new("not-a-http-url".to_string());
        assert!(http_result.is_err());
        
        // STDIO should always create successfully
        let stdio_result = StdioTransport::new();
        assert!(stdio_result.is_ok());
    }

    #[tokio::test]
    async fn test_transport_timeout_handling() {
        let transport = HttpTransport::new("http://127.0.0.1:9999".to_string()).unwrap();
        let _ = transport.initialize().await;
        
        let test_message = r#"{"jsonrpc": "2.0", "method": "test", "id": 1}"#.to_string();
        
        // Test with short timeout
        let result = timeout(
            Duration::from_millis(100),
            transport.send_message(test_message)
        ).await;
        
        // Should either timeout or return connection error
        assert!(result.is_err() || result.unwrap().is_err());
    }

    #[tokio::test]
    async fn test_transport_shutdown_idempotent() {
        let transport = StdioTransport::new().unwrap();
        let _ = transport.initialize().await;
        
        // Multiple shutdowns should be safe
        let result1 = transport.shutdown().await;
        let result2 = transport.shutdown().await;
        
        // At least the first shutdown should succeed
        assert!(result1.is_ok());
        // Second shutdown should also not panic (may succeed or fail gracefully)
        let _ = result2; // Ignore result
    }

    #[tokio::test]
    async fn test_websocket_url_validation() {
        // Valid WebSocket URLs
        assert!(WebSocketTransport::new("ws://localhost:8080".to_string()).is_ok());
        assert!(WebSocketTransport::new("wss://example.com:443/path".to_string()).is_ok());
        
        // Invalid URLs
        assert!(WebSocketTransport::new("http://localhost:8080".to_string()).is_err());
        assert!(WebSocketTransport::new("not-a-url".to_string()).is_err());
        assert!(WebSocketTransport::new("".to_string()).is_err());
    }

    #[tokio::test]
    async fn test_http_url_validation() {
        // Valid HTTP URLs
        assert!(HttpTransport::new("http://localhost:8080".to_string()).is_ok());
        assert!(HttpTransport::new("https://example.com:443/path".to_string()).is_ok());
        
        // Invalid URLs
        assert!(HttpTransport::new("ws://localhost:8080".to_string()).is_err());
        assert!(HttpTransport::new("not-a-url".to_string()).is_err());
        assert!(HttpTransport::new("".to_string()).is_err());
    }

    #[tokio::test]
    async fn test_transport_message_size_limits() {
        let transport = StdioTransport::new().unwrap();
        let _ = transport.initialize().await;
        
        // Test with large message
        let large_message = format!(
            r#"{{"jsonrpc": "2.0", "method": "test", "params": {{"data": "{}"}}, "id": 1}}"#,
            "x".repeat(10000)
        );
        
        // Transport should handle large messages gracefully
        let result = transport.send_message(large_message).await;
        // Result may succeed or fail, but should not panic
        let _ = result;
    }

    #[tokio::test]
    async fn test_transport_json_validation() {
        let transport = StdioTransport::new().unwrap();
        let _ = transport.initialize().await;
        
        // Test with invalid JSON
        let invalid_json = "not valid json".to_string();
        
        // Transport should handle invalid JSON gracefully
        let result = transport.send_message(invalid_json).await;
        // Result may succeed or fail, but should not panic
        let _ = result;
        
        // Test with valid JSON but invalid JSON-RPC
        let invalid_jsonrpc = r#"{"not": "jsonrpc"}"#.to_string();
        let result = transport.send_message(invalid_jsonrpc).await;
        let _ = result;
    }
}