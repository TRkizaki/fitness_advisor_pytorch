use leptos::prelude::*;
use crate::api::{FitnessApiClient, User};
use crate::components::icons::*;

#[component]
pub fn ApiTest() -> impl IntoView {
    let (health_status, set_health_status) = signal(String::new());
    let (users, set_users) = signal(Vec::<User>::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(String::new());
    let (ml_status, set_ml_status) = signal(false);

    // Test backend connection
    let test_connection = move |_| {
        set_loading.set(true);
        set_error.set(String::new());
        
        spawn_local(async move {
            match FitnessApiClient::check_health().await {
                Ok(health) => {
                    set_health_status.set(health);
                    
                    // Also fetch users
                    match FitnessApiClient::get_users().await {
                        Ok(user_list) => set_users.set(user_list),
                        Err(e) => set_error.set(format!("Failed to fetch users: {:?}", e)),
                    }
                    
                    // Check ML service
                    match FitnessApiClient::check_ml_service_status().await {
                        Ok(status) => set_ml_status.set(status),
                        Err(_) => set_ml_status.set(false),
                    }
                }
                Err(e) => set_error.set(format!("Failed to connect to backend: {:?}", e)),
            }
            set_loading.set(false);
        });
    };

    // Test on component mount
    Effect::new(move |_| {
        test_connection(());
    });

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
            <div class="p-6 border-b border-white/10">
                <h3 class="flex items-center gap-2 text-lg font-semibold">
                    <LinkIcon/>
                    "Backend API Test"
                </h3>
            </div>
            <div class="p-6 space-y-4">
                <button 
                    on:click=test_connection
                    disabled=move || loading.get()
                    class="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 px-4 py-2 rounded-lg transition-colors"
                >
                    {move || if loading.get() { "Testing..." } else { "Test Backend Connection" }}
                </button>

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

                // Health status
                {move || {
                    let health = health_status.get();
                    if !health.is_empty() {
                        view! {
                            <div class="bg-green-600/20 border border-green-500/30 rounded-lg p-3">
                                <p class="text-green-300">"✅ Backend Health: " {health}</p>
                            </div>
                        }.into()
                    } else {
                        view! { <div></div> }.into()
                    }
                }}

                // ML Service status
                <div class="bg-blue-600/20 border border-blue-500/30 rounded-lg p-3">
                    <p class="text-blue-300">
                        {move || {
                            if ml_status.get() {
                                "🤖 ML Service: Online"
                            } else {
                                "🤖 ML Service: Offline"
                            }
                        }}
                    </p>
                </div>

                // Users display
                {move || {
                    let user_list = users.get();
                    if !user_list.is_empty() {
                        view! {
                            <div>
                                <h4 class="text-md font-medium mb-2">
                                    "👥 Users in Database (" {user_list.len()} ")"
                                </h4>
                                <div class="space-y-2">
                                    {user_list.into_iter().map(|user| {
                                        let goals_str = user.goals.join(", ");
                                        let fitness_level = crate::api::utils::format_fitness_level(&user.fitness_level);
                                        
                                        view! {
                                            <div class="bg-white/5 rounded-lg p-3">
                                                <p class="font-medium">{user.name.clone()}</p>
                                                <p class="text-sm text-white/70">
                                                    {user.age} "yo • " {fitness_level} " • " {goals_str}
                                                </p>
                                                <p class="text-xs text-white/60">
                                                    {user.height} "cm, " {user.weight} "kg"
                                                </p>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into()
                    } else {
                        view! { <div></div> }.into()
                    }
                }}

                // Connection status indicators
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="bg-white/5 rounded-lg p-3 text-center">
                        <div class="text-2xl mb-1">
                            {move || if !health_status.get().is_empty() { "🟢" } else { "🔴" }}
                        </div>
                        <p class="text-sm text-white/70">"Backend API"</p>
                    </div>
                    
                    <div class="bg-white/5 rounded-lg p-3 text-center">
                        <div class="text-2xl mb-1">
                            {move || if ml_status.get() { "🟢" } else { "🔴" }}
                        </div>
                        <p class="text-sm text-white/70">"ML Service"</p>
                    </div>
                    
                    <div class="bg-white/5 rounded-lg p-3 text-center">
                        <div class="text-2xl mb-1">"🟡"</div>
                        <p class="text-sm text-white/70">"WebSocket"</p>
                    </div>
                </div>

                // Performance metrics
                <div class="bg-gradient-to-r from-purple-600/10 to-blue-600/10 border border-purple-500/30 rounded-lg p-4">
                    <h4 class="text-purple-300 mb-3">"Performance Metrics"</h4>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                        <div>
                            <p class="text-white/60">"Response Time"</p>
                            <p class="text-white font-medium">"< 100ms"</p>
                        </div>
                        <div>
                            <p class="text-white/60">"Bundle Size"</p>
                            <p class="text-white font-medium">"~45KB"</p>
                        </div>
                        <div>
                            <p class="text-white/60">"Memory Usage"</p>
                            <p class="text-white font-medium">"~2.1MB"</p>
                        </div>
                        <div>
                            <p class="text-white/60">"Framework"</p>
                            <p class="text-white font-medium">"Leptos + WASM"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}