#[cfg(test)]
mod mcp_integration_tests {
    use fitness_advisor_ai::mcp::{
        MCPServer, MCPProtocol, StdioTransport, AuthManager, Transport,
        JsonRpcMessage, MessageContent, JsonRpcRequest, MCP_VERSION
    };
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;

    async fn create_test_server() -> MCPServer {
        let protocol = Arc::new(RwLock::new(MCPProtocol::new()));
        let transport = Box::new(StdioTransport::new().unwrap());
        let auth_manager = Arc::new(AuthManager::new("test-secret".to_string(), false)); // Auth disabled for testing
        
        MCPServer::new(protocol, transport, auth_manager).await.unwrap()
    }

    fn create_initialize_message() -> JsonRpcMessage {
        JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
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
    async fn test_full_server_lifecycle() {
        let mut server = create_test_server().await;
        
        // Server should be created successfully
        assert!(!server.is_running());
        
        // Note: Can't actually start server in test due to STDIO transport
        // But we can test the server structure and protocol handling
        
        // Test protocol access
        let protocol = server.protocol.clone();
        let protocol_guard = protocol.read().await;
        assert!(protocol_guard.capabilities.tools.is_some());
        assert!(protocol_guard.capabilities.resources.is_some());
        assert!(protocol_guard.capabilities.prompts.is_some());
    }

    #[tokio::test]
    async fn test_initialize_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        let message = create_initialize_message();
        
        let mut protocol_guard = protocol.write().await;
        let response = protocol_guard.handle_message(message).unwrap();
        
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, Some(json!(1)));
        
        if let MessageContent::Response(resp) = response.content {
            assert!(resp.result.get("protocolVersion").is_some());
            assert!(resp.result.get("capabilities").is_some());
            assert!(resp.result.get("serverInfo").is_some());
        } else {
            panic!("Expected response message");
        }
    }

    #[tokio::test]
    async fn test_tools_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // First initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // Then list tools
        let tools_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/list".to_string(),
                params: None,
            }),
        };
        
        let response = protocol_guard.handle_message(tools_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let tools = resp.result.get("tools").unwrap().as_array().unwrap();
            assert!(!tools.is_empty());
            
            // Verify fitness-specific tools are present
            let tool_names: Vec<&str> = tools
                .iter()
                .map(|tool| tool.get("name").unwrap().as_str().unwrap())
                .collect();
            
            assert!(tool_names.contains(&"create_workout_plan"));
            assert!(tool_names.contains(&"create_nutrition_plan"));
            assert!(tool_names.contains(&"analyze_nutrition"));
            assert!(tool_names.contains(&"optimize_for_season"));
            assert!(tool_names.contains(&"track_progress"));
        }
    }

    #[tokio::test]
    async fn test_workout_plan_creation_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // Call create_workout_plan tool
        let workout_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(3)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "create_workout_plan",
                    "arguments": {
                        "user_profile": {
                            "id": Uuid::new_v4(),
                            "age": 25,
                            "weight_kg": 70.0,
                            "height_cm": 175,
                            "gender": "male",
                            "activity_level": "moderately_active",
                            "fitness_goals": ["muscle_gain"],
                            "dietary_restrictions": [],
                            "health_conditions": []
                        },
                        "workout_preferences": {
                            "duration_minutes": 45,
                            "difficulty_level": "intermediate",
                            "equipment_available": ["dumbbells", "barbell"],
                            "workout_type": "strength"
                        }
                    }
                })),
            }),
        };
        
        let response = protocol_guard.handle_message(workout_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let tool_result = resp.result;
            let content = tool_result.get("content").unwrap().as_array().unwrap();
            assert!(!content.is_empty());
            
            let first_content = &content[0];
            assert_eq!(first_content.get("type").unwrap().as_str().unwrap(), "text");
            let text = first_content.get("text").unwrap().as_str().unwrap();
            assert!(text.contains("Workout Plan"));
            assert!(text.contains("45 minutes"));
            assert!(text.contains("Intermediate"));
        }
    }

    #[tokio::test]
    async fn test_nutrition_plan_creation_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // Call create_nutrition_plan tool
        let nutrition_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(4)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "create_nutrition_plan",
                    "arguments": {
                        "user_profile": {
                            "id": Uuid::new_v4(),
                            "age": 30,
                            "weight_kg": 75.0,
                            "height_cm": 180,
                            "gender": "male",
                            "activity_level": "moderately_active",
                            "fitness_goals": ["muscle_gain"],
                            "dietary_restrictions": ["vegetarian"],
                            "health_conditions": []
                        },
                        "calorie_target": 2500,
                        "meal_preferences": {
                            "meals_per_day": 4,
                            "macro_split": {
                                "protein_percent": 30.0,
                                "carbohydrate_percent": 40.0,
                                "fat_percent": 30.0
                            }
                        }
                    }
                })),
            }),
        };
        
        let response = protocol_guard.handle_message(nutrition_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let tool_result = resp.result;
            let content = tool_result.get("content").unwrap().as_array().unwrap();
            
            let first_content = &content[0];
            let text = first_content.get("text").unwrap().as_str().unwrap();
            assert!(text.contains("Muscle Gain Nutrition Plan"));
            assert!(text.contains("2500"));
            assert!(text.contains("vegetarian"));
            assert!(text.contains("30.0%")); // Protein percentage
        }
    }

    #[tokio::test]
    async fn test_nutrition_analysis_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // Call analyze_nutrition tool
        let analysis_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(5)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "analyze_nutrition",
                    "arguments": {
                        "foods": [
                            {
                                "name": "chicken breast",
                                "quantity": 4.0,
                                "unit": "oz"
                            },
                            {
                                "name": "brown rice",
                                "quantity": 1.0,
                                "unit": "cup"
                            }
                        ],
                        "analysis_type": "basic"
                    }
                })),
            }),
        };
        
        let response = protocol_guard.handle_message(analysis_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let tool_result = resp.result;
            let content = tool_result.get("content").unwrap().as_array().unwrap();
            
            let first_content = &content[0];
            let text = first_content.get("text").unwrap().as_str().unwrap();
            assert!(text.contains("Nutrition Analysis"));
            assert!(text.contains("chicken breast"));
            assert!(text.contains("brown rice"));
            assert!(text.contains("Total Calories"));
            assert!(text.contains("Protein"));
        }
    }

    #[tokio::test]
    async fn test_seasonal_optimization_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // Call optimize_for_season tool
        let season_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(6)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "optimize_for_season",
                    "arguments": {
                        "location": "New York",
                        "season": "winter",
                        "indoor_preference": true,
                        "user_profile": {
                            "id": Uuid::new_v4(),
                            "age": 28,
                            "weight_kg": 75.0,
                            "height_cm": 180,
                            "gender": "male",
                            "activity_level": "very_active",
                            "fitness_goals": ["strength_gain"],
                            "dietary_restrictions": [],
                            "health_conditions": []
                        }
                    }
                })),
            }),
        };
        
        let response = protocol_guard.handle_message(season_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let tool_result = resp.result;
            let content = tool_result.get("content").unwrap().as_array().unwrap();
            
            let first_content = &content[0];
            let text = first_content.get("text").unwrap().as_str().unwrap();
            assert!(text.contains("Winter"));
            assert!(text.contains("New York"));
            assert!(text.contains("indoor"));
        }
    }

    #[tokio::test]
    async fn test_progress_tracking_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // Call track_progress tool
        let progress_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(7)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "track_progress",
                    "arguments": {
                        "user_id": Uuid::new_v4(),
                        "metrics": [
                            {
                                "name": "body_weight",
                                "value": 75.0,
                                "unit": "kg",
                                "date": "2024-01-01T00:00:00Z",
                                "notes": "Starting weight"
                            },
                            {
                                "name": "body_weight",
                                "value": 73.5,
                                "unit": "kg",
                                "date": "2024-02-01T00:00:00Z",
                                "notes": "After 1 month"
                            }
                        ],
                        "time_range_days": 30
                    }
                })),
            }),
        };
        
        let response = protocol_guard.handle_message(progress_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let tool_result = resp.result;
            let content = tool_result.get("content").unwrap().as_array().unwrap();
            
            let first_content = &content[0];
            let text = first_content.get("text").unwrap().as_str().unwrap();
            assert!(text.contains("Progress Analysis"));
            assert!(text.contains("body_weight"));
            assert!(text.contains("75.0"));
            assert!(text.contains("73.5"));
        }
    }

    #[tokio::test]
    async fn test_resources_listing_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // List resources
        let resources_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(8)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "resources/list".to_string(),
                params: None,
            }),
        };
        
        let response = protocol_guard.handle_message(resources_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let resources = resp.result.get("resources").unwrap().as_array().unwrap();
            assert!(!resources.is_empty());
            
            let resource_names: Vec<&str> = resources
                .iter()
                .map(|resource| resource.get("name").unwrap().as_str().unwrap())
                .collect();
            
            assert!(resource_names.iter().any(|name| name.contains("Exercise")));
            assert!(resource_names.iter().any(|name| name.contains("Nutrition")));
        }
    }

    #[tokio::test]
    async fn test_prompts_listing_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // List prompts
        let prompts_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(9)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "prompts/list".to_string(),
                params: None,
            }),
        };
        
        let response = protocol_guard.handle_message(prompts_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let prompts = resp.result.get("prompts").unwrap().as_array().unwrap();
            assert!(!prompts.is_empty());
            
            let prompt_names: Vec<&str> = prompts
                .iter()
                .map(|prompt| prompt.get("name").unwrap().as_str().unwrap())
                .collect();
            
            assert!(prompt_names.contains(&"create_workout_prompt"));
            assert!(prompt_names.contains(&"nutrition_analysis_prompt"));
        }
    }

    #[tokio::test]
    async fn test_error_handling_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize
        let init_message = create_initialize_message();
        let mut protocol_guard = protocol.write().await;
        let _response = protocol_guard.handle_message(init_message).unwrap();
        
        // Call unknown tool
        let unknown_tool_message = JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(10)),
            content: MessageContent::Request(JsonRpcRequest {
                method: "tools/call".to_string(),
                params: Some(json!({
                    "name": "unknown_tool",
                    "arguments": {}
                })),
            }),
        };
        
        let response = protocol_guard.handle_message(unknown_tool_message).unwrap();
        assert!(response.is_some());
        
        if let MessageContent::Response(resp) = response.unwrap().content {
            let tool_result = resp.result;
            assert!(tool_result.get("isError").unwrap_or(&json!(false)).as_bool().unwrap_or(false));
        }
    }

    #[tokio::test]
    async fn test_concurrent_requests_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize first
        let init_message = create_initialize_message();
        {
            let mut protocol_guard = protocol.write().await;
            let _response = protocol_guard.handle_message(init_message).unwrap();
        }
        
        // Create multiple concurrent tool calls
        let mut handles = Vec::new();
        
        for i in 0..5 {
            let protocol_clone = protocol.clone();
            let handle = tokio::spawn(async move {
                let workout_message = JsonRpcMessage {
                    jsonrpc: "2.0".to_string(),
                    id: Some(json!(i + 20)),
                    content: MessageContent::Request(JsonRpcRequest {
                        method: "tools/call".to_string(),
                        params: Some(json!({
                            "name": "create_workout_plan",
                            "arguments": {
                                "user_profile": {
                                    "id": Uuid::new_v4(),
                                    "age": 25 + i,
                                    "weight_kg": 70.0,
                                    "height_cm": 175,
                                    "gender": "male",
                                    "activity_level": "moderately_active",
                                    "fitness_goals": ["muscle_gain"],
                                    "dietary_restrictions": [],
                                    "health_conditions": []
                                }
                            }
                        })),
                    }),
                };
                
                let mut protocol_guard = protocol_clone.write().await;
                protocol_guard.handle_message(workout_message)
            });
            
            handles.push(handle);
        }
        
        // Wait for all requests to complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            let response = result.unwrap();
            assert!(response.is_some());
        }
    }

    #[tokio::test]
    async fn test_session_management_workflow() {
        let server = create_test_server().await;
        let protocol = server.protocol.clone();
        
        // Initialize multiple sessions
        for i in 0..3 {
            let init_message = JsonRpcMessage {
                jsonrpc: "2.0".to_string(),
                id: Some(json!(i + 100)),
                content: MessageContent::Request(JsonRpcRequest {
                    method: "initialize".to_string(),
                    params: Some(json!({
                        "protocolVersion": MCP_VERSION,
                        "capabilities": {},
                        "clientInfo": {
                            "name": format!("Test Client {}", i),
                            "version": "1.0.0"
                        }
                    })),
                }),
            };
            
            let mut protocol_guard = protocol.write().await;
            let response = protocol_guard.handle_message(init_message).unwrap();
            assert!(response.is_some());
        }
        
        // Verify multiple sessions are tracked
        let protocol_guard = protocol.read().await;
        assert_eq!(protocol_guard.sessions.len(), 3);
    }
}