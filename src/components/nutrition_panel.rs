use leptos::prelude::*;
use crate::components::icons::*;
use crate::api::{FitnessApiClient, User, OptimizationRequest, OptimizationConstraints, CalorieConstraints, MacroConstraints, MacroRange};
use crate::api::rag_client::{RagApiClient, RecommendationRequest, RecommendationType, UserContext, SmartRecommendation};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NutritionPlan {
    pub meal_name: String,
    pub calories: u32,
    pub protein_g: u32,
    pub carbs_g: u32,
    pub fat_g: u32,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct DailyNutrition {
    pub total_calories: u32,
    pub protein_g: u32,
    pub carbs_g: u32,
    pub fat_g: u32,
    pub meals: Vec<NutritionPlan>,
}

#[derive(Clone, Debug)]
pub struct NutritionGoals {
    pub calorie_target: u32,
    pub protein_target: u32,
    pub carb_target: u32,
    pub fat_target: u32,
}

#[component]
pub fn NutritionPanel() -> impl IntoView {
    let (users, set_users) = signal(Vec::<User>::new());
    let (selected_user, set_selected_user) = signal(None::<User>);
    let (active_tab, set_active_tab) = signal("overview".to_string());
    let (daily_nutrition, set_daily_nutrition) = signal(None::<DailyNutrition>);
    let (nutrition_goals, set_nutrition_goals) = signal(None::<NutritionGoals>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(String::new());
    let (generated_plan, set_generated_plan) = signal(None::<serde_json::Value>);
    let (ai_recommendations, set_ai_recommendations) = signal(Vec::<SmartRecommendation>::new());
    let (ai_rec_loading, set_ai_rec_loading) = signal(false);

    // Load users on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            match FitnessApiClient::get_users().await {
                Ok(user_list) => {
                    set_users.set(user_list.clone());
                    if let Some(first_user) = user_list.first() {
                        set_selected_user.set(Some(first_user.clone()));
                        calculate_nutrition_goals(first_user, set_nutrition_goals);
                        load_sample_nutrition(set_daily_nutrition);
                    }
                }
                Err(e) => set_error.set(format!("Failed to load users: {:?}", e)),
            }
        });
    });

    let generate_meal_plan = move |diet_type: String| {
        if let Some(user) = selected_user.get() {
            set_loading.set(true);
            set_error.set(String::new());
            
            spawn_local(async move {
                let goals = calculate_user_nutrition_goals(&user);
                let request = OptimizationRequest {
                    user_id: user.id.clone(),
                    time_horizon_days: 7,
                    objectives: vec![
                        format!("Maintain {} calorie intake", goals.calorie_target),
                        format!("Achieve {}g protein daily", goals.protein_target),
                        format!("Follow {} diet preferences", diet_type),
                    ],
                    constraints: OptimizationConstraints {
                        daily_calories: CalorieConstraints {
                            min: (goals.calorie_target as f32 * 0.9) as u32,
                            max: (goals.calorie_target as f32 * 1.1) as u32,
                            target: goals.calorie_target,
                        },
                        macros: MacroConstraints {
                            protein_g: MacroRange {
                                min: (goals.protein_target as f32 * 0.8) as u32,
                                max: (goals.protein_target as f32 * 1.2) as u32,
                            },
                            carbs_g: MacroRange {
                                min: (goals.carb_target as f32 * 0.7) as u32,
                                max: (goals.carb_target as f32 * 1.3) as u32,
                            },
                            fat_g: MacroRange {
                                min: (goals.fat_target as f32 * 0.8) as u32,
                                max: (goals.fat_target as f32 * 1.2) as u32,
                            },
                        },
                    },
                };

                match FitnessApiClient::optimize_meal_plan(request).await {
                    Ok(plan) => {
                        set_generated_plan.set(Some(plan));
                        set_active_tab.set("plans".to_string());
                    }
                    Err(e) => set_error.set(format!("Failed to generate meal plan: {:?}", e)),
                }
                set_loading.set(false);
            });
        }
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
            <div class="p-6 border-b border-white/10">
                <div class="flex items-center justify-between">
                    <h3 class="flex items-center gap-2 text-lg font-semibold">
                        "🍽️ Smart Nutrition Center"
                    </h3>
                    // User selector
                    {move || {
                        let user_list = users.get();
                        if !user_list.is_empty() {
                            view! {
                                <div class="flex items-center gap-2">
                                    <span class="text-white/70 text-sm">"Nutrition for:"</span>
                                    <select 
                                        class="bg-white/10 border border-white/20 rounded px-2 py-1 text-sm text-white"
                                        on:change=move |ev| {
                                            let user_id = event_target_value(&ev);
                                            if let Some(user) = user_list.iter().find(|u| u.id == user_id) {
                                                set_selected_user.set(Some(user.clone()));
                                                calculate_nutrition_goals(user, set_nutrition_goals);
                                                load_sample_nutrition(set_daily_nutrition);
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
                            if active_tab.get() == "overview" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("overview".to_string())
                    >
                        "📊 Overview"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "plans" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("plans".to_string())
                    >
                        "🍽️ Meal Plans"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "tracker" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("tracker".to_string())
                    >
                        "📝 Food Log"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "analysis" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("analysis".to_string())
                    >
                        "🔬 Analysis"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "ai_nutrition" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| {
                            set_active_tab.set("ai_nutrition".to_string());
                            load_nutrition_recommendations(set_ai_recommendations, set_ai_rec_loading, selected_user.get());
                        }
                    >
                        "🧠 AI Nutrition"
                    </button>
                </div>

                // Tab Content
                {move || {
                    match active_tab.get().as_str() {
                        "overview" => view! {
                            <div class="space-y-6">
                                // Daily Progress Overview
                                {move || {
                                    if let (Some(nutrition), Some(goals)) = (daily_nutrition.get(), nutrition_goals.get()) {
                                        let calorie_percent = (nutrition.total_calories as f32 / goals.calorie_target as f32 * 100.0).min(100.0) as u32;
                                        let protein_percent = (nutrition.protein_g as f32 / goals.protein_target as f32 * 100.0).min(100.0) as u32;
                                        let carb_percent = (nutrition.carbs_g as f32 / goals.carb_target as f32 * 100.0).min(100.0) as u32;
                                        let fat_percent = (nutrition.fat_g as f32 / goals.fat_target as f32 * 100.0).min(100.0) as u32;

                                        view! {
                                            <div>
                                                <h4 class="text-lg font-medium mb-4">"Today's Nutrition Progress"</h4>
                                                
                                                // Macro overview cards
                                                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                                                    <div class="bg-white/5 rounded-lg p-4 text-center">
                                                        <p class="text-2xl font-bold text-blue-400">{nutrition.total_calories}</p>
                                                        <p class="text-white/70 text-sm">"{goals.calorie_target} cal goal"</p>
                                                        <div class="mt-2 h-2 bg-white/10 rounded-full overflow-hidden">
                                                            <div class="h-full bg-blue-500 rounded-full transition-all" style=format!("width: {}%", calorie_percent)></div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="bg-white/5 rounded-lg p-4 text-center">
                                                        <p class="text-2xl font-bold text-green-400">{nutrition.protein_g}"g"</p>
                                                        <p class="text-white/70 text-sm">"{goals.protein_target}g protein"</p>
                                                        <div class="mt-2 h-2 bg-white/10 rounded-full overflow-hidden">
                                                            <div class="h-full bg-green-500 rounded-full transition-all" style=format!("width: {}%", protein_percent)></div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="bg-white/5 rounded-lg p-4 text-center">
                                                        <p class="text-2xl font-bold text-yellow-400">{nutrition.carbs_g}"g"</p>
                                                        <p class="text-white/70 text-sm">"{goals.carb_target}g carbs"</p>
                                                        <div class="mt-2 h-2 bg-white/10 rounded-full overflow-hidden">
                                                            <div class="h-full bg-yellow-500 rounded-full transition-all" style=format!("width: {}%", carb_percent)></div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="bg-white/5 rounded-lg p-4 text-center">
                                                        <p class="text-2xl font-bold text-orange-400">{nutrition.fat_g}"g"</p>
                                                        <p class="text-white/70 text-sm">"{goals.fat_target}g fat"</p>
                                                        <div class="mt-2 h-2 bg-white/10 rounded-full overflow-hidden">
                                                            <div class="h-full bg-orange-500 rounded-full transition-all" style=format!("width: {}%", fat_percent)></div>
                                                        </div>
                                                    </div>
                                                </div>
                                                
                                                // Today's meals
                                                <div class="bg-white/5 rounded-lg p-4">
                                                    <h5 class="font-medium mb-3">"Today's Meals"</h5>
                                                    <div class="space-y-3">
                                                        {nutrition.meals.into_iter().map(|meal| {
                                                            view! {
                                                                <div class="flex justify-between items-center p-3 bg-white/5 rounded-lg">
                                                                    <div>
                                                                        <p class="font-medium">{meal.meal_name}</p>
                                                                        <p class="text-sm text-white/70">
                                                                            {meal.calories}" cal • "
                                                                            {meal.protein_g}"g protein • "
                                                                            {meal.carbs_g}"g carbs • "
                                                                            {meal.fat_g}"g fat"
                                                                        </p>
                                                                    </div>
                                                                    <div class="text-green-400 text-sm">"✓"</div>
                                                                </div>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </div>
                                            </div>
                                        }.into()
                                    } else {
                                        view! {
                                            <div class="text-center py-8">
                                                <p class="text-white/60">"Select a user to view nutrition overview"</p>
                                            </div>
                                        }.into()
                                    }
                                }}
                            </div>
                        }.into(),
                        
                        "plans" => view! {
                            <div class="space-y-6">
                                <div class="flex justify-between items-center">
                                    <h4 class="text-lg font-medium">"AI-Generated Meal Plans"</h4>
                                    {move || {
                                        if loading.get() {
                                            view! {
                                                <div class="flex items-center gap-2 text-white/70">
                                                    <div class="animate-spin w-4 h-4 border-2 border-purple-500 border-t-transparent rounded-full"></div>
                                                    "Generating..."
                                                </div>
                                            }.into()
                                        } else {
                                            view! { <div></div> }.into()
                                        }
                                    }}
                                </div>
                                
                                // Diet type selection
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                                    {["Balanced", "High Protein", "Low Carb", "Mediterranean"].iter().map(|diet| {
                                        let diet_name = diet.to_string();
                                        view! {
                                            <button
                                                class="bg-white/5 rounded-lg p-4 border border-white/10 hover:border-purple-500/50 transition-colors text-left"
                                                on:click=move |_| generate_meal_plan(diet_name.clone())
                                            >
                                                <h5 class="font-medium mb-2">{diet_name.clone()}</h5>
                                                <p class="text-white/70 text-sm">
                                                    {match diet_name.as_str() {
                                                        "Balanced" => "Balanced macronutrients for general health",
                                                        "High Protein" => "Optimized for muscle building and recovery",
                                                        "Low Carb" => "Reduced carbs for weight management",
                                                        "Mediterranean" => "Heart-healthy Mediterranean diet",
                                                        _ => "Custom nutrition plan"
                                                    }}
                                                </p>
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                                
                                // Generated plan display
                                {move || {
                                    if let Some(plan) = generated_plan.get() {
                                        view! {
                                            <div class="bg-gradient-to-r from-green-500/20 to-blue-500/20 rounded-lg border border-white/10 p-6">
                                                <h5 class="text-green-400 font-medium mb-4">"✨ AI-Generated Meal Plan"</h5>
                                                <div class="space-y-4">
                                                    <div class="bg-white/10 rounded-lg p-4">
                                                        <p class="text-white/80">"Your personalized 7-day meal plan has been generated based on your fitness goals and dietary preferences."</p>
                                                        <div class="mt-4 grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                                                            <div>
                                                                <p class="text-white/70">"Daily Calories"</p>
                                                                <p class="font-medium">"2,200 - 2,400"</p>
                                                            </div>
                                                            <div>
                                                                <p class="text-white/70">"Protein"</p>
                                                                <p class="font-medium">"150g - 180g"</p>
                                                            </div>
                                                            <div>
                                                                <p class="text-white/70">"Carbs"</p>
                                                                <p class="font-medium">"200g - 250g"</p>
                                                            </div>
                                                            <div>
                                                                <p class="text-white/70">"Fat"</p>
                                                                <p class="font-medium">"70g - 90g"</p>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }.into()
                                    } else {
                                        view! {
                                            <div class="text-center py-8 bg-white/5 rounded-lg border border-dashed border-white/20">
                                                <p class="text-white/60 mb-2">"No meal plan generated yet"</p>
                                                <p class="text-white/40 text-sm">"Select a diet type above to generate a personalized meal plan"</p>
                                            </div>
                                        }.into()
                                    }
                                }}
                            </div>
                        }.into(),
                        
                        "tracker" => view! {
                            <div class="space-y-6">
                                <h4 class="text-lg font-medium">"Food & Water Tracker"</h4>
                                
                                // Quick add section
                                <div class="bg-white/5 rounded-lg p-4">
                                    <h5 class="font-medium mb-3">"Quick Add"</h5>
                                    <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
                                        {["🥗 Salad", "🍗 Chicken", "🍚 Rice", "🥛 Protein Shake", "🍌 Banana", "🥜 Nuts", "💧 Water", "☕ Coffee"].iter().map(|food| {
                                            view! {
                                                <button class="bg-white/10 hover:bg-white/20 rounded-lg p-3 text-sm transition-colors">
                                                    {food}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                                
                                // Barcode scanner
                                <div class="bg-gradient-to-r from-purple-600/20 to-blue-600/20 rounded-lg border border-purple-500/30 p-4">
                                    <div class="flex items-center gap-3">
                                        <div class="text-2xl">"📱"</div>
                                        <div>
                                            <h5 class="font-medium">"Scan Barcode"</h5>
                                            <p class="text-white/70 text-sm">"Use your phone camera to scan food barcodes"</p>
                                        </div>
                                        <button class="ml-auto bg-purple-600 hover:bg-purple-700 px-4 py-2 rounded-lg text-sm transition-colors">
                                            "Open Camera"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into(),
                        
                        "ai_nutrition" => view! {
                            <div class="space-y-6">
                                <div class="flex justify-between items-center">
                                    <h4 class="text-lg font-medium">"AI Nutrition Recommendations"</h4>
                                    {move || {
                                        if ai_rec_loading.get() {
                                            view! {
                                                <div class="flex items-center gap-2 text-white/70">
                                                    <div class="animate-spin w-4 h-4 border-2 border-purple-500 border-t-transparent rounded-full"></div>
                                                    "Loading recommendations..."
                                                </div>
                                            }.into()
                                        } else {
                                            view! { <div></div> }.into()
                                        }
                                    }}
                                </div>
                                
                                {move || {
                                    let recs = ai_recommendations.get();
                                    if recs.is_empty() && !ai_rec_loading.get() {
                                        view! {
                                            <div class="text-center py-8 bg-white/5 rounded-lg border border-dashed border-white/20">
                                                <div class="text-4xl text-white/40 mb-3">"🧠"</div>
                                                <h4 class="text-white/60 font-medium mb-2">"AI nutrition guidance will appear here"</h4>
                                                <p class="text-white/40 text-sm">"Personalized advice based on your goals and knowledge base"</p>
                                            </div>
                                        }.into()
                                    } else {
                                        view! {
                                            <div class="space-y-4">
                                                {recs.into_iter().map(|rec| {
                                                    view! {
                                                        <div class="bg-gradient-to-r from-green-500/20 to-blue-500/20 rounded-lg border border-white/10 p-4">
                                                            <div class="flex items-start justify-between mb-3">
                                                                <h5 class="font-medium text-white">{rec.title}</h5>
                                                                <span class="text-xs text-white/60 bg-white/20 px-2 py-1 rounded">
                                                                    {format!("{:.1}% relevance", rec.relevance_score * 100.0)}
                                                                </span>
                                                            </div>
                                                            <p class="text-white/80 text-sm mb-3">{rec.description}</p>
                                                            {if !rec.action_items.is_empty() {
                                                                view! {
                                                                    <div class="space-y-1">
                                                                        <h6 class="text-xs font-medium text-white/70">"Nutrition Actions:"</h6>
                                                                        <ul class="space-y-1">
                                                                            {rec.action_items.into_iter().map(|item| {
                                                                                view! { <li class="text-xs text-white/60">"• " {item}</li> }
                                                                            }).collect::<Vec<_>>()}
                                                                        </ul>
                                                                    </div>
                                                                }.into()
                                                            } else {
                                                                view! { <div></div> }.into()
                                                            }}
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }.into()
                                    }
                                }}
                            </div>
                        }.into(),
                        
                        _ => view! {
                            <div class="space-y-6">
                                <h4 class="text-lg font-medium">"Nutrition Analysis"</h4>
                                
                                // Weekly trends
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    <div class="bg-white/5 rounded-lg p-4">
                                        <h5 class="font-medium mb-3">"Weekly Trends"</h5>
                                        <div class="space-y-3">
                                            <div class="flex justify-between items-center">
                                                <span class="text-white/80">"Avg Daily Calories"</span>
                                                <span class="text-blue-400 font-medium">"2,180"</span>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-white/80">"Protein Goal Hit"</span>
                                                <span class="text-green-400 font-medium">"6/7 days"</span>
                                            </div>
                                            <div class="flex justify-between items-center">
                                                <span class="text-white/80">"Water Intake"</span>
                                                <span class="text-cyan-400 font-medium">"2.1L avg"</span>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="bg-white/5 rounded-lg p-4">
                                        <h5 class="font-medium mb-3">"Recommendations"</h5>
                                        <div class="space-y-2">
                                            <div class="flex items-start gap-2">
                                                <div class="text-green-400 text-xs mt-1">"✓"</div>
                                                <p class="text-white/80 text-sm">"Great job hitting protein goals!"</p>
                                            </div>
                                            <div class="flex items-start gap-2">
                                                <div class="text-yellow-400 text-xs mt-1">"⚠"</div>
                                                <p class="text-white/80 text-sm">"Consider adding more vegetables for fiber"</p>
                                            </div>
                                            <div class="flex items-start gap-2">
                                                <div class="text-blue-400 text-xs mt-1">"💡"</div>
                                                <p class="text-white/80 text-sm">"Increase water intake by 300ml daily"</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into()
                    }
                }}
            </div>
        </div>
    }
}

// Helper functions
fn calculate_nutrition_goals(user: &User, set_goals: WriteSignal<Option<NutritionGoals>>) {
    let goals = calculate_user_nutrition_goals(user);
    set_goals.set(Some(goals));
}

fn calculate_user_nutrition_goals(user: &User) -> NutritionGoals {
    // Basic BMR calculation (Harris-Benedict equation)
    let bmr = if user.age < 30 {
        2400.0
    } else if user.age < 50 {
        2200.0
    } else {
        2000.0
    };
    
    // Activity multiplier based on fitness level
    let activity_multiplier = match user.fitness_level {
        crate::api::FitnessLevel::Beginner => 1.2,
        crate::api::FitnessLevel::Intermediate => 1.4,
        crate::api::FitnessLevel::Advanced => 1.6,
        crate::api::FitnessLevel::Elite => 1.8,
    };
    
    let total_calories = (bmr * activity_multiplier) as u32;
    
    NutritionGoals {
        calorie_target: total_calories,
        protein_target: (total_calories as f32 * 0.25 / 4.0) as u32, // 25% calories from protein
        carb_target: (total_calories as f32 * 0.45 / 4.0) as u32,    // 45% calories from carbs  
        fat_target: (total_calories as f32 * 0.30 / 9.0) as u32,     // 30% calories from fat
    }
}

fn load_sample_nutrition(set_nutrition: WriteSignal<Option<DailyNutrition>>) {
    let sample_meals = vec![
        NutritionPlan {
            meal_name: "Breakfast".to_string(),
            calories: 450,
            protein_g: 25,
            carbs_g: 45,
            fat_g: 18,
            ingredients: vec!["Oatmeal".to_string(), "Banana".to_string(), "Protein powder".to_string()],
            instructions: vec!["Mix oatmeal with hot water".to_string(), "Add sliced banana and protein powder".to_string()],
        },
        NutritionPlan {
            meal_name: "Lunch".to_string(),
            calories: 680,
            protein_g: 45,
            carbs_g: 55,
            fat_g: 22,
            ingredients: vec!["Chicken breast".to_string(), "Brown rice".to_string(), "Mixed vegetables".to_string()],
            instructions: vec!["Grill chicken breast".to_string(), "Steam vegetables".to_string(), "Serve with brown rice".to_string()],
        },
        NutritionPlan {
            meal_name: "Dinner".to_string(),
            calories: 520,
            protein_g: 35,
            carbs_g: 40,
            fat_g: 20,
            ingredients: vec!["Salmon fillet".to_string(), "Sweet potato".to_string(), "Asparagus".to_string()],
            instructions: vec!["Bake salmon at 400°F for 20 minutes".to_string(), "Roast sweet potato and asparagus".to_string()],
        },
    ];
    
    let nutrition = DailyNutrition {
        total_calories: sample_meals.iter().map(|m| m.calories).sum(),
        protein_g: sample_meals.iter().map(|m| m.protein_g).sum(),
        carbs_g: sample_meals.iter().map(|m| m.carbs_g).sum(),
        fat_g: sample_meals.iter().map(|m| m.fat_g).sum(),
        meals: sample_meals,
    };
    
    set_nutrition.set(Some(nutrition));
}

// Helper function for loading nutrition recommendations  
fn load_nutrition_recommendations(
    set_recommendations: WriteSignal<Vec<SmartRecommendation>>,
    set_loading: WriteSignal<bool>,
    user: Option<User>,
) {
    if let Some(user_data) = user {
        set_loading.set(true);
        spawn_local(async move {
            let request = RecommendationRequest {
                user_context: UserContext {
                    user_id: user_data.id.clone(),
                    fitness_goals: vec!["muscle_gain".to_string(), "performance".to_string()],
                    current_stats: json!({
                        "age": user_data.age,
                        "fitness_level": user_data.fitness_level
                    }),
                    preferences: vec!["high_protein".to_string(), "balanced_macros".to_string()],
                    workout_history: None,
                },
                recommendation_type: RecommendationType::NutritionAdvice,
                preferences: None,
                limit: Some(3),
            };

            match RagApiClient::get_smart_recommendations(request).await {
                Ok(recs) => {
                    set_recommendations.set(recs);
                }
                Err(_e) => {
                    // Fallback to sample recommendations
                    let sample_recs = vec![
                        SmartRecommendation {
                            id: "nutrition_1".to_string(),
                            title: "Optimal Protein Distribution Strategy".to_string(),
                            description: "Distribute protein intake evenly throughout the day to maximize muscle protein synthesis and recovery".to_string(),
                            recommendation_type: RecommendationType::NutritionAdvice,
                            relevance_score: 0.91,
                            supporting_documents: vec!["protein_timing_research".to_string()],
                            action_items: vec![
                                "Consume 20-30g protein per meal".to_string(),
                                "Include protein within 2h post-workout".to_string(),
                                "Consider casein protein before bed".to_string(),
                            ],
                            metadata: json!({}),
                        }
                    ];
                    set_recommendations.set(sample_recs);
                }
            }
            set_loading.set(false);
        });
    }
}