use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::collections::HashMap;
use uuid::Uuid;

use crate::mcp::types::*;

pub struct MCPProtocol {
    pub server_info: ServerInfo,
    pub capabilities: ServerCapabilities,
    pub sessions: HashMap<Uuid, MCPSession>,
}

impl MCPProtocol {
    pub fn new() -> Self {
        Self {
            server_info: ServerInfo {
                name: "Fitness Advisor AI MCP Server".to_string(),
                version: "1.0.0".to_string(),
                description: "AI-powered fitness and nutrition advisory server with RAG capabilities".to_string(),
                author: "Fitness Advisor AI Team".to_string(),
                license: "MIT".to_string(),
            },
            capabilities: ServerCapabilities {
                logging: Some(LoggingCapability {}),
                prompts: Some(PromptsCapability {
                    list_changed: Some(true),
                }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(true),
                    list_changed: Some(true),
                }),
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
            },
            sessions: HashMap::new(),
        }
    }

    pub fn handle_message(&mut self, message: JsonRpcMessage) -> Result<Option<JsonRpcMessage>> {
        let message_id = message.id.clone();
        match message.content {
            MessageContent::Request(request) => {
                let response = self.handle_request(request, message_id.clone())?;
                Ok(Some(JsonRpcMessage {
                    jsonrpc: "2.0".to_string(),
                    id: message_id,
                    content: MessageContent::Response(response),
                }))
            }
            MessageContent::Notification(notification) => {
                self.handle_notification(notification)?;
                Ok(None) // Notifications don't require responses
            }
            MessageContent::Response(_) | MessageContent::Error(_) => {
                // These are responses to our requests, not something we handle as a server
                Ok(None)
            }
        }
    }

    fn handle_request(&mut self, request: JsonRpcRequest, id: Option<Value>) -> Result<JsonRpcResponse> {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params),
            "initialized" => Ok(JsonRpcResponse {
                result: json!({}),
            }),
            "tools/list" => self.handle_tools_list(),
            "tools/call" => self.handle_tools_call(request.params),
            "resources/list" => self.handle_resources_list(),
            "resources/read" => self.handle_resources_read(request.params),
            "prompts/list" => self.handle_prompts_list(),
            "prompts/get" => self.handle_prompts_get(request.params),
            "logging/setLevel" => self.handle_logging_set_level(request.params),
            method => Err(anyhow!(
                "Method not found: {}",
                method
            )),
        }
    }

    fn handle_notification(&mut self, notification: JsonRpcNotification) -> Result<()> {
        match notification.method.as_str() {
            "notifications/cancelled" => {
                // Handle cancellation notification
                if let Some(params) = notification.params {
                    tracing::info!("Received cancellation: {:?}", params);
                }
                Ok(())
            }
            "notifications/progress" => {
                // Handle progress notification
                if let Some(params) = notification.params {
                    tracing::debug!("Progress update: {:?}", params);
                }
                Ok(())
            }
            method => {
                tracing::warn!("Unknown notification method: {}", method);
                Ok(())
            }
        }
    }

    fn handle_initialize(&mut self, params: Option<Value>) -> Result<JsonRpcResponse> {
        let params: InitializeParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => return Err(anyhow!("Initialize requires parameters")),
        };

        // Validate protocol version
        if params.protocol_version != MCP_VERSION {
            return Err(anyhow!(
                "Unsupported protocol version: {}. Expected: {}",
                params.protocol_version,
                MCP_VERSION
            ));
        }

        // Create session
        let session_id = Uuid::new_v4();
        let session = MCPSession {
            id: session_id,
            client_info: params.client_info,
            capabilities: params.capabilities,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            is_active: true,
        };

        self.sessions.insert(session_id, session);

        let result = InitializeResult {
            protocol_version: MCP_VERSION.to_string(),
            capabilities: self.capabilities.clone(),
            server_info: self.server_info.clone(),
        };

        Ok(JsonRpcResponse {
            result: serde_json::to_value(result)?,
        })
    }

    fn handle_tools_list(&self) -> Result<JsonRpcResponse> {
        let tools = vec![
            Tool {
                name: "create_workout_plan".to_string(),
                description: "Create a personalized workout plan based on user goals and preferences".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "user_profile": {
                            "type": "object",
                            "properties": {
                                "age": {"type": "number"},
                                "weight_kg": {"type": "number"},
                                "height_cm": {"type": "number"},
                                "fitness_goals": {
                                    "type": "array",
                                    "items": {"type": "string"}
                                },
                                "activity_level": {"type": "string"},
                                "equipment_available": {
                                    "type": "array",
                                    "items": {"type": "string"}
                                }
                            },
                            "required": ["age", "weight_kg", "height_cm", "fitness_goals", "activity_level"]
                        },
                        "workout_preferences": {
                            "type": "object",
                            "properties": {
                                "duration_minutes": {"type": "number"},
                                "days_per_week": {"type": "number"},
                                "difficulty_level": {"type": "string"}
                            }
                        }
                    },
                    "required": ["user_profile"]
                }),
            },
            Tool {
                name: "create_nutrition_plan".to_string(),
                description: "Generate a personalized nutrition plan with meal suggestions".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "user_profile": {
                            "type": "object",
                            "properties": {
                                "age": {"type": "number"},
                                "weight_kg": {"type": "number"},
                                "height_cm": {"type": "number"},
                                "activity_level": {"type": "string"},
                                "fitness_goals": {
                                    "type": "array",
                                    "items": {"type": "string"}
                                },
                                "dietary_restrictions": {
                                    "type": "array",
                                    "items": {"type": "string"}
                                }
                            },
                            "required": ["age", "weight_kg", "height_cm", "activity_level", "fitness_goals"]
                        },
                        "calorie_target": {"type": "number"},
                        "meal_preferences": {
                            "type": "object",
                            "properties": {
                                "meals_per_day": {"type": "number"},
                                "prep_time_minutes": {"type": "number"}
                            }
                        }
                    },
                    "required": ["user_profile"]
                }),
            },
            Tool {
                name: "analyze_nutrition".to_string(),
                description: "Analyze nutritional content of foods and meals with micronutrient interactions".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "foods": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"},
                                    "quantity": {"type": "number"},
                                    "unit": {"type": "string"}
                                },
                                "required": ["name", "quantity", "unit"]
                            }
                        },
                        "analysis_type": {
                            "type": "string",
                            "enum": ["basic", "micronutrients", "interactions", "timing"]
                        }
                    },
                    "required": ["foods"]
                }),
            },
            Tool {
                name: "rag_fitness_query".to_string(),
                description: "Query the RAG knowledge base for evidence-based fitness and nutrition information".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "max_sources": {"type": "number", "default": 5},
                        "focus_area": {
                            "type": "string",
                            "enum": ["exercise", "nutrition", "recovery", "supplements", "general"]
                        }
                    },
                    "required": ["query"]
                }),
            },
        ];

        Ok(JsonRpcResponse {
            result: json!({
                "tools": tools
            }),
        })
    }

    fn handle_tools_call(&self, params: Option<Value>) -> Result<JsonRpcResponse> {
        let request: CallToolRequest = match params {
            Some(p) => serde_json::from_value(p)?,
            None => return Err(anyhow!("Tool call requires parameters")),
        };

        let result = match request.name.as_str() {
            "create_workout_plan" => self.call_create_workout_plan(request.arguments),
            "create_nutrition_plan" => self.call_create_nutrition_plan(request.arguments),
            "analyze_nutrition" => self.call_analyze_nutrition(request.arguments),
            "rag_fitness_query" => self.call_rag_fitness_query(request.arguments),
            _ => {
                return Ok(JsonRpcResponse {
                    result: serde_json::to_value(CallToolResult {
                        content: vec![ToolResponseContent::Text {
                            text: format!("Unknown tool: {}", request.name),
                        }],
                        is_error: Some(true),
                    })?,
                })
            }
        }?;

        Ok(JsonRpcResponse {
            result: serde_json::to_value(result)?,
        })
    }

    fn call_create_workout_plan(&self, args: Option<Value>) -> Result<CallToolResult> {
        // This would integrate with the existing workout planning system
        let response_text = if let Some(args) = args {
            format!("Creating workout plan with parameters: {}", 
                serde_json::to_string_pretty(&args)?)
        } else {
            "Created a basic full-body workout plan".to_string()
        };

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: response_text,
            }],
            is_error: None,
        })
    }

    fn call_create_nutrition_plan(&self, args: Option<Value>) -> Result<CallToolResult> {
        // This would integrate with the nutrition planning system
        let response_text = if let Some(args) = args {
            format!("Creating nutrition plan with parameters: {}", 
                serde_json::to_string_pretty(&args)?)
        } else {
            "Created a balanced nutrition plan".to_string()
        };

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: response_text,
            }],
            is_error: None,
        })
    }

    fn call_analyze_nutrition(&self, args: Option<Value>) -> Result<CallToolResult> {
        // This would integrate with advanced nutrition analysis
        let response_text = if let Some(args) = args {
            format!("Analyzing nutrition with parameters: {}", 
                serde_json::to_string_pretty(&args)?)
        } else {
            "Performed basic nutrition analysis".to_string()
        };

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: response_text,
            }],
            is_error: None,
        })
    }

    fn call_rag_fitness_query(&self, args: Option<Value>) -> Result<CallToolResult> {
        // This would integrate with the existing RAG system
        let response_text = if let Some(args) = args {
            if let Ok(query_data) = serde_json::from_value::<serde_json::Map<String, Value>>(args) {
                if let Some(query) = query_data.get("query").and_then(|v| v.as_str()) {
                    format!("RAG Query: '{}'\n\nBased on the fitness knowledge base, here's what I found:\n[This would integrate with your existing RAG system to provide evidence-based answers]", query)
                } else {
                    "Invalid query format".to_string()
                }
            } else {
                "Failed to parse query parameters".to_string()
            }
        } else {
            "No query provided".to_string()
        };

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: response_text,
            }],
            is_error: None,
        })
    }

    fn handle_resources_list(&self) -> Result<JsonRpcResponse> {
        let resources = vec![
            Resource {
                uri: "fitness://knowledge_base/exercise_science".to_string(),
                name: "Exercise Science Knowledge Base".to_string(),
                description: Some("Comprehensive database of exercise science research and guidelines".to_string()),
                mime_type: "application/json".to_string(),
            },
            Resource {
                uri: "fitness://knowledge_base/nutrition".to_string(),
                name: "Nutrition Knowledge Base".to_string(),
                description: Some("Evidence-based nutrition information and meal planning data".to_string()),
                mime_type: "application/json".to_string(),
            },
            Resource {
                uri: "fitness://templates/workout_plans".to_string(),
                name: "Workout Plan Templates".to_string(),
                description: Some("Pre-designed workout plans for various fitness goals".to_string()),
                mime_type: "application/json".to_string(),
            },
        ];

        Ok(JsonRpcResponse {
            result: json!({
                "resources": resources
            }),
        })
    }

    fn handle_resources_read(&self, params: Option<Value>) -> Result<JsonRpcResponse> {
        let request: ReadResourceRequest = match params {
            Some(p) => serde_json::from_value(p)?,
            None => return Err(anyhow!("Resource read requires parameters")),
        };

        let content = match request.uri.as_str() {
            "fitness://knowledge_base/exercise_science" => {
                ResourceContent::Text {
                    uri: request.uri,
                    text: json!({
                        "categories": ["cardiovascular", "strength", "flexibility", "recovery"],
                        "research_papers": 150,
                        "guidelines": ["ACSM", "WHO", "CDC"]
                    }).to_string(),
                    mime_type: "application/json".to_string(),
                }
            }
            "fitness://knowledge_base/nutrition" => {
                ResourceContent::Text {
                    uri: request.uri,
                    text: json!({
                        "categories": ["macronutrients", "micronutrients", "hydration", "supplements"],
                        "meal_plans": 50,
                        "dietary_approaches": ["Mediterranean", "DASH", "Ketogenic", "Plant-based"]
                    }).to_string(),
                    mime_type: "application/json".to_string(),
                }
            }
            _ => {
                return Err(anyhow!("Resource not found: {}", request.uri));
            }
        };

        Ok(JsonRpcResponse {
            result: serde_json::to_value(ReadResourceResult {
                contents: vec![content],
            })?,
        })
    }

    fn handle_prompts_list(&self) -> Result<JsonRpcResponse> {
        let prompts = vec![
            Prompt {
                name: "create_workout_prompt".to_string(),
                description: "Generate a structured workout plan prompt".to_string(),
                arguments: Some(vec![
                    PromptArgument {
                        name: "fitness_goal".to_string(),
                        description: "Primary fitness goal (strength, endurance, weight loss, etc.)".to_string(),
                        required: Some(true),
                    },
                    PromptArgument {
                        name: "experience_level".to_string(),
                        description: "User's experience level (beginner, intermediate, advanced)".to_string(),
                        required: Some(true),
                    },
                    PromptArgument {
                        name: "available_time".to_string(),
                        description: "Available workout time in minutes".to_string(),
                        required: Some(false),
                    },
                ]),
            },
            Prompt {
                name: "nutrition_analysis_prompt".to_string(),
                description: "Generate a nutrition analysis and recommendations prompt".to_string(),
                arguments: Some(vec![
                    PromptArgument {
                        name: "dietary_approach".to_string(),
                        description: "Preferred dietary approach or restrictions".to_string(),
                        required: Some(false),
                    },
                    PromptArgument {
                        name: "health_goals".to_string(),
                        description: "Health and fitness goals".to_string(),
                        required: Some(true),
                    },
                ]),
            },
        ];

        Ok(JsonRpcResponse {
            result: json!({
                "prompts": prompts
            }),
        })
    }

    fn handle_prompts_get(&self, params: Option<Value>) -> Result<JsonRpcResponse> {
        let request: GetPromptRequest = match params {
            Some(p) => serde_json::from_value(p)?,
            None => return Err(anyhow!("Prompt get requires parameters")),
        };

        let result = match request.name.as_str() {
            "create_workout_prompt" => {
                let fitness_goal = request.arguments
                    .as_ref()
                    .and_then(|args| args.get("fitness_goal"))
                    .unwrap_or("general fitness");
                
                let experience_level = request.arguments
                    .as_ref()
                    .and_then(|args| args.get("experience_level"))
                    .unwrap_or("beginner");

                GetPromptResult {
                    description: "Workout plan creation prompt".to_string(),
                    messages: vec![
                        PromptMessage {
                            role: "user".to_string(),
                            content: PromptContent::Text {
                                text: format!(
                                    "Create a detailed workout plan for someone with a {} fitness goal at {} level. \
                                    Include exercises, sets, reps, and rest periods. \
                                    Focus on proper form and progressive overload principles.",
                                    fitness_goal, experience_level
                                ),
                            },
                        },
                    ],
                }
            }
            "nutrition_analysis_prompt" => {
                let health_goals = request.arguments
                    .as_ref()
                    .and_then(|args| args.get("health_goals"))
                    .unwrap_or("general health");

                GetPromptResult {
                    description: "Nutrition analysis and recommendations prompt".to_string(),
                    messages: vec![
                        PromptMessage {
                            role: "user".to_string(),
                            content: PromptContent::Text {
                                text: format!(
                                    "Analyze the nutritional needs for someone with {} goals. \
                                    Provide macronutrient recommendations, meal timing suggestions, \
                                    and key micronutrients to focus on.",
                                    health_goals
                                ),
                            },
                        },
                    ],
                }
            }
            _ => return Err(anyhow!("Prompt not found: {}", request.name)),
        };

        Ok(JsonRpcResponse {
            result: serde_json::to_value(result)?,
        })
    }

    fn handle_logging_set_level(&self, params: Option<Value>) -> Result<JsonRpcResponse> {
        if let Some(params) = params {
            tracing::info!("Logging level set: {:?}", params);
        }
        Ok(JsonRpcResponse {
            result: json!({}),
        })
    }

    pub fn create_error_response(id: Option<Value>, code: i32, message: String) -> JsonRpcMessage {
        JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id,
            content: MessageContent::Error(JsonRpcError {
                error: ErrorObject {
                    code,
                    message,
                    data: None,
                },
            }),
        }
    }

    pub fn send_log(&self, level: LogLevel, message: String, data: Option<Value>) -> JsonRpcMessage {
        JsonRpcMessage {
            jsonrpc: "2.0".to_string(),
            id: None, // Notifications don't have IDs
            content: MessageContent::Notification(JsonRpcNotification {
                method: "notifications/message".to_string(),
                params: Some(serde_json::to_value(LoggingMessage {
                    level,
                    data,
                    logger: "fitness-advisor-mcp".to_string(),
                }).unwrap()),
            }),
        }
    }
}

impl Default for MCPProtocol {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience type aliases
pub type MCPMessage = JsonRpcMessage;
pub type MCPRequest = JsonRpcRequest;
pub type MCPResponse = JsonRpcResponse;
pub type MCPError = JsonRpcError;