// src/frontend/components/workout_panel.rs - Real-time workout tracking

use leptos::prelude::*;
use web_sys::WebSocket;

#[component]
pub fn WorkoutPanel() -> impl IntoView {
    let (ws_status, set_ws_status) = create_signal("Disconnected".to_string());
    let (form_metrics, set_form_metrics) = create_signal(FormMetrics::default());
    let (current_exercise, set_current_exercise) = create_signal("Squats".to_string());
    let (is_recording, set_is_recording) = create_signal(false);

    let start_recording = move |_| {
        set_is_recording(true);
        set_ws_status("Connecting...".to_string());
        
        spawn_local(async move {
            // Connect to WebSocket
            let ws = WebSocket::new("ws://localhost:3000/api/ai/realtime").unwrap();
            set_ws_status("Connected".to_string());
            leptos::logging::log!("WebSocket connected for real-time analysis");
        });
    };

    let stop_recording = move |_| {
        set_is_recording(false);
        set_ws_status("Disconnected".to_string());
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 text-white rounded-lg">
            <div class="p-6 border-b border-white/10">
                <h2 class="text-xl flex items-center gap-2">
                    <CameraIcon/>
                    "Real-Time Workout Tracking"
                </h2>
            </div>
            <div class="p-6 space-y-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    // Camera Feed Area
                    <div class="space-y-4">
                        <div class="aspect-video bg-gradient-to-br from-gray-800 to-gray-900 rounded-lg border border-white/20 flex items-center justify-center relative overflow-hidden">
                            <div class="absolute inset-0 bg-gradient-to-br from-purple-500/20 to-blue-500/20"></div>
                            <div class="relative z-10 text-center">
                                <CameraIcon/>
                                <p class="text-white/80 mt-2">"Camera Feed Active"</p>
                                <p class="text-white/60 text-sm">"AI Form Analysis Running"</p>
                            </div>
                            <div class="absolute top-4 right-4 flex gap-2">
                                <div class="w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
                                <span class="text-xs text-white/80">"LIVE"</span>
                            </div>
                        </div>
                        
                        <div class="flex gap-2">
                            <button 
                                class="flex-1 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 text-white py-2 px-4 rounded-lg flex items-center justify-center gap-2 transition-all"
                                on:click=start_recording
                                disabled=move || is_recording.get()
                            >
                                <PlayIcon/>
                                {move || if is_recording.get() { "Recording..." } else { "Start Recording" }}
                            </button>
                            <button 
                                class="border border-white/20 text-white hover:bg-white/10 py-2 px-4 rounded-lg transition-all"
                                on:click=stop_recording
                            >
                                <SquareIcon/>
                            </button>
                        </div>
                    </div>
                    
                    // Form Analysis Metrics
                    <div class="space-y-4">
                        <h3 class="text-lg">"Form Analysis"</h3>
                        
                        <div class="space-y-4">
                            <MetricProgress 
                                label="Squat Depth"
                                value=move || form_metrics.get().squat_depth
                                color="green"
                            />
                            <MetricProgress 
                                label="Knee Alignment"
                                value=move || form_metrics.get().knee_alignment
                                color="yellow"
                            />
                            <MetricProgress 
                                label="Back Posture"
                                value=move || form_metrics.get().back_posture
                                color="green"
                            />
                            <MetricProgress 
                                label="Rep Tempo"
                                value=move || form_metrics.get().rep_tempo
                                color="blue"
                            />
                        </div>
                        
                        <div class="mt-6 p-4 bg-gradient-to-r from-green-500/20 to-blue-500/20 rounded-lg border border-white/10">
                            <h4 class="text-green-400 mb-2">
                                "Current Exercise: " {move || current_exercise.get()}
                            </h4>
                            <p class="text-white/80 text-sm">
                                "Great form! Focus on knee tracking for optimal performance."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MetricProgress(
    label: &'static str,
    value: impl Fn() -> u32 + 'static,
    color: &'static str,
) -> impl IntoView {
    let color_class = match color {
        "green" => "text-green-400",
        "yellow" => "text-yellow-400",
        "blue" => "text-blue-400",
        _ => "text-white",
    };

    view! {
        <div class="space-y-2">
            <div class="flex justify-between">
                <span class="text-white/80">{label}</span>
                <span class=color_class>{move || format!("{}%", value())}</span>
            </div>
            <div class="w-full bg-gray-700 rounded-full h-2">
                <div 
                    class=format!("bg-gradient-to-r from-{}-500 to-{}-600 h-2 rounded-full transition-all duration-300", color, color)
                    style=move || format!("width: {}%", value())
                ></div>
            </div>
        </div>
    }
}

// Icons
#[component]
fn CameraIcon() -> impl IntoView {
    view! {
        <svg class="h-12 w-12 text-white/60 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"/>
        </svg>
    }
}

#[component]
fn PlayIcon() -> impl IntoView {
    view! {
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1.586a1 1 0 01.707.293l2.414 2.414a1 1 0 00.707.293H15"/>
        </svg>
    }
}

#[component]
fn SquareIcon() -> impl IntoView {
    view! {
        <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
            <rect width="18" height="18" x="3" y="3" rx="2"/>
        </svg>
    }
}

#[derive(Clone, Debug)]
pub struct FormMetrics {
    pub squat_depth: u32,
    pub knee_alignment: u32,
    pub back_posture: u32,
    pub rep_tempo: u32,
}

impl Default for FormMetrics {
    fn default() -> Self {
        Self {
            squat_depth: 92,
            knee_alignment: 78,
            back_posture: 88,
            rep_tempo: 85,
        }
    }
}