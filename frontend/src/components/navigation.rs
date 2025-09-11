use leptos::prelude::*;
use crate::components::icons::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="w-full bg-black/20 backdrop-blur-lg border-b border-white/10 p-4">
            <div class="max-w-7xl mx-auto flex items-center justify-between">
                <div class="flex items-center space-x-4">
                    <button class="text-white hover:bg-white/20 p-2 rounded-lg transition-colors">
                        <MenuIcon/>
                    </button>
                    <h1 class="text-xl text-white">"FitAdvisor Pro"</h1>
                </div>
                
                <div class="flex items-center space-x-4">
                    <button class="text-white hover:bg-white/20 p-2 rounded-lg transition-colors">
                        <BellIcon/>
                    </button>
                    <button class="text-white hover:bg-white/20 p-2 rounded-lg transition-colors">
                        <SettingsIcon/>
                    </button>
                    <div class="h-8 w-8 bg-gradient-to-br from-purple-500 to-blue-500 text-white rounded-full flex items-center justify-center">
                        <UserIcon/>
                    </div>
                </div>
            </div>
        </nav>
    }
}