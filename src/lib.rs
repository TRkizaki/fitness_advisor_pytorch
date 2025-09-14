use leptos::prelude::*;
use leptos_meta::*;
use leptos::mount::mount_to_body;

mod components;
mod api;
use components::{
    navigation::Navigation,
    stats_cards::StatsCards,
    workout_panel::WorkoutPanel,
    menu_optimization::MenuOptimization,
    progress_charts::ProgressCharts,
    quick_actions::QuickActions,
    api_test::ApiTest,
    nutrition_panel::NutritionPanel,
    knowledge_base_panel::KnowledgeBasePanel,
    mcp_server_panel::McpServerPanel,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    // Provide WebSocket context
    let ws_context = api::provide_websocket_context("ws://localhost:3000/api/ai/realtime");
    provide_context(ws_context);

    view! {
        <Html attr:lang="en"/>
        <Title text="Fitness Advisor Dashboard"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
            // Background Effects
            <div class="fixed inset-0">
                <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 via-blue-500/10 to-indigo-500/10"></div>
                <div class="absolute top-0 left-1/4 w-96 h-96 bg-purple-500/20 rounded-full blur-3xl"></div>
                <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-blue-500/20 rounded-full blur-3xl"></div>
            </div>
            
            // Main Content
            <div class="relative z-10">
                <Navigation/>
                
                <main class="max-w-7xl mx-auto p-6 space-y-8">
                    // Hero Section with Stats
                    <div class="space-y-6">
                        <div class="text-center space-y-2">
                            <h2 class="text-3xl text-white">"Welcome back, Alex!"</h2>
                            <p class="text-white/70">"Here's your fitness overview for today"</p>
                        </div>
                        <StatsCards/>
                    </div>
                    
                    // Backend API Integration Test
                    <ApiTest/>
                    
                    // Workout Tracking Panel
                    <WorkoutPanel/>
                    
                    // Smart Nutrition Center
                    <NutritionPanel/>
                    
                    // AI Knowledge Base
                    <KnowledgeBasePanel/>
                    
                    // MCP Server Integration
                    <McpServerPanel/>
                    
                    // Menu Optimization
                    <MenuOptimization/>
                    
                    // Progress Charts
                    <div class="space-y-6">
                        <h2 class="text-2xl text-white">"Progress Analytics"</h2>
                        <ProgressCharts/>
                    </div>
                    
                    // Real-time WebSocket Integration  
                    <api::RealtimeWorkoutTracker/>
                    
                    // Quick Actions
                    <QuickActions/>
                </main>
            </div>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}

#[cfg(not(feature = "hydrate"))]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}