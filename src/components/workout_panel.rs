use leptos::prelude::*;
use crate::components::icons::*;
use crate::api::{FitnessApiClient, User, FrameAnalysisResponse};

#[derive(Clone)]
struct FormMetric {
    name: &'static str,
    value: u32,
    color: &'static str,
}

#[derive(Clone, Debug)]
pub struct WorkoutSession {
    pub exercise: String,
    pub sets: u32,
    pub reps: u32,
    pub weight: f32,
    pub duration_minutes: u32,
}

#[derive(Clone, Debug)]
pub struct RealTimeMetrics {
    pub rep_count: u32,
    pub form_score: f32,
    pub exercise_detected: String,
    pub feedback: Vec<String>,
}

// Helper function for starting live analysis simulation
fn start_live_analysis(
    set_real_time_metrics: WriteSignal<Option<RealTimeMetrics>>,
    set_error: WriteSignal<String>,
) {
    spawn_local(async move {
        // Simulate analysis frames every 2 seconds
        for i in 0..10 {
            let metrics = RealTimeMetrics {
                rep_count: i * 2 + (js_sys::Math::random() * 3.0) as u32,
                form_score: 0.8 + (js_sys::Math::random() * 0.2) as f32,
                exercise_detected: "Squat".to_string(),
                feedback: vec![
                    "Keep your back straight".to_string(),
                    "Good depth".to_string(),
                ],
            };
            set_real_time_metrics.set(Some(metrics));
            
            // Wait 2 seconds
            gloo_timers::future::TimeoutFuture::new(2000).await;
        }
    });
}

#[component]
pub fn WorkoutPanel() -> impl IntoView {
    let (recording, set_recording) = signal(false);
    let (users, set_users) = signal(Vec::<User>::new());
    let (selected_user, set_selected_user) = signal(None::<User>);
    let (current_session, set_current_session) = signal(None::<WorkoutSession>);
    let (real_time_metrics, set_real_time_metrics) = signal(None::<RealTimeMetrics>);
    let (analysis_running, set_analysis_running) = signal(false);
    let (error, set_error) = signal(String::new());
    let (active_tab, set_active_tab) = signal("live".to_string());
    
    // Static form metrics for demo
    let metrics = vec![
        FormMetric { name: "Squat Depth", value: 92, color: "text-green-400" },
        FormMetric { name: "Knee Alignment", value: 78, color: "text-yellow-400" },
        FormMetric { name: "Back Posture", value: 88, color: "text-green-400" },
        FormMetric { name: "Rep Tempo", value: 85, color: "text-blue-400" },
    ];

    // Load users on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            match FitnessApiClient::get_users().await {
                Ok(user_list) => {
                    set_users.set(user_list.clone());
                    if let Some(first_user) = user_list.first() {
                        set_selected_user.set(Some(first_user.clone()));
                    }
                }
                Err(e) => set_error.set(format!("Failed to load users: {:?}", e)),
            }
        });
    });

    let toggle_recording = move |_| {
        set_recording.update(|r| *r = !*r);
        if recording.get() {
            set_analysis_running.set(true);
            start_live_analysis(set_real_time_metrics, set_error);
        } else {
            set_analysis_running.set(false);
        }
    };

    let start_workout = move |exercise: String| {
        if let Some(_user) = selected_user.get() {
            let session = WorkoutSession {
                exercise: exercise.clone(),
                sets: 0,
                reps: 0,
                weight: 0.0,
                duration_minutes: 0,
            };
            set_current_session.set(Some(session));
            set_recording.set(true);
            set_analysis_running.set(true);
            start_live_analysis(set_real_time_metrics, set_error);
        }
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
            <div class="p-6 border-b border-white/10">
                <div class="flex items-center justify-between">
                    <h3 class="flex items-center gap-2 text-lg font-semibold">
                        <CameraIcon/>
                        "Smart Workout Center"
                    </h3>
                    // User selector
                    {move || {
                        let user_list = users.get();
                        if !user_list.is_empty() {
                            view! {
                                <div class="flex items-center gap-2">
                                    <span class="text-white/70 text-sm">"Training:"</span>
                                    <select 
                                        class="bg-white/10 border border-white/20 rounded px-2 py-1 text-sm text-white"
                                        on:change=move |ev| {
                                            let user_id = event_target_value(&ev);
                                            if let Some(user) = user_list.iter().find(|u| u.id == user_id) {
                                                set_selected_user.set(Some(user.clone()));
                                            }
                                        }
                                    >
                                        {user_list.into_iter().map(|user| {
                                            view! {
                                                <option value={user.id.clone()}>{user.name.clone()}</option>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                </div>
                            }.into()
                        } else {
                            view! { <div></div> }.into()
                        }
                    }}
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
                            if active_tab.get() == "live" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("live".to_string())
                    >
                        "📹 Live Tracking"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "plans" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("plans".to_string())
                    >
                        "🏋️ Workout Plans"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "history" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("history".to_string())
                    >
                        "📊 Progress"
                    </button>
                </div>

                // Tab Content
                {move || {
                    match active_tab.get().as_str() {
                        "live" => view! {
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                // Camera Feed Area
                                <div class="space-y-4">
                                    <div class="aspect-video bg-gradient-to-br from-gray-800 to-gray-900 rounded-lg border border-white/20 flex items-center justify-center relative overflow-hidden">
                                        <div class="absolute inset-0 bg-gradient-to-br from-purple-500/20 to-blue-500/20"></div>
                                        <div class="relative z-10 text-center">
                                            <div class="h-12 w-12 text-white/60 mx-auto mb-2">
                                                <CameraIcon/>
                                            </div>
                                            <p class="text-white/80">{move || if recording.get() { "Live Analysis Active" } else { "Camera Ready" }}</p>
                                            <p class="text-white/60 text-sm">
                                                {move || if let Some(metrics) = real_time_metrics.get() {
                                                    format!("Reps: {} • Form Score: {:.1}%", metrics.rep_count, metrics.form_score * 100.0)
                                                } else {
                                                    "Click start to begin".to_string()
                                                }}
                                            </p>
                                        </div>
                                        {move || {
                                            if recording.get() {
                                                view! {
                                                    <div class="absolute top-4 right-4 flex gap-2">
                                                        <div class="w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
                                                        <span class="text-xs text-white/80">"LIVE"</span>
                                                    </div>
                                                }.into()
                                            } else {
                                                view! { <div></div> }.into()
                                            }
                                        }}
                                    </div>
                                    
                                    <div class="flex gap-2">
                                        <button 
                                            on:click=toggle_recording
                                            class=move || format!("flex-1 px-4 py-2 rounded-lg transition-all flex items-center justify-center gap-2 {}",
                                                if recording.get() { "bg-red-600 hover:bg-red-700" } else { "bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700" })
                                        >
                                            <PlayIcon/>
                                            {move || if recording.get() { "Stop Analysis" } else { "Start Analysis" }}
                                        </button>
                                        <button 
                                            class="border border-white/20 text-white hover:bg-white/10 p-2 rounded-lg transition-colors"
                                            on:click=move |_| start_workout("Squat".to_string())
                                        >
                                            "🏋️"
                                        </button>
                                    </div>
                                </div>
                                
                                // Form Analysis Metrics
                                <div class="space-y-4">
                                    <h4 class="text-lg font-medium">"Form Analysis"</h4>
                                    
                                    <div class="space-y-4">
                                        {metrics.into_iter().map(|metric| {
                                            view! {
                                                <div class="space-y-2">
                                                    <div class="flex justify-between">
                                                        <span class="text-white/80">{metric.name}</span>
                                                        <span class=metric.color>{metric.value}"%"</span>
                                                    </div>
                                                    <div class="h-2 bg-white/10 rounded-full overflow-hidden">
                                                        <div 
                                                            class="h-full bg-gradient-to-r from-purple-500 to-blue-500 rounded-full transition-all duration-300"
                                                            style=format!("width: {}%", metric.value)
                                                        ></div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                    
                                    // Real-time feedback
                                    {move || {
                                        if let Some(metrics) = real_time_metrics.get() {
                                            view! {
                                                <div class="mt-6 p-4 bg-gradient-to-r from-green-500/20 to-blue-500/20 rounded-lg border border-white/10">
                                                    <h4 class="text-green-400 mb-2">
                                                        "Exercise: " {metrics.exercise_detected}
                                                    </h4>
                                                    <div class="space-y-1">
                                                        {metrics.feedback.into_iter().map(|feedback| {
                                                            view! {
                                                                <p class="text-white/80 text-sm">
                                                                    "• " {feedback}
                                                                </p>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </div>
                                            }.into()
                                        } else {
                                            view! {
                                                <div class="mt-6 p-4 bg-white/5 rounded-lg border border-white/10">
                                                    <h4 class="text-white/70 mb-2">"Ready to Analyze"</h4>
                                                    <p class="text-white/60 text-sm">"Start recording to get real-time form feedback"</p>
                                                </div>
                                            }.into()
                                        }
                                    }}
                                </div>
                            </div>
                        }.into(),
                        
                        "plans" => view! {
                            <div class="space-y-4">
                                <h4 class="text-lg font-medium">"Workout Plans"</h4>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    {["Strength Training", "Cardio HIIT", "Flexibility", "Full Body"].iter().map(|plan| {
                                        let plan_name = plan.to_string();
                                        view! {
                                            <div class="bg-white/5 rounded-lg p-4 border border-white/10 hover:border-purple-500/50 transition-colors cursor-pointer"
                                                on:click=move |_| start_workout(plan_name.clone())>
                                                <h5 class="font-medium mb-2">{plan_name.clone()}</h5>
                                                <p class="text-white/70 text-sm">"45-60 minutes • Intermediate level"</p>
                                                <div class="mt-3 flex items-center gap-2">
                                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                                    <span class="text-xs text-white/60">"Available now"</span>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into(),
                        
                        _ => view! {
                            <div class="space-y-4">
                                <h4 class="text-lg font-medium">"Progress History"</h4>
                                {move || {
                                    if let Some(user) = selected_user.get() {
                                        view! {
                                            <div class="space-y-4">
                                                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                                    <div class="bg-white/5 rounded-lg p-4">
                                                        <p class="text-white/70 text-sm">"Workouts This Week"</p>
                                                        <p class="text-2xl font-bold text-purple-400">"5"</p>
                                                    </div>
                                                    <div class="bg-white/5 rounded-lg p-4">
                                                        <p class="text-white/70 text-sm">"Average Form Score"</p>
                                                        <p class="text-2xl font-bold text-green-400">"87%"</p>
                                                    </div>
                                                    <div class="bg-white/5 rounded-lg p-4">
                                                        <p class="text-white/70 text-sm">"Total Duration"</p>
                                                        <p class="text-2xl font-bold text-blue-400">"4.2h"</p>
                                                    </div>
                                                </div>
                                                <div class="bg-white/5 rounded-lg p-4">
                                                    <h5 class="font-medium mb-3">"Recent Sessions"</h5>
                                                    <div class="space-y-2">
                                                        {["Strength Training - Today", "Cardio HIIT - Yesterday", "Full Body - 2 days ago"].iter().map(|session| {
                                                            view! {
                                                                <div class="flex justify-between items-center py-2 border-b border-white/10 last:border-b-0">
                                                                    <span class="text-white/80">{session}</span>
                                                                    <span class="text-green-400 text-sm">"✓ Completed"</span>
                                                                </div>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </div>
                                            </div>
                                        }.into()
                                    } else {
                                        view! {
                                            <p class="text-white/60">"Select a user to view progress"</p>
                                        }.into()
                                    }
                                }}
                            </div>
                        }.into()
                    }
                }}
            </div>
        </div>
    }
}