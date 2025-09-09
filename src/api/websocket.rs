use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeWorkoutData {
    pub user_id: String,
    pub exercise: String,
    pub rep_count: u32,
    pub form_score: f32,
    pub calories_burned: u32,
    pub heart_rate: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeNutritionUpdate {
    pub user_id: String,
    pub food_item: String,
    pub calories_added: u32,
    pub total_calories_today: u32,
    pub macros: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    WorkoutUpdate(RealtimeWorkoutData),
    NutritionUpdate(RealtimeNutritionUpdate),
    SystemNotification(String),
    Error(String),
}

#[derive(Clone)]
pub struct WebSocketService {
    url: String,
    websocket: Option<WebSocket>,
    message_handlers: Vec<Box<dyn Fn(WebSocketEvent) + 'static>>,
}

impl WebSocketService {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            websocket: None,
            message_handlers: Vec::new(),
        }
    }

    pub fn connect(&mut self) -> Result<(), JsValue> {
        let ws = WebSocket::new(&self.url)?;
        
        // Set up onopen handler
        let onopen = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"WebSocket connection opened".into());
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();

        // Set up onmessage handler
        let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let message_str = String::from(&txt);
                if let Ok(ws_message) = serde_json::from_str::<WebSocketMessage>(&message_str) {
                    handle_websocket_message(ws_message);
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();

        // Set up onerror handler
        let onerror = Closure::wrap(Box::new(move |e: ErrorEvent| {
            web_sys::console::log_1(&format!("WebSocket error: {:?}", e).into());
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        // Set up onclose handler
        let onclose = Closure::wrap(Box::new(move |e: CloseEvent| {
            web_sys::console::log_1(&format!("WebSocket closed: {} - {}", e.code(), e.reason()).into());
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget();

        self.websocket = Some(ws);
        Ok(())
    }

    pub fn send_message(&self, message: &WebSocketMessage) -> Result<(), JsValue> {
        if let Some(ws) = &self.websocket {
            let message_str = serde_json::to_string(message)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;
            ws.send_with_str(&message_str)?;
        }
        Ok(())
    }

    pub fn send_workout_data(&self, data: &RealtimeWorkoutData) -> Result<(), JsValue> {
        let message = WebSocketMessage {
            message_type: "workout_update".to_string(),
            data: serde_json::to_value(data)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?,
            timestamp: js_sys::Date::now().to_string(),
        };
        self.send_message(&message)
    }

    pub fn send_nutrition_update(&self, data: &RealtimeNutritionUpdate) -> Result<(), JsValue> {
        let message = WebSocketMessage {
            message_type: "nutrition_update".to_string(),
            data: serde_json::to_value(data)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?,
            timestamp: js_sys::Date::now().to_string(),
        };
        self.send_message(&message)
    }

    pub fn disconnect(&mut self) {
        if let Some(ws) = &self.websocket {
            let _ = ws.close();
        }
        self.websocket = None;
    }

    pub fn is_connected(&self) -> bool {
        self.websocket.as_ref()
            .map(|ws| ws.ready_state() == WebSocket::OPEN)
            .unwrap_or(false)
    }
}

// Global message handler
fn handle_websocket_message(message: WebSocketMessage) {
    match message.message_type.as_str() {
        "workout_update" => {
            if let Ok(data) = serde_json::from_value::<RealtimeWorkoutData>(message.data) {
                // Trigger reactive updates for workout data
                web_sys::console::log_1(&format!("Workout update: {:?}", data).into());
            }
        },
        "nutrition_update" => {
            if let Ok(data) = serde_json::from_value::<RealtimeNutritionUpdate>(message.data) {
                // Trigger reactive updates for nutrition data
                web_sys::console::log_1(&format!("Nutrition update: {:?}", data).into());
            }
        },
        "system_notification" => {
            if let Ok(notification) = message.data.as_str() {
                web_sys::console::log_1(&format!("System notification: {}", notification).into());
            }
        },
        _ => {
            web_sys::console::log_1(&format!("Unknown message type: {}", message.message_type).into());
        }
    }
}

// Leptos reactive WebSocket hook
#[derive(Clone)]
pub struct WebSocketContext {
    pub service: WebSocketService,
    pub connection_status: ReadSignal<bool>,
    pub last_message: ReadSignal<Option<WebSocketEvent>>,
}

pub fn provide_websocket_context(url: &str) -> WebSocketContext {
    let mut service = WebSocketService::new(url);
    let (connection_status, set_connection_status) = signal(false);
    let (last_message, set_last_message) = signal(None::<WebSocketEvent>);
    
    // Try to connect
    spawn_local(async move {
        if service.connect().is_ok() {
            set_connection_status.set(true);
        }
    });
    
    WebSocketContext {
        service,
        connection_status,
        last_message,
    }
}

// Hook to use WebSocket in components
pub fn use_websocket() -> Option<WebSocketContext> {
    use_context::<WebSocketContext>()
}

// WebSocket status component
#[component]
pub fn WebSocketStatus() -> impl IntoView {
    let ws_context = use_websocket();
    
    view! {
        <div class="flex items-center gap-2 text-sm">
            {move || {
                if let Some(ctx) = &ws_context {
                    let is_connected = ctx.connection_status.get();
                    view! {
                        <div class="flex items-center gap-2">
                            <div class={format!("w-2 h-2 rounded-full {}", 
                                if is_connected { "bg-green-500 animate-pulse" } else { "bg-red-500" }
                            )}></div>
                            <span class="text-white/70">
                                {if is_connected { "WebSocket Connected" } else { "WebSocket Disconnected" }}
                            </span>
                        </div>
                    }.into()
                } else {
                    view! {
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 bg-gray-500 rounded-full"></div>
                            <span class="text-white/70">"WebSocket Not Available"</span>
                        </div>
                    }.into()
                }
            }}
        </div>
    }
}

// Real-time workout tracker component using WebSocket
#[component]
pub fn RealtimeWorkoutTracker() -> impl IntoView {
    let ws_context = use_websocket();
    let (workout_data, set_workout_data) = signal(None::<RealtimeWorkoutData>);
    
    // Simulate sending workout data
    let send_workout_update = move |_| {
        if let Some(ctx) = &ws_context {
            let data = RealtimeWorkoutData {
                user_id: "user_123".to_string(),
                exercise: "Squats".to_string(),
                rep_count: 15,
                form_score: 0.92,
                calories_burned: 45,
                heart_rate: Some(142),
            };
            
            if let Err(e) = ctx.service.send_workout_data(&data) {
                web_sys::console::log_1(&format!("Failed to send workout data: {:?}", e).into());
            }
        }
    };

    view! {
        <div class="bg-white/5 rounded-lg p-4 border border-white/10">
            <div class="flex items-center justify-between mb-4">
                <h4 class="font-medium">"Real-time Workout Sync"</h4>
                <WebSocketStatus/>
            </div>
            
            <div class="space-y-3">
                <button 
                    on:click=send_workout_update
                    class="bg-purple-600 hover:bg-purple-700 px-4 py-2 rounded-lg text-sm transition-colors"
                >
                    "Send Workout Data"
                </button>
                
                {move || {
                    if let Some(data) = workout_data.get() {
                        view! {
                            <div class="bg-white/10 rounded-lg p-3 text-sm">
                                <p>"Exercise: " {data.exercise}</p>
                                <p>"Reps: " {data.rep_count}</p>
                                <p>"Form Score: " {format!("{:.1}%", data.form_score * 100.0)}</p>
                                <p>"Calories: " {data.calories_burned}</p>
                                {data.heart_rate.map(|hr| view! {
                                    <p>"Heart Rate: " {hr} " bpm"</p>
                                })}
                            </div>
                        }.into()
                    } else {
                        view! {
                            <p class="text-white/60 text-sm">"No real-time data yet"</p>
                        }.into()
                    }
                }}
            </div>
        </div>
    }
}