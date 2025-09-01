// src/frontend/app.rs - Main Leptos application

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use crate::frontend::components::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/fitness_advisor_ai.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
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
                        <Routes>
                            <Route path="" view=Dashboard/>
                            <Route path="/dashboard" view=Dashboard/>
                        </Routes>
                    </main>
                </div>
            </div>
        </Router>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    view! {
        // Hero Section with Stats
        <div class="space-y-6">
            <div class="text-center space-y-2">
                <h2 class="text-3xl text-white">"Welcome back, Alex!"</h2>
                <p class="text-white/70">"Here's your fitness overview for today"</p>
            </div>
            <StatsCards/>
        </div>
        
        // Workout Tracking Panel
        <WorkoutPanel/>
        
        // Menu Optimization
        <MenuOptimization/>
        
        // Progress Charts
        <div class="space-y-6">
            <h2 class="text-2xl text-white">"Progress Analytics"</h2>
            <ProgressCharts/>
        </div>
        
        // Quick Actions
        <QuickActions/>
    }
}