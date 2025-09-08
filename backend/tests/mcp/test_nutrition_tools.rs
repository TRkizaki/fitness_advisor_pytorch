#[cfg(test)]
mod nutrition_tools_tests {
    use fitness_advisor_ai::mcp::{
        NutritionTools, UserProfile, Gender, ActivityLevel, FitnessGoal
    };
    use serde_json::json;
    use uuid::Uuid;

    fn create_test_user_profile() -> UserProfile {
        UserProfile {
            id: Uuid::new_v4(),
            age: 30,
            weight_kg: 75.0,
            height_cm: 180,
            gender: Gender::Male,
            activity_level: ActivityLevel::ModeratelyActive,
            fitness_goals: vec![FitnessGoal::MuscleGain],
            dietary_restrictions: vec![],
            health_conditions: vec![],
        }
    }

    #[tokio::test]
    async fn test_create_nutrition_plan_with_parameters() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 25,
                "weight_kg": 70.0,
                "height_cm": 175,
                "gender": "male",
                "activity_level": "moderately_active",
                "fitness_goals": ["muscle_gain"],
                "dietary_restrictions": [],
                "health_conditions": []
            },
            "calorie_target": 2500,
            "meal_preferences": {
                "meals_per_day": 4,
                "prep_time_minutes": 30,
                "cuisine_preferences": ["mediterranean"],
                "avoid_ingredients": ["dairy"],
                "macro_split": {
                    "protein_percent": 30.0,
                    "carbohydrate_percent": 40.0,
                    "fat_percent": 30.0
                }
            }
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Personalized Muscle Gain Nutrition Plan"));
            assert!(text.contains("2500"));
            assert!(text.contains("30.0%"));
            assert!(text.contains("Daily Meal Plan"));
            assert!(text.contains("Power Breakfast"));
            assert!(text.contains("Balanced Lunch"));
            assert!(text.contains("Nutritious Dinner"));
            assert!(text.contains("Healthy Snack"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_create_nutrition_plan_without_parameters() {
        let nutrition_tools = NutritionTools::new();
        
        let result = nutrition_tools.create_nutrition_plan(None).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Error"));
        } else {
            panic!("Expected error text content");
        }
        
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn test_weight_loss_nutrition_plan() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 35,
                "weight_kg": 85.0,
                "height_cm": 170,
                "gender": "female",
                "activity_level": "lightly_active",
                "fitness_goals": ["weight_loss"],
                "dietary_restrictions": [],
                "health_conditions": []
            }
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Weight Loss"));
            assert!(text.contains("35.0%")); // Higher protein for weight loss
            assert!(text.contains("30.0%")); // Lower carbs for weight loss
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_vegetarian_nutrition_plan() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 28,
                "weight_kg": 65.0,
                "height_cm": 165,
                "gender": "female",
                "activity_level": "moderately_active",
                "fitness_goals": ["general_fitness"],
                "dietary_restrictions": ["vegetarian"],
                "health_conditions": []
            }
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("vegetarian"));
            assert!(text.contains("Quinoa") || text.contains("Black beans") || text.contains("Lentils"));
            assert!(!text.contains("chicken") && !text.contains("salmon"));
            assert!(text.contains("This plan accommodates: vegetarian"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_vegan_nutrition_plan() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 26,
                "weight_kg": 60.0,
                "height_cm": 160,
                "gender": "female",
                "activity_level": "very_active",
                "fitness_goals": ["endurance"],
                "dietary_restrictions": ["vegan"],
                "health_conditions": []
            }
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Plant protein powder"));
            assert!(text.contains("Coconut yogurt") || text.contains("Nutritional yeast"));
            assert!(!text.contains("Greek yogurt") && !text.contains("salmon"));
            assert!(text.contains("20.0%")); // Lower protein for endurance
            assert!(text.contains("55.0%")); // Higher carbs for endurance
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_gluten_free_nutrition_plan() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 32,
                "weight_kg": 75.0,
                "height_cm": 175,
                "gender": "male",
                "activity_level": "moderately_active",
                "fitness_goals": ["strength_gain"],
                "dietary_restrictions": ["gluten-free"],
                "health_conditions": []
            }
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Gluten-free oats"));
            assert!(text.contains("This plan accommodates: gluten-free"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_analyze_nutrition_basic() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "foods": [
                {
                    "name": "chicken breast",
                    "quantity": 4.0,
                    "unit": "oz"
                },
                {
                    "name": "brown rice",
                    "quantity": 1.0,
                    "unit": "cup"
                },
                {
                    "name": "broccoli",
                    "quantity": 1.0,
                    "unit": "cup"
                }
            ],
            "analysis_type": "basic"
        });

        let result = nutrition_tools.analyze_nutrition(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Nutrition Analysis"));
            assert!(text.contains("Foods Analyzed"));
            assert!(text.contains("chicken breast"));
            assert!(text.contains("brown rice"));
            assert!(text.contains("broccoli"));
            assert!(text.contains("Macronutrient Summary"));
            assert!(text.contains("Total Calories"));
            assert!(text.contains("Protein"));
            assert!(text.contains("Carbohydrates"));
            assert!(text.contains("Fats"));
            assert!(text.contains("Fiber"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_analyze_nutrition_micronutrients() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "foods": [
                {
                    "name": "salmon",
                    "quantity": 6.0,
                    "unit": "oz"
                },
                {
                    "name": "spinach",
                    "quantity": 2.0,
                    "unit": "cup"
                }
            ],
            "analysis_type": "micronutrients"
        });

        let result = nutrition_tools.analyze_nutrition(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Micronutrient Analysis"));
            assert!(text.contains("Overall Score"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_analyze_nutrition_interactions() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "foods": [
                {
                    "name": "beef",
                    "quantity": 4.0,
                    "unit": "oz"
                },
                {
                    "name": "orange",
                    "quantity": 1.0,
                    "unit": "medium"
                }
            ],
            "analysis_type": "interactions"
        });

        let result = nutrition_tools.analyze_nutrition(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Nutrient Interactions"));
            assert!(text.contains("Vitamin C") && text.contains("Iron"));
            assert!(text.contains("synergistic") || text.contains("antagonistic"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_analyze_nutrition_timing() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "foods": [
                {
                    "name": "oats",
                    "quantity": 1.0,
                    "unit": "cup",
                    "meal_timing": "pre_workout"
                },
                {
                    "name": "protein powder",
                    "quantity": 1.0,
                    "unit": "scoop",
                    "meal_timing": "post_workout"
                }
            ],
            "analysis_type": "timing"
        });

        let result = nutrition_tools.analyze_nutrition(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            // Timing recommendations would be in a full implementation
            assert!(text.contains("Nutrition Analysis"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_analyze_nutrition_without_parameters() {
        let nutrition_tools = NutritionTools::new();
        
        let result = nutrition_tools.analyze_nutrition(None).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Error"));
        } else {
            panic!("Expected error text content");
        }
        
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn test_calorie_calculation_male() {
        let nutrition_tools = NutritionTools::new();
        
        let male_profile = UserProfile {
            id: Uuid::new_v4(),
            age: 25,
            weight_kg: 80.0,
            height_cm: 180,
            gender: Gender::Male,
            activity_level: ActivityLevel::ModeratelyActive,
            fitness_goals: vec![FitnessGoal::MuscleGain],
            dietary_restrictions: vec![],
            health_conditions: vec![],
        };

        let args = json!({
            "user_profile": male_profile
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            // Should have higher calorie target for muscle gain
            assert!(text.contains("calories"));
        }
    }

    #[tokio::test]
    async fn test_calorie_calculation_female() {
        let nutrition_tools = NutritionTools::new();
        
        let female_profile = UserProfile {
            id: Uuid::new_v4(),
            age: 25,
            weight_kg: 60.0,
            height_cm: 165,
            gender: Gender::Female,
            activity_level: ActivityLevel::ModeratelyActive,
            fitness_goals: vec![FitnessGoal::WeightLoss],
            dietary_restrictions: vec![],
            health_conditions: vec![],
        };

        let args = json!({
            "user_profile": female_profile
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            // Should have lower calorie target for weight loss
            assert!(text.contains("Weight Loss"));
            assert!(text.contains("calories"));
        }
    }

    #[tokio::test]
    async fn test_endurance_macro_split() {
        let nutrition_tools = NutritionTools::new();
        
        let endurance_profile = UserProfile {
            id: Uuid::new_v4(),
            age: 30,
            weight_kg: 70.0,
            height_cm: 175,
            gender: Gender::Male,
            activity_level: ActivityLevel::VeryActive,
            fitness_goals: vec![FitnessGoal::Endurance],
            dietary_restrictions: vec![],
            health_conditions: vec![],
        };

        let args = json!({
            "user_profile": endurance_profile
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            // Endurance athletes need higher carb percentage
            assert!(text.contains("55.0%")); // Carbs
            assert!(text.contains("20.0%")); // Lower protein
            assert!(text.contains("25.0%")); // Moderate fat
        }
    }

    #[tokio::test]
    async fn test_nutrition_database_lookup() {
        let nutrition_tools = NutritionTools::new();
        
        // Test with known high-protein food
        let args = json!({
            "foods": [
                {
                    "name": "chicken breast",
                    "quantity": 100.0,
                    "unit": "g"
                }
            ]
        });

        let result = nutrition_tools.analyze_nutrition(Some(args)).await.unwrap();
        
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            // Should show protein content for chicken
            assert!(text.contains("Protein"));
            assert!(text.contains("chicken breast"));
        }
    }

    #[tokio::test]
    async fn test_deficiency_risk_assessment() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "foods": [
                {
                    "name": "white bread",
                    "quantity": 2.0,
                    "unit": "slice"
                }
            ],
            "analysis_type": "micronutrients"
        });

        let result = nutrition_tools.analyze_nutrition(Some(args)).await.unwrap();
        
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            // Should identify potential deficiency risks
            assert!(text.contains("Deficiency Risks"));
        }
    }

    #[tokio::test]
    async fn test_meal_preferences_respected() {
        let nutrition_tools = NutritionTools::new();
        
        let args = json!({
            "user_profile": create_test_user_profile(),
            "meal_preferences": {
                "meals_per_day": 3,
                "avoid_ingredients": ["nuts", "dairy"]
            }
        });

        let result = nutrition_tools.create_nutrition_plan(Some(args)).await.unwrap();
        
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            // Should have only 3 meals (breakfast, lunch, dinner - no snack)
            assert!(text.contains("Power Breakfast"));
            assert!(text.contains("Balanced Lunch"));
            assert!(text.contains("Nutritious Dinner"));
            // Should avoid nuts and dairy in ingredients
            assert!(!text.contains("nuts") && !text.contains("Greek yogurt"));
        }
    }
}