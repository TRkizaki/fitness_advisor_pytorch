// src/frontend/components/navigation.rs - Top navigation bar

use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="bg-black/20 backdrop-blur-lg border-b border-white/10">
            <div class="max-w-7xl mx-auto px-6 py-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <h1 class="text-xl font-semibold text-white">"FitAdvisor Pro"</h1>
                    </div>
                    
                    <div class="flex items-center gap-4">
                        <button class="p-2 text-white/70 hover:text-white transition-colors">
                            <BellIcon/>
                        </button>
                        <button class="p-2 text-white/70 hover:text-white transition-colors">
                            <SettingsIcon/>
                        </button>
                        <div class="w-8 h-8 bg-gradient-to-br from-purple-500 to-blue-500 rounded-full flex items-center justify-center text-white text-sm font-medium">
                            "A"
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn BellIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-5 5v-5zM11.613 15.931c-.631-.634-1.613-.634-2.244 0l-4.947 4.947A1 1 0 005 22h14a1 1 0 00.707-1.707l-8.094-8.362z"/>
        </svg>
    }
}

#[component]
fn SettingsIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
        </svg>
    }
}