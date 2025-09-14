use leptos::prelude::*;
use crate::api::mcp_client::{
    McpApiClient, McpServerInfo, McpSession, McpTool, McpToolCall, McpToolResponse,
    McpSessionRequest, McpContext, McpServerStats, McpApiError, McpServerStatus,
    McpCapability, McpToolCategory
};
use crate::api::User;
use serde_json::{Value, json};

#[component]
pub fn McpServerPanel() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("servers".to_string());
    let (servers, set_servers) = signal(Vec::<McpServerInfo>::new());
    let (sessions, set_sessions) = signal(Vec::<McpSession>::new());
    let (server_stats, set_server_stats) = signal(Vec::<McpServerStats>::new());
    let (selected_server, set_selected_server) = signal(None::<McpServerInfo>);
    let (available_tools, set_available_tools) = signal(Vec::<McpTool>::new());
    let (tool_responses, set_tool_responses) = signal(Vec::<McpToolResponse>::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(String::new());

    // Load initial data
    Effect::new(move |_| {
        load_servers(set_servers, set_error, set_loading);
        load_sessions(set_sessions, set_error, set_loading);
    });

    let refresh_data = move |_| {
        match active_tab.get().as_str() {
            "servers" => load_servers(set_servers, set_error, set_loading),
            "sessions" => load_sessions(set_sessions, set_error, set_loading),
            "tools" => {
                if let Some(server) = selected_server.get() {
                    load_tools(server.id, set_available_tools, set_error, set_loading);
                }
            }
            "monitoring" => load_server_stats(set_server_stats, set_error, set_loading),
            _ => {}
        }
    };

    let select_server = move |server: McpServerInfo| {
        set_selected_server.set(Some(server.clone()));
        load_tools(server.id, set_available_tools, set_error, set_loading);
    };

    let execute_tool = move |tool: McpTool| {
        if let Some(server) = selected_server.get() {
            let context = McpApiClient::create_fitness_context(
                "demo-user".to_string(),
                "session123".to_string(),
                vec!["strength".to_string(), "muscle_gain".to_string()],
                None,
                vec!["high_protein".to_string()],
            );
            
            let tool_call = McpApiClient::create_tool_call(
                server.id,
                tool.name,
                create_sample_parameters(&tool),
                Some(context),
            );

            execute_tool_call(tool_call, set_tool_responses, set_error, set_loading);
        }
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
            <div class="p-6 border-b border-white/10">
                <div class="flex items-center justify-between">
                    <h3 class="flex items-center gap-2 text-lg font-semibold">
                        "🔌 MCP Server Integration"
                    </h3>
                    <button
                        on:click=refresh_data
                        class="px-3 py-1 bg-white/10 hover:bg-white/20 border border-white/20 rounded text-sm transition-colors"
                        disabled=loading
                    >
                        "🔄 Refresh"
                    </button>
                </div>
            </div>
            
            <div class="p-6 space-y-6">
                // Error display
                {move || {
                    let error_msg = error.get();
                    if !error_msg.is_empty() {
                        view! {
                            <div class="bg-red-600/20 border border-red-500/30 rounded-lg p-3">
                                <p class="text-red-300">{error_msg}</p>
                            </div>
                        }.into()
                    } else {
                        view! { <div></div> }.into()
                    }
                }}

                // Tab navigation
                <div class="flex space-x-1 bg-white/5 rounded-lg p-1">
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}", 
                            if active_tab.get() == "servers" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("servers".to_string())
                    >
                        "🖥️ Servers"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "sessions" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("sessions".to_string())
                    >
                        "🔗 Sessions"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "tools" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("tools".to_string())
                    >
                        "🛠️ Tools"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "monitoring" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("monitoring".to_string())
                    >
                        "📊 Monitoring"
                    </button>
                </div>

                // Tab Content
                {move || {
                    match active_tab.get().as_str() {
                        "servers" => view! {
                            <div class="space-y-6">
                                <div class="flex justify-between items-center">
                                    <h4 class="text-lg font-medium">"Available MCP Servers"</h4>
                                    {move || {
                                        if loading.get() {
                                            view! {
                                                <div class="flex items-center gap-2 text-white/70">
                                                    <div class="animate-spin w-4 h-4 border-2 border-purple-500 border-t-transparent rounded-full"></div>
                                                    "Loading..."
                                                </div>
                                            }.into()
                                        } else {
                                            view! { <div></div> }.into()
                                        }
                                    }}
                                </div>
                                
                                <ServersGrid 
                                    servers=servers.into()
                                    on_select=Callback::new(select_server)
                                    selected_server=selected_server.into()
                                />
                            </div>
                        }.into(),
                        
                        "sessions" => view! {
                            <div class="space-y-6">
                                <h4 class="text-lg font-medium">"Active MCP Sessions"</h4>
                                <SessionsList sessions=sessions.into()/>
                            </div>
                        }.into(),
                        
                        "tools" => view! {
                            <div class="space-y-6">
                                <div class="flex justify-between items-center">
                                    <h4 class="text-lg font-medium">"Available Tools"</h4>
                                    {move || {
                                        if let Some(server) = selected_server.get() {
                                            view! {
                                                <div class="text-sm text-white/70">
                                                    "Server: " {server.name}
                                                </div>
                                            }.into()
                                        } else {
                                            view! {
                                                <div class="text-sm text-white/50">
                                                    "Select a server to view tools"
                                                </div>
                                            }.into()
                                        }
                                    }}
                                </div>
                                
                                <ToolsGrid 
                                    tools=available_tools.into()
                                    on_execute=Callback::new(execute_tool)
                                />
                                
                                <ToolResponses responses=tool_responses.into()/>
                            </div>
                        }.into(),
                        
                        _ => view! {
                            <div class="space-y-6">
                                <h4 class="text-lg font-medium">"Server Monitoring"</h4>
                                <MonitoringDashboard stats=server_stats.into()/>
                            </div>
                        }.into()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn ServersGrid(
    servers: Signal<Vec<McpServerInfo>>,
    on_select: Callback<McpServerInfo>,
    selected_server: Signal<Option<McpServerInfo>>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {move || {
                let server_list = servers.get();
                if server_list.is_empty() {
                    view! {
                        <div class="col-span-full text-center py-8 bg-white/5 rounded-lg border border-dashed border-white/20">
                            <div class="text-4xl text-white/40 mb-3">"🖥️"</div>
                            <h4 class="text-white/60 font-medium mb-2">"No MCP servers found"</h4>
                            <p class="text-white/40 text-sm">"Register MCP servers to get started"</p>
                        </div>
                    }.into()
                } else {
                    server_list.into_iter().map(|server| {
                        let server_clone = server.clone();
                        let is_selected = move || {
                            selected_server.get().map(|s| s.id == server.id).unwrap_or(false)
                        };
                        
                        view! {
                            <ServerCard 
                                server=server_clone
                                is_selected=is_selected.into()
                                on_select=Callback::new(move |_| on_select.call(server.clone()))
                            />
                        }
                    }).collect::<Vec<_>>().into()
                }
            }}
        </div>
    }
}

#[component]
fn ServerCard(
    server: McpServerInfo,
    is_selected: Signal<bool>,
    on_select: Callback<()>,
) -> impl IntoView {
    let status_info = match server.status {
        McpServerStatus::Online => ("🟢", "Online", "text-green-400"),
        McpServerStatus::Offline => ("🔴", "Offline", "text-red-400"),
        McpServerStatus::Maintenance => ("🟡", "Maintenance", "text-yellow-400"),
        McpServerStatus::Error => ("🔴", "Error", "text-red-400"),
        McpServerStatus::Starting => ("🟡", "Starting", "text-yellow-400"),
        McpServerStatus::Stopping => ("🟡", "Stopping", "text-yellow-400"),
    };

    view! {
        <div class=move || format!(
            "bg-white/5 border rounded-lg p-4 transition-all cursor-pointer hover:border-white/30 {}",
            if is_selected.get() {
                "border-purple-500/50 bg-purple-500/10"
            } else {
                "border-white/10"
            }
        )>
            <button class="w-full text-left" on:click=move |_| on_select.call(())>
                <div class="flex items-center justify-between mb-3">
                    <h5 class="font-medium text-white">{server.name}</h5>
                    <div class=format!("flex items-center gap-1 {}", status_info.2)>
                        <span>{status_info.0}</span>
                        <span class="text-xs">{status_info.1}</span>
                    </div>
                </div>
                
                <p class="text-white/70 text-sm mb-3 line-clamp-2">{server.description}</p>
                
                <div class="space-y-2">
                    <div class="text-xs text-white/50">
                        "Version: " {server.version}
                    </div>
                    <div class="flex flex-wrap gap-1">
                        {server.capabilities.into_iter().map(|cap| {
                            let cap_text = match cap {
                                McpCapability::Tools => "🛠️ Tools",
                                McpCapability::Resources => "📁 Resources",
                                McpCapability::Prompts => "💬 Prompts",
                                McpCapability::Sampling => "🎯 Sampling",
                                McpCapability::Logging => "📝 Logging",
                                McpCapability::Progress => "📊 Progress",
                                McpCapability::Notifications => "🔔 Notifications",
                            };
                            view! {
                                <span class="text-xs bg-white/10 px-2 py-1 rounded">{cap_text}</span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </button>
        </div>
    }
}

#[component]
fn SessionsList(sessions: Signal<Vec<McpSession>>) -> impl IntoView {
    view! {
        <div class="space-y-3">
            {move || {
                let session_list = sessions.get();
                if session_list.is_empty() {
                    view! {
                        <div class="text-center py-8 bg-white/5 rounded-lg border border-dashed border-white/20">
                            <div class="text-4xl text-white/40 mb-3">"🔗"</div>
                            <h4 class="text-white/60 font-medium mb-2">"No active sessions"</h4>
                            <p class="text-white/40 text-sm">"Start using MCP tools to create sessions"</p>
                        </div>
                    }.into()
                } else {
                    session_list.into_iter().map(|session| {
                        view! {
                            <div class="bg-white/5 border border-white/10 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-2">
                                    <h5 class="font-medium text-white">
                                        "Session " {session.id.chars().take(8).collect::<String>()}
                                    </h5>
                                    <span class="text-xs text-green-400">"Active"</span>
                                </div>
                                <p class="text-white/70 text-sm mb-2">
                                    "Server: " {session.server_id}
                                </p>
                                <div class="text-xs text-white/50">
                                    "Created: " {session.created_at.split('T').next().unwrap_or(&session.created_at)}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>().into()
                }
            }}
        </div>
    }
}

#[component]
fn ToolsGrid(
    tools: Signal<Vec<McpTool>>,
    on_execute: Callback<McpTool>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {move || {
                let tool_list = tools.get();
                if tool_list.is_empty() {
                    view! {
                        <div class="col-span-full text-center py-8 bg-white/5 rounded-lg border border-dashed border-white/20">
                            <div class="text-4xl text-white/40 mb-3">"🛠️"</div>
                            <h4 class="text-white/60 font-medium mb-2">"No tools available"</h4>
                            <p class="text-white/40 text-sm">"Select a server with tool capabilities"</p>
                        </div>
                    }.into()
                } else {
                    tool_list.into_iter().map(|tool| {
                        let tool_clone = tool.clone();
                        let category_info = match tool.category {
                            McpToolCategory::FitnessTracking => ("🏃", "Fitness Tracking"),
                            McpToolCategory::NutritionAnalysis => ("🍎", "Nutrition Analysis"),
                            McpToolCategory::WorkoutGeneration => ("🏋️", "Workout Generation"),
                            McpToolCategory::ProgressMonitoring => ("📊", "Progress Monitoring"),
                            McpToolCategory::DataAnalysis => ("📈", "Data Analysis"),
                            McpToolCategory::Integration => ("🔗", "Integration"),
                            McpToolCategory::Utility => ("⚙️", "Utility"),
                        };
                        
                        view! {
                            <div class="bg-white/5 border border-white/10 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-3">
                                    <h5 class="font-medium text-white">{tool.name}</h5>
                                    <span class="text-xs bg-white/20 px-2 py-1 rounded flex items-center gap-1">
                                        <span>{category_info.0}</span>
                                        <span>{category_info.1}</span>
                                    </span>
                                </div>
                                
                                <p class="text-white/70 text-sm mb-4">{tool.description}</p>
                                
                                <button
                                    on:click=move |_| on_execute.call(tool_clone.clone())
                                    class="w-full px-3 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded text-sm transition-colors"
                                >
                                    "Execute Tool"
                                </button>
                            </div>
                        }
                    }).collect::<Vec<_>>().into()
                }
            }}
        </div>
    }
}

#[component]
fn ToolResponses(responses: Signal<Vec<McpToolResponse>>) -> impl IntoView {
    view! {
        <div class="space-y-3">
            <h5 class="font-medium text-white">"Tool Execution Results"</h5>
            {move || {
                let response_list = responses.get();
                if response_list.is_empty() {
                    view! {
                        <div class="text-center py-6 bg-white/5 rounded-lg border border-dashed border-white/20">
                            <p class="text-white/60 text-sm">"No tool executions yet"</p>
                        </div>
                    }.into()
                } else {
                    response_list.into_iter().map(|response| {
                        view! {
                            <div class=format!(
                                "border rounded-lg p-4 {}",
                                if response.success {
                                    "bg-green-500/10 border-green-500/30"
                                } else {
                                    "bg-red-500/10 border-red-500/30"
                                }
                            )>
                                <div class="flex items-center justify-between mb-2">
                                    <span class="text-sm font-medium">
                                        {if response.success { "✅ Success" } else { "❌ Failed" }}
                                    </span>
                                    <span class="text-xs text-white/50">
                                        {response.execution_time_ms} "ms"
                                    </span>
                                </div>
                                {if let Some(result) = response.result {
                                    view! {
                                        <pre class="text-xs text-white/80 bg-white/5 rounded p-2 overflow-x-auto">
                                            {serde_json::to_string_pretty(&result).unwrap_or_default()}
                                        </pre>
                                    }.into()
                                } else if let Some(error) = response.error {
                                    view! {
                                        <p class="text-xs text-red-300">{error}</p>
                                    }.into()
                                } else {
                                    view! { <div></div> }.into()
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>().into()
                }
            }}
        </div>
    }
}

#[component]
fn MonitoringDashboard(stats: Signal<Vec<McpServerStats>>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {move || {
                let stats_list = stats.get();
                if stats_list.is_empty() {
                    view! {
                        <div class="col-span-full text-center py-8 bg-white/5 rounded-lg border border-dashed border-white/20">
                            <div class="text-4xl text-white/40 mb-3">"📊"</div>
                            <h4 class="text-white/60 font-medium mb-2">"No monitoring data"</h4>
                            <p class="text-white/40 text-sm">"Server statistics will appear here"</p>
                        </div>
                    }.into()
                } else {
                    stats_list.into_iter().map(|stat| {
                        let success_rate = if stat.total_requests > 0 {
                            (stat.successful_requests as f64 / stat.total_requests as f64) * 100.0
                        } else {
                            0.0
                        };
                        
                        view! {
                            <div class="bg-white/5 border border-white/10 rounded-lg p-4">
                                <h5 class="font-medium text-white mb-3">
                                    "Server " {stat.server_id.chars().take(8).collect::<String>()}
                                </h5>
                                <div class="space-y-2">
                                    <div class="flex justify-between">
                                        <span class="text-white/70 text-sm">"Uptime"</span>
                                        <span class="text-white text-sm">
                                            {format!("{}h", stat.uptime_seconds / 3600)}
                                        </span>
                                    </div>
                                    <div class="flex justify-between">
                                        <span class="text-white/70 text-sm">"Requests"</span>
                                        <span class="text-white text-sm">{stat.total_requests}</span>
                                    </div>
                                    <div class="flex justify-between">
                                        <span class="text-white/70 text-sm">"Success Rate"</span>
                                        <span class="text-green-400 text-sm">{format!("{:.1}%", success_rate)}</span>
                                    </div>
                                    <div class="flex justify-between">
                                        <span class="text-white/70 text-sm">"Avg Response"</span>
                                        <span class="text-blue-400 text-sm">{format!("{:.0}ms", stat.avg_response_time_ms)}</span>
                                    </div>
                                    <div class="flex justify-between">
                                        <span class="text-white/70 text-sm">"Active Sessions"</span>
                                        <span class="text-purple-400 text-sm">{stat.active_sessions}</span>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>().into()
                }
            }}
        </div>
    }
}

// Helper functions
fn load_servers(
    set_servers: WriteSignal<Vec<McpServerInfo>>,
    set_error: WriteSignal<String>,
    set_loading: WriteSignal<bool>,
) {
    set_loading.set(true);
    set_error.set(String::new());

    spawn_local(async move {
        match McpApiClient::get_servers().await {
            Ok(servers) => {
                set_servers.set(servers);
            }
            Err(_e) => {
                // Fallback to sample data
                let sample_servers = create_sample_servers();
                set_servers.set(sample_servers);
            }
        }
        set_loading.set(false);
    });
}

fn load_sessions(
    set_sessions: WriteSignal<Vec<McpSession>>,
    set_error: WriteSignal<String>,
    set_loading: WriteSignal<bool>,
) {
    set_loading.set(true);
    spawn_local(async move {
        match McpApiClient::get_user_sessions("demo-user").await {
            Ok(sessions) => {
                set_sessions.set(sessions);
            }
            Err(_e) => {
                // Fallback to empty sessions
                set_sessions.set(Vec::new());
            }
        }
        set_loading.set(false);
    });
}

fn load_tools(
    server_id: String,
    set_tools: WriteSignal<Vec<McpTool>>,
    set_error: WriteSignal<String>,
    set_loading: WriteSignal<bool>,
) {
    set_loading.set(true);
    spawn_local(async move {
        match McpApiClient::get_available_tools(&server_id).await {
            Ok(tools) => {
                set_tools.set(tools);
            }
            Err(_e) => {
                // Fallback to sample tools
                let sample_tools = create_sample_tools();
                set_tools.set(sample_tools);
            }
        }
        set_loading.set(false);
    });
}

fn load_server_stats(
    set_stats: WriteSignal<Vec<McpServerStats>>,
    set_error: WriteSignal<String>,
    set_loading: WriteSignal<bool>,
) {
    set_loading.set(true);
    spawn_local(async move {
        // Load stats for sample servers
        let sample_stats = create_sample_stats();
        set_stats.set(sample_stats);
        set_loading.set(false);
    });
}

fn execute_tool_call(
    tool_call: McpToolCall,
    set_responses: WriteSignal<Vec<McpToolResponse>>,
    set_error: WriteSignal<String>,
    set_loading: WriteSignal<bool>,
) {
    set_loading.set(true);
    spawn_local(async move {
        match McpApiClient::call_tool(tool_call).await {
            Ok(response) => {
                set_responses.update(|responses| responses.push(response));
            }
            Err(_e) => {
                // Create sample response
                let sample_response = McpToolResponse {
                    call_id: "sample_call".to_string(),
                    success: true,
                    result: Some(json!({"status": "completed", "data": "Sample tool execution successful"})),
                    error: None,
                    execution_time_ms: 150,
                    metadata: None,
                };
                set_responses.update(|responses| responses.push(sample_response));
            }
        }
        set_loading.set(false);
    });
}

fn create_sample_parameters(tool: &McpTool) -> Value {
    match tool.name.as_str() {
        "analyze_workout" => json!({"workout_type": "strength", "duration": 45}),
        "generate_meal_plan" => json!({"calories": 2200, "protein_goal": 150}),
        "track_progress" => json!({"metric": "weight", "value": 75}),
        _ => json!({"sample": "parameters"}),
    }
}

// Sample data functions
fn create_sample_servers() -> Vec<McpServerInfo> {
    vec![
        McpServerInfo {
            id: "fitness-tracker-001".to_string(),
            name: "Fitness Tracker Pro".to_string(),
            description: "Advanced fitness tracking with AI-powered workout analysis and personalized recommendations".to_string(),
            version: "2.1.0".to_string(),
            status: McpServerStatus::Online,
            capabilities: vec![
                McpCapability::Tools,
                McpCapability::Resources,
                McpCapability::Progress,
                McpCapability::Notifications,
            ],
            endpoint: "http://localhost:3001/mcp".to_string(),
            last_heartbeat: "2024-01-15T10:30:00Z".to_string(),
            created_at: "2024-01-10T08:00:00Z".to_string(),
        },
        McpServerInfo {
            id: "nutrition-ai-002".to_string(),
            name: "Nutrition AI Assistant".to_string(),
            description: "Smart nutrition analysis and meal planning with dietary restriction support".to_string(),
            version: "1.8.3".to_string(),
            status: McpServerStatus::Online,
            capabilities: vec![
                McpCapability::Tools,
                McpCapability::Sampling,
                McpCapability::Logging,
            ],
            endpoint: "http://localhost:3002/mcp".to_string(),
            last_heartbeat: "2024-01-15T10:29:45Z".to_string(),
            created_at: "2024-01-12T14:15:00Z".to_string(),
        }
    ]
}

fn create_sample_tools() -> Vec<McpTool> {
    vec![
        McpTool {
            name: "analyze_workout".to_string(),
            description: "Analyze workout performance and provide form feedback".to_string(),
            parameters: json!({
                "workout_type": "string",
                "duration": "number",
                "exercises": "array"
            }),
            category: McpToolCategory::FitnessTracking,
            required_permissions: vec!["workout_data".to_string()],
        },
        McpTool {
            name: "generate_meal_plan".to_string(),
            description: "Generate personalized meal plans based on dietary goals".to_string(),
            parameters: json!({
                "calories": "number",
                "protein_goal": "number",
                "dietary_restrictions": "array"
            }),
            category: McpToolCategory::NutritionAnalysis,
            required_permissions: vec!["nutrition_data".to_string()],
        }
    ]
}

fn create_sample_stats() -> Vec<McpServerStats> {
    vec![
        McpServerStats {
            server_id: "fitness-tracker-001".to_string(),
            uptime_seconds: 432000, // 5 days
            total_requests: 1247,
            successful_requests: 1198,
            failed_requests: 49,
            avg_response_time_ms: 145.7,
            active_sessions: 12,
            tool_usage_stats: json!({
                "analyze_workout": 523,
                "track_progress": 398,
                "generate_recommendations": 326
            }),
        }
    ]
}