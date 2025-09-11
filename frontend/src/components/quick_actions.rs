use leptos::prelude::*;
use crate::components::icons::*;

#[derive(Clone)]
struct ActionData {
    title: &'static str,
    subtitle: &'static str,
    gradient: &'static str,
}

#[component]
fn ActionButton(
    icon: impl IntoView,
    title: &'static str,
    subtitle: &'static str,
    gradient: &'static str,
    on_click: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| on_click()
            class=format!(
                "h-auto p-6 bg-gradient-to-br {} hover:scale-105 transition-all duration-200 border-0 rounded-lg",
                gradient
            )
        >
            <div class="flex flex-col items-center space-y-3 text-white">
                <div class="p-3 bg-white/20 rounded-xl backdrop-blur-sm">
                    {icon}
                </div>
                <div class="text-center">
                    <h4 class="text-sm font-medium">{title}</h4>
                    <p class="text-xs text-white/80">{subtitle}</p>
                </div>
            </div>
        </button>
    }
}

#[component]
pub fn QuickActions() -> impl IntoView {
    let (message, set_message) = signal(String::new());

    let actions = vec![
        ActionData {
            title: "Start Workout",
            subtitle: "Begin session",
            gradient: "from-purple-600 to-purple-800",
        },
        ActionData {
            title: "Form Check",
            subtitle: "AI analysis",
            gradient: "from-blue-600 to-blue-800",
        },
        ActionData {
            title: "Optimize Meals",
            subtitle: "Generate plan",
            gradient: "from-green-600 to-green-800",
        },
        ActionData {
            title: "View Progress",
            subtitle: "Analytics",
            gradient: "from-orange-600 to-orange-800",
        },
        ActionData {
            title: "Schedule",
            subtitle: "Plan workouts",
            gradient: "from-pink-600 to-pink-800",
        },
        ActionData {
            title: "Set Goals",
            subtitle: "Track targets",
            gradient: "from-indigo-600 to-indigo-800",
        },
    ];

    let create_action_handler = move |action_name: &'static str| {
        let set_message = set_message.clone();
        move || {
            set_message.set(format!("{} clicked!", action_name));
            // Clear message after 2 seconds
            let set_message = set_message.clone();
            set_timeout(
                move || set_message.set(String::new()),
                std::time::Duration::from_secs(2),
            );
        }
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg">
            <div class="p-6">
                <h3 class="text-lg text-white mb-6">"Quick Actions"</h3>
                
<Show when=move || !message.get().is_empty()>
                    <div class="mb-4 p-3 bg-blue-500/20 border border-blue-500/30 rounded-lg text-blue-300 text-sm">
                        {message.get()}
                    </div>
                </Show>
                
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
                    <ActionButton
                        icon=view! { <PlayIcon/> }
                        title=actions[0].title
                        subtitle=actions[0].subtitle
                        gradient=actions[0].gradient
                        on_click=create_action_handler("Start Workout")
                    />
                    <ActionButton
                        icon=view! { <CameraIcon/> }
                        title=actions[1].title
                        subtitle=actions[1].subtitle
                        gradient=actions[1].gradient
                        on_click=create_action_handler("Form Check")
                    />
                    <ActionButton
                        icon=view! { <UtensilsIcon/> }
                        title=actions[2].title
                        subtitle=actions[2].subtitle
                        gradient=actions[2].gradient
                        on_click=create_action_handler("Optimize Meals")
                    />
                    <ActionButton
                        icon=view! { 
                            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                            </svg>
                        }
                        title=actions[3].title
                        subtitle=actions[3].subtitle
                        gradient=actions[3].gradient
                        on_click=create_action_handler("View Progress")
                    />
                    <ActionButton
                        icon=view! { 
                            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                            </svg>
                        }
                        title=actions[4].title
                        subtitle=actions[4].subtitle
                        gradient=actions[4].gradient
                        on_click=create_action_handler("Schedule")
                    />
                    <ActionButton
                        icon=view! { <TargetIcon/> }
                        title=actions[5].title
                        subtitle=actions[5].subtitle
                        gradient=actions[5].gradient
                        on_click=create_action_handler("Set Goals")
                    />
                </div>
            </div>
        </div>
    }
}