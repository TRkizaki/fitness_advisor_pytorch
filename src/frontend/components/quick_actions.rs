// src/frontend/components/quick_actions.rs - Quick action buttons

use leptos::prelude::*;

#[component]
pub fn QuickActions() -> impl IntoView {
    let actions = vec![
        ActionButton {
            label: "Start Workout".to_string(),
            subtitle: "Begin session".to_string(),
            color: "purple".to_string(),
        },
        ActionButton {
            label: "Form Check".to_string(),
            subtitle: "AI analysis".to_string(),
            color: "blue".to_string(),
        },
        ActionButton {
            label: "Optimize Meals".to_string(),
            subtitle: "Generate plan".to_string(),
            color: "green".to_string(),
        },
        ActionButton {
            label: "View Progress".to_string(),
            subtitle: "Analytics".to_string(),
            color: "orange".to_string(),
        },
        ActionButton {
            label: "Schedule".to_string(),
            subtitle: "Plan workouts".to_string(),
            color: "pink".to_string(),
        },
        ActionButton {
            label: "Set Goals".to_string(),
            subtitle: "Track targets".to_string(),
            color: "indigo".to_string(),
        },
    ];

    view! {
        <div class="space-y-4">
            <h2 class="text-2xl text-white">"Quick Actions"</h2>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
                <For
                    each=move || actions.clone()
                    key=|action| action.label.clone()
                    children=move |action| {
                        let action_clone = action.clone();
                        view! {
                            <ActionCard action=action_clone/>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn ActionCard(action: ActionButton) -> impl IntoView {
    let color_classes = match action.color.as_str() {
        "purple" => "from-purple-600 to-purple-700 hover:from-purple-700 hover:to-purple-800",
        "blue" => "from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800",
        "green" => "from-green-600 to-green-700 hover:from-green-700 hover:to-green-800",
        "orange" => "from-orange-600 to-orange-700 hover:from-orange-700 hover:to-orange-800",
        "pink" => "from-pink-600 to-pink-700 hover:from-pink-700 hover:to-pink-800",
        "indigo" => "from-indigo-600 to-indigo-700 hover:from-indigo-700 hover:to-indigo-800",
        _ => "from-gray-600 to-gray-700 hover:from-gray-700 hover:to-gray-800",
    };

    view! {
        <button class=format!("bg-gradient-to-br {} text-white p-4 rounded-lg transition-all duration-200 transform hover:scale-105 flex flex-col items-center justify-center space-y-2 min-h-[100px]", color_classes)>
            <div class="text-2xl">
                {match action.color.as_str() {
                    "purple" => view! { <PlayIcon/> }.into_view(),
                    "blue" => view! { <CameraIcon/> }.into_view(),
                    "green" => view! { <UtensilsIcon/> }.into_view(),
                    "orange" => view! { <ChartIcon/> }.into_view(),
                    "pink" => view! { <CalendarIcon/> }.into_view(),
                    "indigo" => view! { <TargetIcon/> }.into_view(),
                    _ => view! { <PlayIcon/> }.into_view(),
                }}
            </div>
            <div class="text-center">
                <p class="font-medium text-sm">{action.label}</p>
                <p class="text-xs opacity-70">{action.subtitle}</p>
            </div>
        </button>
    }
}

// Icon components
#[component]
fn PlayIcon() -> impl IntoView {
    view! {
        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1.586a1 1 0 01.707.293l2.414 2.414a1 1 0 00.707.293H15"/>
        </svg>
    }
}

#[component]
fn CameraIcon() -> impl IntoView {
    view! {
        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
        </svg>
    }
}

#[component]
fn UtensilsIcon() -> impl IntoView {
    view! {
        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4m0 0L7 13m0 0l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17M17 13v6a2 2 0 01-2 2H9a2 2 0 01-2-2v-6.28"/>
        </svg>
    }
}

#[component]
fn ChartIcon() -> impl IntoView {
    view! {
        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
        </svg>
    }
}

#[component]
fn CalendarIcon() -> impl IntoView {
    view! {
        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
        </svg>
    }
}

#[component]
fn TargetIcon() -> impl IntoView {
    view! {
        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"/>
        </svg>
    }
}

#[derive(Clone, Debug)]
pub struct ActionButton {
    pub label: String,
    pub subtitle: String,
    pub color: String,
}