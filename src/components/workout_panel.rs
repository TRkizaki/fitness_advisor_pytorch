use leptos::prelude::*;
use crate::components::icons::*;

#[derive(Clone)]
struct FormMetric {
    name: &'static str,
    value: u32,
    color: &'static str,
}

#[component]
pub fn WorkoutPanel() -> impl IntoView {
    let (recording, set_recording) = signal(false);
    
    let metrics = vec![
        FormMetric { name: "Squat Depth", value: 92, color: "text-green-400" },
        FormMetric { name: "Knee Alignment", value: 78, color: "text-yellow-400" },
        FormMetric { name: "Back Posture", value: 88, color: "text-green-400" },
        FormMetric { name: "Rep Tempo", value: 85, color: "text-blue-400" },
    ];

    let toggle_recording = move |_| {
        set_recording.update(|r| *r = !*r);
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
            <div class="p-6 border-b border-white/10">
                <h3 class="flex items-center gap-2 text-lg font-semibold">
                    <CameraIcon/>
                    "Real-Time Workout Tracking"
                </h3>
            </div>
            <div class="p-6 space-y-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    // Camera Feed Area
                    <div class="space-y-4">
                        <div class="aspect-video bg-gradient-to-br from-gray-800 to-gray-900 rounded-lg border border-white/20 flex items-center justify-center relative overflow-hidden">
                            <div class="absolute inset-0 bg-gradient-to-br from-purple-500/20 to-blue-500/20"></div>
                            <div class="relative z-10 text-center">
                                <div class="h-12 w-12 text-white/60 mx-auto mb-2">
                                    <CameraIcon/>
                                </div>
                                <p class="text-white/80">"Camera Feed Active"</p>
                                <p class="text-white/60 text-sm">"AI Form Analysis Running"</p>
                            </div>
                            <div class="absolute top-4 right-4 flex gap-2">
                                <div class="w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
                                <span class="text-xs text-white/80">"LIVE"</span>
                            </div>
                        </div>
                        
                        <div class="flex gap-2">
                            <button 
                                on:click=toggle_recording
                                class="flex-1 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 px-4 py-2 rounded-lg transition-all"
                            >
                                <div class="flex items-center justify-center">
                                    <PlayIcon/>
                                    {move || if recording.get() { "Stop Recording" } else { "Start Recording" }}
                                </div>
                            </button>
                            <button class="border border-white/20 text-white hover:bg-white/10 p-2 rounded-lg transition-colors">
                                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-6.219-8.56"></path>
                                </svg>
                            </button>
                            <button class="border border-white/20 text-white hover:bg-white/10 p-2 rounded-lg transition-colors">
                                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                    
                    // Form Analysis Metrics
                    <div class="space-y-4">
                        <h3 class="text-lg">"Form Analysis"</h3>
                        
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
                        
                        <div class="mt-6 p-4 bg-gradient-to-r from-green-500/20 to-blue-500/20 rounded-lg border border-white/10">
                            <h4 class="text-green-400 mb-2">"Current Exercise: Squats"</h4>
                            <p class="text-white/80 text-sm">"Great form! Focus on knee tracking for optimal performance."</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}