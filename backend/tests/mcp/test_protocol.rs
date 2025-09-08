#[cfg(test)]
mod protocol_tests {
    use fitness_advisor_ai::mcp::{
        MCPProtocol, JsonRpcMessage, MessageContent, JsonRpcRequest, InitializeParams, 
        ClientCapabilities, ClientInfo, CallToolRequest, MCP_VERSION
    };
    use serde_json::{json, Value};

    fn create_test_protocol() -> MCPProtocol {
        MCPProtocol::new()
    }

    fn create_initialize_message(id: Value) -> JsonRpcMessage {
        JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            content: MessageContent::Request(JsonRpcRequest {
                method: "initialize".to_string(),
                params: Some(json!({
                    "protocolVersion": MCP_VERSION,
                    "capabilities": {},
                    "clientInfo": {
                        "name": "Test Client",
                        "version": "1.0.0"
                    }
                })),
            }),
        }
    }

    #[tokio::test]
    async fn test_initialize_request() {
        let mut protocol = create_test_protocol();
        let message = create_initialize_message(json!(1));

        let result = protocol.handle_message(message).unwrap();
        assert!(result.is_some());

        let response = result.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, Some(json!(1)));

        if let MessageContent::Response(resp) = response.content {
            let result_data = resp.result;
            assert!(result_data.get("protocolVersion").is_some());
            assert!(result_data.get("capabilities").is_some());
            assert!(result_data.get("serverInfo").is_some());
        } else {
            panic!("Expected response message");
        }
    }

    #[tokio::test]
    async fn test_invalid_protocol_version() {
        let mut protocol = create_test_protocol();
        let mut message = create_initialize_message(json!(1));

        // Set invalid protocol version
        if let MessageContent::Request(ref mut req) = message.content {
            if let Some(ref mut params) = req.params {
                params["protocolVersion"] = json!("invalid-version");
            }
        }

        let result = protocol.handle_message(message);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tools_list() {
        let protocol = create_test_protocol();
        
        let message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/list".to_string(),
                params: None,
            }),
        };

        let mut protocol_mut = protocol;
        let result = protocol_mut.handle_message(message).unwrap();
        assert!(result.is_some());

        let response = result.unwrap();
        if let MessageContent::Response(resp) = response.content {
            let tools = resp.result.get("tools").unwrap().as_array().unwrap();
            assert!(!tools.is_empty());
            
            // Check that fitness-specific tools are present
            let tool_names: Vec<&str> = tools
                .iter()
                .map(|tool| tool.get("name").unwrap().as_str().unwrap())
                .collect();
            
            assert!(tool_names.contains(&"create_workout_plan"));
            assert!(tool_names.contains(&"create_nutrition_plan"));
            assert!(tool_names.contains(&"analyze_nutrition"));
            assert!(tool_names.contains(&"rag_fitness_query"));
        } else {
            panic!("Expected response message");
        }
    }

    #[tokio::test]
    async fn test_tool_call_create_workout_plan() {
        let protocol = create_test_protocol();
        
        let message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(3)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "create_workout_plan",
                    "arguments": {
                        "user_profile": {
                            "age": 25,
                            "weight_kg": 70.0,
                            "height_cm": 175,
                            "fitness_goals": ["muscle_gain"],
                            "activity_level": "moderately_active"
                        }
                    }
                })),
            }),
        };

        let mut protocol_mut = protocol;
        let result = protocol_mut.handle_message(message).unwrap();
        assert!(result.is_some());

        let response = result.unwrap();
        if let MessageContent::Response(resp) = response.content {
            let tool_result = resp.result;
            assert!(tool_result.get("content").is_some());
            
            let content = tool_result.get("content").unwrap().as_array().unwrap();
            assert!(!content.is_empty());
            
            let first_content = &content[0];
            assert_eq!(first_content.get("type").unwrap().as_str().unwrap(), "text");
            assert!(first_content.get("text").unwrap().as_str().unwrap().contains("workout"));
        } else {
            panic!("Expected response message");
        }
    }

    #[tokio::test]
    async fn test_tool_call_unknown_tool() {
        let protocol = create_test_protocol();
        
        let message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(4)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "unknown_tool",
                    "arguments": {}
                })),
            }),
        };

        let mut protocol_mut = protocol;
        let result = protocol_mut.handle_message(message).unwrap();
        assert!(result.is_some());

        let response = result.unwrap();
        if let MessageContent::Response(resp) = response.content {
            let tool_result = resp.result;
            assert!(tool_result.get("isError").unwrap_or(&json!(false)).as_bool().unwrap_or(false));
        } else {
            panic!("Expected response message");
        }
    }

    #[tokio::test]
    async fn test_resources_list() {
        let protocol = create_test_protocol();
        
        let message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(5)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "resources/list".to_string(),
                params: None,
            }),
        };

        let mut protocol_mut = protocol;
        let result = protocol_mut.handle_message(message).unwrap();
        assert!(result.is_some());

        let response = result.unwrap();
        if let MessageContent::Response(resp) = response.content {
            let resources = resp.result.get("resources").unwrap().as_array().unwrap();
            assert!(!resources.is_empty());
            
            // Check for fitness-specific resources
            let resource_names: Vec<&str> = resources
                .iter()
                .map(|resource| resource.get("name").unwrap().as_str().unwrap())
                .collect();
            
            assert!(resource_names.iter().any(|name| name.contains("Exercise")));
            assert!(resource_names.iter().any(|name| name.contains("Nutrition")));
        } else {
            panic!("Expected response message");
        }
    }

    #[tokio::test]
    async fn test_prompts_list() {
        let protocol = create_test_protocol();
        
        let message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(6)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "prompts/list".to_string(),
                params: None,
            }),
        };

        let mut protocol_mut = protocol;
        let result = protocol_mut.handle_message(message).unwrap();
        assert!(result.is_some());

        let response = result.unwrap();
        if let MessageContent::Response(resp) = response.content {
            let prompts = resp.result.get("prompts").unwrap().as_array().unwrap();
            assert!(!prompts.is_empty());
            
            // Check for fitness-specific prompts
            let prompt_names: Vec<&str> = prompts
                .iter()
                .map(|prompt| prompt.get("name").unwrap().as_str().unwrap())
                .collect();
            
            assert!(prompt_names.contains(&"create_workout_prompt"));
            assert!(prompt_names.contains(&"nutrition_analysis_prompt"));
        } else {
            panic!("Expected response message");
        }
    }

    #[tokio::test]
    async fn test_method_not_found() {
        let protocol = create_test_protocol();
        
        let message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(7)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "unknown/method".to_string(),
                params: None,
            }),
        };

        let mut protocol_mut = protocol;
        let result = protocol_mut.handle_message(message);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_notification_handling() {
        let protocol = create_test_protocol();
        
        let message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: None, // Notifications don't have IDs
            content: MessageContent::Notification(crate::mcp::types::JsonRpcNotification {
                method: "notifications/cancelled".to_string(),
                params: Some(json!({"reason": "user_cancelled"})),
            }),
        };

        let mut protocol_mut = protocol;
        let result = protocol_mut.handle_message(message).unwrap();
        
        // Notifications should not return responses
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_session_management() {
        let mut protocol = create_test_protocol();
        
        // Initialize first session
        let message1 = create_initialize_message(json!(1));
        let result1 = protocol.handle_message(message1).unwrap();
        assert!(result1.is_some());
        
        // Initialize second session
        let message2 = create_initialize_message(json!(2));
        let result2 = protocol.handle_message(message2).unwrap();
        assert!(result2.is_some());
        
        // Check that we have two active sessions
        assert_eq!(protocol.sessions.len(), 2);
    }

    #[tokio::test]
    async fn test_server_capabilities() {
        let protocol = create_test_protocol();
        
        assert!(protocol.capabilities.logging.is_some());
        assert!(protocol.capabilities.prompts.is_some());
        assert!(protocol.capabilities.resources.is_some());
        assert!(protocol.capabilities.tools.is_some());
    }

    #[tokio::test]
    async fn test_server_info() {
        let protocol = create_test_protocol();
        
        assert_eq!(protocol.server_info.name, "Fitness Advisor AI MCP Server");
        assert_eq!(protocol.server_info.version, "1.0.0");
        assert!(protocol.server_info.description.contains("AI-powered fitness"));
        assert_eq!(protocol.server_info.license, "MIT");
    }
}