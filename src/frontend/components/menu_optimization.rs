// src/frontend/components/menu_optimization.rs - Menu optimization UI

use leptos::prelude::*;

#[component]
pub fn MenuOptimization() -> impl IntoView {
    let (calorie_target, set_calorie_target) = create_signal(1960.0);
    let (protein_target, set_protein_target) = create_signal(133.0);
    let (carb_limit, set_carb_limit) = create_signal(180.0);
    let (fat_target, set_fat_target) = create_signal(70.0);
    let (is_optimizing, set_is_optimizing) = create_signal(false);

    let meal_plan = vec![
        MealDisplay {
            meal_type: "Breakfast".to_string(),
            items: vec!["Greek Yogurt with Berries".to_string(), "Oatmeal with Almonds".to_string(), "Green Tea".to_string()],
            calories: 420,
            protein: 28,
        },
        MealDisplay {
            meal_type: "Lunch".to_string(),
            items: vec!["Grilled Chicken Salad".to_string(), "Quinoa Bowl".to_string(), "Avocado".to_string()],
            calories: 580,
            protein: 42,
        },
        MealDisplay {
            meal_type: "Dinner".to_string(),
            items: vec!["Salmon Fillet".to_string(), "Sweet Potato".to_string(), "Steamed Broccoli".to_string()],
            calories: 650,
            protein: 38,
        },
        MealDisplay {
            meal_type: "Snacks".to_string(),
            items: vec!["Protein Shake".to_string(), "Mixed Nuts".to_string(), "Apple".to_string()],
            calories: 310,
            protein: 25,
        },
    ];

    let optimize_meal_plan = move |_| {
        set_is_optimizing(true);
        // TODO: Call Rust API for optimization
        spawn_local(async move {
            // Simulate optimization
            leptos::logging::log!("Starting meal plan optimization...");
            gloo_timers::future::TimeoutFuture::new(2000).await;
            set_is_optimizing(false);
        });
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 text-white rounded-lg">
            <div class="p-6 border-b border-white/10">
                <h2 class="text-xl flex items-center gap-2">
                    <BrainIcon/>
                    "AI Menu Optimization"
                </h2>
            </div>
            <div class="p-6 space-y-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    // Genetic Algorithm Controls
                    <div class="space-y-6">
                        <h3 class="text-lg flex items-center gap-2">
                            <ZapIcon/>
                            "Algorithm Controls"
                        </h3>
                        
                        <div class="space-y-4">
                            <SliderControl
                                label=format!("Calorie Target: {:.0} kcal", calorie_target.get())
                                value=calorie_target
                                set_value=set_calorie_target
                                min=1200.0
                                max=3000.0
                                step=50.0
                            />
                            
                            <SliderControl
                                label=format!("Protein Target: {:.0}g", protein_target.get())
                                value=protein_target
                                set_value=set_protein_target
                                min=80.0
                                max=200.0
                                step=5.0
                            />
                            
                            <SliderControl
                                label=format!("Carb Limit: {:.0}g", carb_limit.get())
                                value=carb_limit
                                set_value=set_carb_limit
                                min=100.0
                                max=300.0
                                step=10.0
                            />
                            
                            <SliderControl
                                label=format!("Fat Target: {:.0}g", fat_target.get())
                                value=fat_target
                                set_value=set_fat_target
                                min=40.0
                                max=120.0
                                step=5.0
                            />
                        </div>
                        
                        <div class="space-y-3">
                            <button 
                                class="w-full bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 text-white py-3 px-4 rounded-lg flex items-center justify-center gap-2 transition-all"
                                on:click=optimize_meal_plan
                                disabled=move || is_optimizing.get()
                            >
                                <BrainIcon/>
                                {move || if is_optimizing.get() { "Optimizing..." } else { "Optimize Meal Plan" }}
                            </button>
                            <div class="flex gap-2">
                                <span class="px-3 py-1 bg-green-500/20 text-green-400 border border-green-500/30 rounded-full text-sm">
                                    "Generation 23"
                                </span>
                                <span class="px-3 py-1 bg-blue-500/20 text-blue-400 border border-blue-500/30 rounded-full text-sm">
                                    "Fitness: 94.2%"
                                </span>
                            </div>
                        </div>
                    </div>
                    
                    // Generated Meal Plan
                    <div class="space-y-4">
                        <h3 class="text-lg flex items-center gap-2">
                            <UtensilsIcon/>
                            "Generated Meal Plan"
                        </h3>
                        
                        <div class="space-y-3">
                            <For
                                each=move || meal_plan.clone()
                                key=|meal| meal.meal_type.clone()
                                children=move |meal| {
                                    view! {
                                        <MealCard meal=meal/>
                                    }
                                }
                            />
                        </div>
                        
                        <div class="p-4 bg-gradient-to-r from-purple-500/20 to-blue-500/20 rounded-lg border border-white/10">
                            <div class="flex items-center gap-2 mb-2">
                                <TimerIcon/>
                                <span class="text-green-400">"Optimization Complete"</span>
                            </div>
                            <p class="text-white/80 text-sm">
                                "Total: 1,960 kcal, 133g protein, 45g fiber. Matches your goals perfectly!"
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SliderControl(
    label: String,
    value: ReadSignal<f64>,
    set_value: WriteSignal<f64>,
    min: f64,
    max: f64,
    step: f64,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <label class="text-white/80">{label}</label>
            <input
                type="range"
                class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer slider"
                min=min
                max=max
                step=step
                prop:value=move || value.get()
                on:input=move |ev| {
                    let new_value = event_target_value(&ev).parse().unwrap_or(value.get());
                    set_value(new_value);
                }
            />
        </div>
    }
}

#[component]
fn MealCard(meal: MealDisplay) -> impl IntoView {
    view! {
        <div class="bg-gradient-to-r from-white/5 to-white/10 border border-white/20 rounded-lg p-4">
            <div class="flex justify-between items-start mb-2">
                <h4 class="text-white">{meal.meal_type}</h4>
                <div class="flex gap-2">
                    <span class="px-2 py-1 border border-purple-500/50 text-purple-300 rounded text-xs">
                        {meal.calories}" kcal"
                    </span>
                    <span class="px-2 py-1 border border-blue-500/50 text-blue-300 rounded text-xs">
                        {meal.protein}"g protein"
                    </span>
                </div>
            </div>
            <div class="space-y-1">
                <For
                    each=move || meal.items.clone()
                    key=|item| item.clone()
                    children=move |item| {
                        view! {
                            <p class="text-white/70 text-sm">"• " {item}</p>
                        }
                    }
                />
            </div>
        </div>
    }
}

// Icons as components
#[component]
fn BrainIcon() -> impl IntoView {
    view! {
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"/>
        </svg>
    }
}

#[component]
fn ZapIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
        </svg>
    }
}

#[component]
fn UtensilsIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4m0 0L7 13m0 0l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17M17 13v6a2 2 0 01-2 2H9a2 2 0 01-2-2v-6.28"/>
        </svg>
    }
}

#[component]
fn TimerIcon() -> impl IntoView {
    view! {
        <svg class="h-4 w-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
    }
}

#[derive(Clone, Debug)]
pub struct MealDisplay {
    pub meal_type: String,
    pub items: Vec<String>,
    pub calories: u32,
    pub protein: u32,
}

#[derive(Clone, Debug)]
pub struct UserStats {
    pub bmi: f64,
    pub tdee: f64,
    pub fitness_level: String,
}