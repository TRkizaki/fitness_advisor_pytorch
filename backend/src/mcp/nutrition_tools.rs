use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use uuid::Uuid;

use crate::mcp::types::{CallToolResult, ToolResponseContent, UserProfile, NutritionPlan, Meal, MealType, MacronutrientSplit};

#[derive(Debug, Deserialize)]
pub struct NutritionPlanRequest {
    pub user_profile: UserProfile,
    pub calorie_target: Option<u32>,
    pub meal_preferences: Option<MealPreferences>,
}

#[derive(Debug, Deserialize)]
pub struct MealPreferences {
    pub meals_per_day: Option<u32>,
    pub prep_time_minutes: Option<u32>,
    pub cuisine_preferences: Option<Vec<String>>,
    pub avoid_ingredients: Option<Vec<String>>,
    pub macro_split: Option<MacronutrientSplit>,
}

#[derive(Debug, Deserialize)]
pub struct NutritionAnalysisRequest {
    pub foods: Vec<FoodItem>,
    pub analysis_type: Option<String>, // "basic", "micronutrients", "interactions", "timing"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodItem {
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    pub meal_timing: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionAnalysis {
    pub total_calories: f32,
    pub macronutrients: MacronutrientBreakdown,
    pub micronutrients: Option<MicronutrientAnalysis>,
    pub nutrient_interactions: Option<Vec<NutrientInteraction>>,
    pub timing_recommendations: Option<TimingRecommendations>,
    pub deficiency_risks: Vec<DeficiencyRisk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MacronutrientBreakdown {
    pub protein_grams: f32,
    pub carbohydrate_grams: f32,
    pub fat_grams: f32,
    pub fiber_grams: f32,
    pub protein_percent: f32,
    pub carb_percent: f32,
    pub fat_percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MicronutrientAnalysis {
    pub vitamins: HashMap<String, MicronutrientInfo>,
    pub minerals: HashMap<String, MicronutrientInfo>,
    pub overall_score: f32, // 0-100 score for micronutrient adequacy
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MicronutrientInfo {
    pub amount: f32,
    pub unit: String,
    pub daily_value_percent: f32,
    pub adequacy_status: String, // "deficient", "adequate", "optimal", "excessive"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutrientInteraction {
    pub nutrients: Vec<String>,
    pub interaction_type: String, // "synergistic", "antagonistic", "neutral"
    pub effect: String,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimingRecommendations {
    pub pre_workout: Vec<String>,
    pub post_workout: Vec<String>,
    pub morning: Vec<String>,
    pub evening: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeficiencyRisk {
    pub nutrient: String,
    pub risk_level: String, // "low", "moderate", "high"
    pub symptoms: Vec<String>,
    pub food_sources: Vec<String>,
}

pub struct NutritionTools {
    // Comprehensive nutrition database
    nutrition_database: NutritionDatabase,
}

impl NutritionTools {
    pub fn new() -> Self {
        Self {
            nutrition_database: NutritionDatabase::new(),
        }
    }

    // Create personalized nutrition plan
    pub async fn create_nutrition_plan(&self, args: Option<Value>) -> Result<CallToolResult> {
        let request: NutritionPlanRequest = match args {
            Some(args) => serde_json::from_value(args)?,
            None => return Ok(CallToolResult {
                content: vec![ToolResponseContent::Text {
                    text: "Error: No nutrition plan parameters provided".to_string(),
                }],
                is_error: Some(true),
            }),
        };

        let nutrition_plan = self.generate_nutrition_plan(&request).await?;
        
        let response_text = self.format_nutrition_plan_response(&nutrition_plan);

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: response_text,
            }],
            is_error: None,
        })
    }

    async fn generate_nutrition_plan(&self, request: &NutritionPlanRequest) -> Result<NutritionPlan> {
        let daily_calories = request.calorie_target
            .unwrap_or_else(|| self.calculate_daily_calories(&request.user_profile));

        let macro_split = request.meal_preferences
            .as_ref()
            .and_then(|prefs| prefs.macro_split.clone())
            .unwrap_or_else(|| self.determine_macro_split(&request.user_profile));

        let meals_per_day = request.meal_preferences
            .as_ref()
            .and_then(|prefs| prefs.meals_per_day)
            .unwrap_or(4); // 3 main meals + 1 snack

        let meals = self.generate_meals(
            daily_calories,
            &macro_split,
            meals_per_day,
            &request.user_profile,
            request.meal_preferences.as_ref(),
        );

        Ok(NutritionPlan {
            id: Uuid::new_v4(),
            name: format!("Personalized {} Nutrition Plan", 
                request.user_profile.fitness_goals.iter()
                    .map(|g| format!("{:?}", g))
                    .collect::<Vec<_>>()
                    .join(" & ")
                    .replace('_', " ")
            ),
            description: format!(
                "A {} calorie nutrition plan designed for {} goals with {}% protein, {}% carbs, and {}% fats.",
                daily_calories,
                request.user_profile.fitness_goals.iter()
                    .map(|g| format!("{:?}", g).to_lowercase().replace('_', " "))
                    .collect::<Vec<_>>()
                    .join(" and "),
                macro_split.protein_percent,
                macro_split.carbohydrate_percent,
                macro_split.fat_percent
            ),
            daily_calories,
            macronutrient_split: macro_split,
            meals,
            dietary_restrictions: request.user_profile.dietary_restrictions.clone(),
        })
    }

    fn calculate_daily_calories(&self, profile: &UserProfile) -> u32 {
        // Calculate BMR using Mifflin-St Jeor equation
        let bmr = match profile.gender {
            crate::mcp::types::Gender::Male => {
                (10.0 * profile.weight_kg) + (6.25 * profile.height_cm as f32) - (5.0 * profile.age as f32) + 5.0
            }
            _ => {
                (10.0 * profile.weight_kg) + (6.25 * profile.height_cm as f32) - (5.0 * profile.age as f32) - 161.0
            }
        };

        // Apply activity factor
        let activity_factor = match profile.activity_level {
            crate::mcp::types::ActivityLevel::Sedentary => 1.2,
            crate::mcp::types::ActivityLevel::LightlyActive => 1.375,
            crate::mcp::types::ActivityLevel::ModeratelyActive => 1.55,
            crate::mcp::types::ActivityLevel::VeryActive => 1.725,
            crate::mcp::types::ActivityLevel::SuperActive => 1.9,
        };

        let tdee = bmr * activity_factor;

        // Adjust based on fitness goals
        let calorie_adjustment = match profile.fitness_goals.first() {
            Some(crate::mcp::types::FitnessGoal::WeightLoss) => 0.85, // 15% deficit
            Some(crate::mcp::types::FitnessGoal::MuscleGain) => 1.15, // 15% surplus
            Some(crate::mcp::types::FitnessGoal::BodyRecomposition) => 1.0,
            _ => 1.0,
        };

        (tdee * calorie_adjustment) as u32
    }

    fn determine_macro_split(&self, profile: &UserProfile) -> MacronutrientSplit {
        match profile.fitness_goals.first() {
            Some(crate::mcp::types::FitnessGoal::MuscleGain) | 
            Some(crate::mcp::types::FitnessGoal::StrengthGain) => {
                MacronutrientSplit {
                    protein_percent: 30.0,
                    carbohydrate_percent: 40.0,
                    fat_percent: 30.0,
                }
            }
            Some(crate::mcp::types::FitnessGoal::WeightLoss) => {
                MacronutrientSplit {
                    protein_percent: 35.0,
                    carbohydrate_percent: 30.0,
                    fat_percent: 35.0,
                }
            }
            Some(crate::mcp::types::FitnessGoal::Endurance) => {
                MacronutrientSplit {
                    protein_percent: 20.0,
                    carbohydrate_percent: 55.0,
                    fat_percent: 25.0,
                }
            }
            _ => {
                MacronutrientSplit {
                    protein_percent: 25.0,
                    carbohydrate_percent: 45.0,
                    fat_percent: 30.0,
                }
            }
        }
    }

    fn generate_meals(&self, daily_calories: u32, macro_split: &MacronutrientSplit, meals_per_day: u32, profile: &UserProfile, preferences: Option<&MealPreferences>) -> Vec<Meal> {
        let calorie_per_meal = daily_calories / meals_per_day;
        let protein_grams_per_meal = (daily_calories as f32 * macro_split.protein_percent / 100.0) / 4.0 / meals_per_day as f32;
        let carb_grams_per_meal = (daily_calories as f32 * macro_split.carbohydrate_percent / 100.0) / 4.0 / meals_per_day as f32;
        let fat_grams_per_meal = (daily_calories as f32 * macro_split.fat_percent / 100.0) / 9.0 / meals_per_day as f32;

        let avoid_ingredients = preferences
            .and_then(|p| p.avoid_ingredients.as_ref())
            .cloned()
            .unwrap_or_else(Vec::new);

        let mut meals = Vec::new();
        
        // Generate breakfast
        meals.push(Meal {
            name: "Power Breakfast".to_string(),
            meal_type: MealType::Breakfast,
            calories: calorie_per_meal,
            protein_grams: protein_grams_per_meal,
            carbohydrate_grams: carb_grams_per_meal,
            fat_grams: fat_grams_per_meal,
            ingredients: self.generate_breakfast_ingredients(&avoid_ingredients, &profile.dietary_restrictions),
            instructions: "Combine oats with protein powder and berries. Prepare the night before for quick morning meal.".to_string(),
        });

        // Generate lunch
        meals.push(Meal {
            name: "Balanced Lunch".to_string(),
            meal_type: MealType::Lunch,
            calories: calorie_per_meal,
            protein_grams: protein_grams_per_meal,
            carbohydrate_grams: carb_grams_per_meal,
            fat_grams: fat_grams_per_meal,
            ingredients: self.generate_lunch_ingredients(&avoid_ingredients, &profile.dietary_restrictions),
            instructions: "Grill chicken and serve over quinoa with roasted vegetables. Drizzle with olive oil.".to_string(),
        });

        // Generate dinner
        meals.push(Meal {
            name: "Nutritious Dinner".to_string(),
            meal_type: MealType::Dinner,
            calories: calorie_per_meal,
            protein_grams: protein_grams_per_meal,
            carbohydrate_grams: carb_grams_per_meal,
            fat_grams: fat_grams_per_meal,
            ingredients: self.generate_dinner_ingredients(&avoid_ingredients, &profile.dietary_restrictions),
            instructions: "Bake salmon with sweet potato and steamed broccoli. Season with herbs and lemon.".to_string(),
        });

        // Generate snack if more than 3 meals
        if meals_per_day > 3 {
            meals.push(Meal {
                name: "Healthy Snack".to_string(),
                meal_type: MealType::Snack,
                calories: calorie_per_meal,
                protein_grams: protein_grams_per_meal,
                carbohydrate_grams: carb_grams_per_meal,
                fat_grams: fat_grams_per_meal,
                ingredients: self.generate_snack_ingredients(&avoid_ingredients, &profile.dietary_restrictions),
                instructions: "Mix Greek yogurt with nuts and berries for a protein-rich snack.".to_string(),
            });
        }

        meals
    }

    fn generate_breakfast_ingredients(&self, avoid: &[String], restrictions: &[String]) -> Vec<String> {
        let mut ingredients = vec![
            "Rolled oats (1/2 cup)".to_string(),
            "Protein powder (1 scoop)".to_string(),
            "Mixed berries (1/2 cup)".to_string(),
            "Almond butter (1 tbsp)".to_string(),
            "Chia seeds (1 tsp)".to_string(),
        ];

        // Filter based on dietary restrictions
        if restrictions.contains(&"gluten-free".to_string()) {
            ingredients = ingredients.into_iter()
                .map(|ing| if ing.contains("oats") { "Gluten-free oats (1/2 cup)".to_string() } else { ing })
                .collect();
        }

        if restrictions.contains(&"vegan".to_string()) {
            ingredients = ingredients.into_iter()
                .map(|ing| {
                    if ing.contains("protein powder") {
                        "Plant protein powder (1 scoop)".to_string()
                    } else {
                        ing
                    }
                })
                .collect();
        }

        // Remove avoided ingredients
        ingredients.retain(|ing| !avoid.iter().any(|avoid_item| ing.to_lowercase().contains(&avoid_item.to_lowercase())));

        ingredients
    }

    fn generate_lunch_ingredients(&self, avoid: &[String], restrictions: &[String]) -> Vec<String> {
        let mut ingredients = if restrictions.contains(&"vegetarian".to_string()) || restrictions.contains(&"vegan".to_string()) {
            vec![
                "Quinoa (1 cup cooked)".to_string(),
                "Black beans (1/2 cup)".to_string(),
                "Roasted vegetables (1 cup)".to_string(),
                "Avocado (1/4 medium)".to_string(),
                "Olive oil (1 tbsp)".to_string(),
            ]
        } else {
            vec![
                "Grilled chicken breast (4 oz)".to_string(),
                "Brown rice (1/2 cup)".to_string(),
                "Mixed vegetables (1 cup)".to_string(),
                "Olive oil (1 tbsp)".to_string(),
                "Lemon juice (1 tbsp)".to_string(),
            ]
        };

        ingredients.retain(|ing| !avoid.iter().any(|avoid_item| ing.to_lowercase().contains(&avoid_item.to_lowercase())));
        ingredients
    }

    fn generate_dinner_ingredients(&self, avoid: &[String], restrictions: &[String]) -> Vec<String> {
        let mut ingredients = if restrictions.contains(&"vegetarian".to_string()) || restrictions.contains(&"vegan".to_string()) {
            vec![
                "Lentils (1 cup cooked)".to_string(),
                "Sweet potato (1 medium)".to_string(),
                "Steamed broccoli (1 cup)".to_string(),
                "Tahini (1 tbsp)".to_string(),
                "Nutritional yeast (1 tbsp)".to_string(),
            ]
        } else {
            vec![
                "Baked salmon (4 oz)".to_string(),
                "Sweet potato (1 medium)".to_string(),
                "Steamed broccoli (1 cup)".to_string(),
                "Olive oil (1 tbsp)".to_string(),
                "Fresh herbs".to_string(),
            ]
        };

        ingredients.retain(|ing| !avoid.iter().any(|avoid_item| ing.to_lowercase().contains(&avoid_item.to_lowercase())));
        ingredients
    }

    fn generate_snack_ingredients(&self, avoid: &[String], restrictions: &[String]) -> Vec<String> {
        let mut ingredients = if restrictions.contains(&"vegan".to_string()) {
            vec![
                "Coconut yogurt (1/2 cup)".to_string(),
                "Mixed nuts (1 oz)".to_string(),
                "Fresh berries (1/4 cup)".to_string(),
            ]
        } else {
            vec![
                "Greek yogurt (1/2 cup)".to_string(),
                "Mixed nuts (1 oz)".to_string(),
                "Fresh berries (1/4 cup)".to_string(),
            ]
        };

        ingredients.retain(|ing| !avoid.iter().any(|avoid_item| ing.to_lowercase().contains(&avoid_item.to_lowercase())));
        ingredients
    }

    fn format_nutrition_plan_response(&self, plan: &NutritionPlan) -> String {
        let mut response = format!(
            "# {}\n\n## Overview\n{}\n\n**Daily Calories:** {}\n**Macronutrient Split:**\n- Protein: {:.1}%\n- Carbohydrates: {:.1}%\n- Fats: {:.1}%\n\n",
            plan.name,
            plan.description,
            plan.daily_calories,
            plan.macronutrient_split.protein_percent,
            plan.macronutrient_split.carbohydrate_percent,
            plan.macronutrient_split.fat_percent
        );

        response.push_str("## Daily Meal Plan\n\n");
        for (i, meal) in plan.meals.iter().enumerate() {
            response.push_str(&format!(
                "### {}. {} ({:?})\n**Calories:** {} | **Protein:** {:.1}g | **Carbs:** {:.1}g | **Fats:** {:.1}g\n\n**Ingredients:**\n{}\n\n**Instructions:** {}\n\n",
                i + 1,
                meal.name,
                meal.meal_type,
                meal.calories,
                meal.protein_grams,
                meal.carbohydrate_grams,
                meal.fat_grams,
                meal.ingredients.iter().map(|ing| format!("- {}", ing)).collect::<Vec<_>>().join("\n"),
                meal.instructions
            ));
        }

        if !plan.dietary_restrictions.is_empty() {
            response.push_str(&format!(
                "## Dietary Considerations\nThis plan accommodates: {}\n\n",
                plan.dietary_restrictions.join(", ")
            ));
        }

        response.push_str("## Additional Notes\n- Drink plenty of water throughout the day (8-10 glasses)\n- Adjust portion sizes based on hunger and satiety cues\n- Include variety by rotating different protein sources and vegetables\n- Consider meal prep on weekends for convenience");

        response
    }

    // Advanced nutrition analysis
    pub async fn analyze_nutrition(&self, args: Option<Value>) -> Result<CallToolResult> {
        let request: NutritionAnalysisRequest = match args {
            Some(args) => serde_json::from_value(args)?,
            None => return Ok(CallToolResult {
                content: vec![ToolResponseContent::Text {
                    text: "Error: No nutrition analysis parameters provided".to_string(),
                }],
                is_error: Some(true),
            }),
        };

        let analysis = self.perform_nutrition_analysis(&request).await?;
        let response_text = self.format_nutrition_analysis(&analysis, &request);

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: response_text,
            }],
            is_error: None,
        })
    }

    async fn perform_nutrition_analysis(&self, request: &NutritionAnalysisRequest) -> Result<NutritionAnalysis> {
        let analysis_type = request.analysis_type.as_deref().unwrap_or("basic");
        
        // Calculate basic macronutrients
        let macronutrients = self.calculate_macronutrients(&request.foods);
        let total_calories = macronutrients.protein_grams * 4.0 + 
                           macronutrients.carbohydrate_grams * 4.0 + 
                           macronutrients.fat_grams * 9.0;

        let micronutrients = if analysis_type == "micronutrients" || analysis_type == "interactions" {
            Some(self.analyze_micronutrients(&request.foods))
        } else {
            None
        };

        let nutrient_interactions = if analysis_type == "interactions" {
            Some(self.analyze_nutrient_interactions(&request.foods))
        } else {
            None
        };

        let timing_recommendations = if analysis_type == "timing" {
            Some(self.generate_timing_recommendations(&request.foods))
        } else {
            None
        };

        let deficiency_risks = self.assess_deficiency_risks(&request.foods, micronutrients.as_ref());

        Ok(NutritionAnalysis {
            total_calories,
            macronutrients,
            micronutrients,
            nutrient_interactions,
            timing_recommendations,
            deficiency_risks,
        })
    }

    fn calculate_macronutrients(&self, foods: &[FoodItem]) -> MacronutrientBreakdown {
        // This would integrate with a comprehensive nutrition database
        // For now, using estimated values based on common foods
        let mut total_protein = 0.0;
        let mut total_carbs = 0.0;
        let mut total_fat = 0.0;
        let mut total_fiber = 0.0;

        for food in foods {
            let nutrition_info = self.nutrition_database.get_nutrition_info(&food.name, food.quantity, &food.unit);
            total_protein += nutrition_info.protein;
            total_carbs += nutrition_info.carbohydrates;
            total_fat += nutrition_info.fat;
            total_fiber += nutrition_info.fiber;
        }

        let total_calories = total_protein * 4.0 + total_carbs * 4.0 + total_fat * 9.0;

        MacronutrientBreakdown {
            protein_grams: total_protein,
            carbohydrate_grams: total_carbs,
            fat_grams: total_fat,
            fiber_grams: total_fiber,
            protein_percent: if total_calories > 0.0 { (total_protein * 4.0 / total_calories) * 100.0 } else { 0.0 },
            carb_percent: if total_calories > 0.0 { (total_carbs * 4.0 / total_calories) * 100.0 } else { 0.0 },
            fat_percent: if total_calories > 0.0 { (total_fat * 9.0 / total_calories) * 100.0 } else { 0.0 },
        }
    }

    fn analyze_micronutrients(&self, _foods: &[FoodItem]) -> MicronutrientAnalysis {
        // Simplified micronutrient analysis
        let mut vitamins = HashMap::new();
        let mut minerals = HashMap::new();

        // Example vitamin analysis
        vitamins.insert("Vitamin C".to_string(), MicronutrientInfo {
            amount: 45.0,
            unit: "mg".to_string(),
            daily_value_percent: 75.0,
            adequacy_status: "adequate".to_string(),
        });

        minerals.insert("Iron".to_string(), MicronutrientInfo {
            amount: 8.5,
            unit: "mg".to_string(),
            daily_value_percent: 65.0,
            adequacy_status: "adequate".to_string(),
        });

        MicronutrientAnalysis {
            vitamins,
            minerals,
            overall_score: 72.0,
        }
    }

    fn analyze_nutrient_interactions(&self, _foods: &[FoodItem]) -> Vec<NutrientInteraction> {
        vec![
            NutrientInteraction {
                nutrients: vec!["Vitamin C".to_string(), "Iron".to_string()],
                interaction_type: "synergistic".to_string(),
                effect: "Vitamin C enhances iron absorption".to_string(),
                recommendation: "Consume vitamin C-rich foods with iron-rich meals".to_string(),
            },
            NutrientInteraction {
                nutrients: vec!["Calcium".to_string(), "Iron".to_string()],
                interaction_type: "antagonistic".to_string(),
                effect: "Calcium can inhibit iron absorption".to_string(),
                recommendation: "Separate calcium and iron supplements by 2+ hours".to_string(),
            },
        ]
    }

    fn generate_timing_recommendations(&self, _foods: &[FoodItem]) -> TimingRecommendations {
        TimingRecommendations {
            pre_workout: vec![
                "Consume carbohydrates 1-2 hours before exercise".to_string(),
                "Light protein snack 30 minutes before".to_string(),
            ],
            post_workout: vec![
                "Consume protein within 30 minutes after exercise".to_string(),
                "Include carbohydrates to replenish glycogen".to_string(),
            ],
            morning: vec![
                "Include protein to stabilize blood sugar".to_string(),
                "Add healthy fats for sustained energy".to_string(),
            ],
            evening: vec![
                "Lighter meals 2-3 hours before bed".to_string(),
                "Include magnesium-rich foods for better sleep".to_string(),
            ],
        }
    }

    fn assess_deficiency_risks(&self, _foods: &[FoodItem], _micronutrients: Option<&MicronutrientAnalysis>) -> Vec<DeficiencyRisk> {
        vec![
            DeficiencyRisk {
                nutrient: "Vitamin D".to_string(),
                risk_level: "moderate".to_string(),
                symptoms: vec!["Fatigue".to_string(), "Bone pain".to_string(), "Muscle weakness".to_string()],
                food_sources: vec!["Fatty fish".to_string(), "Fortified dairy".to_string(), "Egg yolks".to_string()],
            },
        ]
    }

    fn format_nutrition_analysis(&self, analysis: &NutritionAnalysis, request: &NutritionAnalysisRequest) -> String {
        let mut response = format!(
            "# Nutrition Analysis\n\n## Foods Analyzed\n{}\n\n## Macronutrient Summary\n**Total Calories:** {:.0}\n**Protein:** {:.1}g ({:.1}%)\n**Carbohydrates:** {:.1}g ({:.1}%)\n**Fats:** {:.1}g ({:.1}%)\n**Fiber:** {:.1}g\n\n",
            request.foods.iter().map(|f| format!("- {} ({} {})", f.name, f.quantity, f.unit)).collect::<Vec<_>>().join("\n"),
            analysis.total_calories,
            analysis.macronutrients.protein_grams,
            analysis.macronutrients.protein_percent,
            analysis.macronutrients.carbohydrate_grams,
            analysis.macronutrients.carb_percent,
            analysis.macronutrients.fat_grams,
            analysis.macronutrients.fat_percent,
            analysis.macronutrients.fiber_grams
        );

        if let Some(ref micronutrients) = analysis.micronutrients {
            response.push_str(&format!("## Micronutrient Analysis\n**Overall Score:** {:.0}/100\n\n", micronutrients.overall_score));
        }

        if let Some(ref interactions) = analysis.nutrient_interactions {
            response.push_str("## Nutrient Interactions\n");
            for interaction in interactions {
                response.push_str(&format!("- **{}**: {}\n  *Recommendation:* {}\n\n", 
                    interaction.nutrients.join(" + "), 
                    interaction.effect, 
                    interaction.recommendation
                ));
            }
        }

        if !analysis.deficiency_risks.is_empty() {
            response.push_str("## Potential Deficiency Risks\n");
            for risk in &analysis.deficiency_risks {
                response.push_str(&format!("- **{}** (Risk: {})\n  Food sources: {}\n\n", 
                    risk.nutrient, 
                    risk.risk_level, 
                    risk.food_sources.join(", ")
                ));
            }
        }

        response
    }
}

impl Default for NutritionTools {
    fn default() -> Self {
        Self::new()
    }
}

// Simplified nutrition database
#[derive(Debug)]
struct NutritionDatabase {
    // In a real implementation, this would be a comprehensive database
}

#[derive(Debug)]
struct NutritionInfo {
    protein: f32,
    carbohydrates: f32,
    fat: f32,
    fiber: f32,
}

impl NutritionDatabase {
    fn new() -> Self {
        Self {}
    }

    fn get_nutrition_info(&self, food_name: &str, quantity: f32, unit: &str) -> NutritionInfo {
        // Simplified nutrition lookup - in reality would use comprehensive database
        let base_nutrition = match food_name.to_lowercase().as_str() {
            name if name.contains("chicken") => NutritionInfo { protein: 25.0, carbohydrates: 0.0, fat: 3.0, fiber: 0.0 },
            name if name.contains("salmon") => NutritionInfo { protein: 22.0, carbohydrates: 0.0, fat: 12.0, fiber: 0.0 },
            name if name.contains("rice") => NutritionInfo { protein: 4.0, carbohydrates: 45.0, fat: 1.0, fiber: 2.0 },
            name if name.contains("broccoli") => NutritionInfo { protein: 3.0, carbohydrates: 6.0, fat: 0.0, fiber: 3.0 },
            name if name.contains("oats") => NutritionInfo { protein: 6.0, carbohydrates: 27.0, fat: 3.0, fiber: 4.0 },
            _ => NutritionInfo { protein: 5.0, carbohydrates: 15.0, fat: 2.0, fiber: 1.0 }, // Default values
        };

        // Scale by quantity (simplified - doesn't account for different units properly)
        let scale = match unit {
            "cup" | "cups" => quantity,
            "oz" | "ounces" => quantity / 4.0,
            "g" | "grams" => quantity / 100.0,
            _ => quantity,
        };

        NutritionInfo {
            protein: base_nutrition.protein * scale,
            carbohydrates: base_nutrition.carbohydrates * scale,
            fat: base_nutrition.fat * scale,
            fiber: base_nutrition.fiber * scale,
        }
    }
}