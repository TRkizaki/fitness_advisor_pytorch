use leptos::prelude::*;
use crate::components::icons::*;

#[derive(Clone)]
struct MealPlan {
    meal: &'static str,
    items: Vec<&'static str>,
    calories: u32,
    protein: u32,
}

#[component]
pub fn MenuOptimization() -> impl IntoView {
    let (calories, set_calories) = signal(1960);
    let (protein, set_protein) = signal(133);
    let (carbs, set_carbs) = signal(180);
    let (fat, set_fat) = signal(70);
    let (generation, set_generation) = signal(23);
    let (fitness, set_fitness) = signal(94.2);

    let meal_plan = vec![
        MealPlan {
            meal: "Breakfast",
            items: vec!["Greek Yogurt with Berries", "Oatmeal with Almonds", "Green Tea"],
            calories: 420,
            protein: 28,
        },
        MealPlan {
            meal: "Lunch", 
            items: vec!["Grilled Chicken Salad", "Quinoa Bowl", "Avocado"],
            calories: 580,
            protein: 42,
        },
        MealPlan {
            meal: "Dinner",
            items: vec!["Salmon Fillet", "Sweet Potato", "Steamed Broccoli"],
            calories: 650,
            protein: 38,
        },
        MealPlan {
            meal: "Snacks",
            items: vec!["Protein Shake", "Mixed Nuts", "Apple"],
            calories: 310,
            protein: 25,
        },
    ];

    let optimize_meal_plan = move |_| {
        set_generation.update(|g| *g += 1);
        set_fitness.update(|f| *f = (*f + 0.5) % 100.0);
    };

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
            <div class="p-6 border-b border-white/10">
                <h3 class="flex items-center gap-2 text-lg font-semibold">
                    <BrainIcon/>
                    "AI Menu Optimization"
                </h3>
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
                            <div class="space-y-2">
                                <label class="text-white/80">{move || format!("Calorie Target: {} kcal", calories.get())}</label>
                                <input 
                                    type="range"
                                    min="1200" 
                                    max="3000" 
                                    step="50"
                                    prop:value=calories
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev).parse::<u32>().unwrap_or(1960);
                                        set_calories.set(value);
                                    }
                                    class="w-full h-2 bg-white/20 rounded-lg appearance-none cursor-pointer slider"
                                />
                            </div>
                            
                            <div class="space-y-2">
                                <label class="text-white/80">{move || format!("Protein Target: {}g", protein.get())}</label>
                                <input 
                                    type="range"
                                    min="80" 
                                    max="200" 
                                    step="5"
                                    prop:value=protein
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev).parse::<u32>().unwrap_or(133);
                                        set_protein.set(value);
                                    }
                                    class="w-full h-2 bg-white/20 rounded-lg appearance-none cursor-pointer slider"
                                />
                            </div>
                            
                            <div class="space-y-2">
                                <label class="text-white/80">{move || format!("Carb Limit: {}g", carbs.get())}</label>
                                <input 
                                    type="range"
                                    min="100" 
                                    max="300" 
                                    step="10"
                                    prop:value=carbs
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev).parse::<u32>().unwrap_or(180);
                                        set_carbs.set(value);
                                    }
                                    class="w-full h-2 bg-white/20 rounded-lg appearance-none cursor-pointer slider"
                                />
                            </div>
                            
                            <div class="space-y-2">
                                <label class="text-white/80">{move || format!("Fat Target: {}g", fat.get())}</label>
                                <input 
                                    type="range"
                                    min="40" 
                                    max="120" 
                                    step="5"
                                    prop:value=fat
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev).parse::<u32>().unwrap_or(70);
                                        set_fat.set(value);
                                    }
                                    class="w-full h-2 bg-white/20 rounded-lg appearance-none cursor-pointer slider"
                                />
                            </div>
                        </div>
                        
                        <div class="space-y-3">
                            <button 
                                on:click=optimize_meal_plan
                                class="w-full bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 px-4 py-2 rounded-lg transition-all"
                            >
                                <div class="flex items-center justify-center">
                                    <BrainIcon/>
                                    <span class="ml-2">"Optimize Meal Plan"</span>
                                </div>
                            </button>
                            <div class="flex gap-2">
                                <span class="bg-green-500/20 text-green-400 border border-green-500/30 px-2 py-1 rounded text-sm">
                                    {move || format!("Generation {}", generation.get())}
                                </span>
                                <span class="bg-blue-500/20 text-blue-400 border border-blue-500/30 px-2 py-1 rounded text-sm">
                                    {move || format!("Fitness: {:.1}%", fitness.get())}
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
                            {meal_plan.into_iter().map(|meal| {
                                view! {
                                    <div class="bg-gradient-to-r from-white/5 to-white/10 border border-white/20 rounded-lg">
                                        <div class="p-4">
                                            <div class="flex justify-between items-start mb-2">
                                                <h4 class="text-white">{meal.meal}</h4>
                                                <div class="flex gap-2">
                                                    <span class="border border-purple-500/50 text-purple-300 px-2 py-1 rounded text-xs">
                                                        {meal.calories}" kcal"
                                                    </span>
                                                    <span class="border border-blue-500/50 text-blue-300 px-2 py-1 rounded text-xs">
                                                        {meal.protein}"g protein"
                                                    </span>
                                                </div>
                                            </div>
                                            <div class="space-y-1">
                                                {meal.items.into_iter().map(|item| {
                                                    view! {
                                                        <p class="text-white/70 text-sm">"• "{item}</p>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                        
                        <div class="p-4 bg-gradient-to-r from-purple-500/20 to-blue-500/20 rounded-lg border border-white/10">
                            <div class="flex items-center gap-2 mb-2">
                                <svg class="h-4 w-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
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