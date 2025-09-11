// src/sample_data/recipes.rs - Sample recipe database with comprehensive meal options

use crate::models::food::*;

pub fn create_sample_recipes() -> Vec<Recipe> {
    let mut recipes = Vec::new();

    // === BREAKFAST RECIPES ===

    // Greek Yogurt Berry Bowl
    recipes.push(Recipe {
        id: "greek_yogurt_berry_bowl".to_string(),
        name: "Greek Yogurt Berry Bowl".to_string(),
        description: "High-protein breakfast with fresh berries and almonds".to_string(),
        ingredients: vec![
            Ingredient {
                food_id: "greek_yogurt".to_string(),
                amount_g: 200.0,
                preparation: Some("plain".to_string()),
                substitutes: vec![],
            },
            Ingredient {
                food_id: "blueberries".to_string(),
                amount_g: 100.0,
                preparation: Some("fresh".to_string()),
                substitutes: vec!["strawberries".to_string()],
            },
            Ingredient {
                food_id: "almonds".to_string(),
                amount_g: 30.0,
                preparation: Some("sliced".to_string()),
                substitutes: vec!["walnuts".to_string()],
            },
        ],
        instructions: vec![
            "Place Greek yogurt in a bowl".to_string(),
            "Top with fresh blueberries".to_string(),
            "Sprinkle sliced almonds on top".to_string(),
            "Serve immediately".to_string(),
        ],
        prep_time_minutes: 5,
        cook_time_minutes: 0,
        servings: 1,
        difficulty: DifficultyLevel::Easy,
        cuisine_type: Some("American".to_string()),
        meal_type: MealType::Breakfast,
        nutrition_per_serving: NutritionFacts {
            calories: 291.0, // 200g yogurt (118) + 100g blueberries (57) + 30g almonds (174)
            protein_g: 26.4, // 20.0 + 0.7 + 6.4
            carbs_g: 21.6,   // 7.2 + 14.5 + 6.5
            fat_g: 15.4,     // 0.8 + 0.3 + 15.0
            fiber_g: 6.2,    // 0.0 + 2.4 + 3.8
            sugar_g: 17.7,   // 7.2 + 10.0 + 1.3
            sodium_mg: 72.3, // 72.0 + 1.0 + 0.3
            potassium_mg: 502.1, // 282.0 + 77.0 + 220.0
            calcium_mg: 300.7, // 220.0 + 6.0 + 80.7
            iron_mg: 1.1,    // 0.0 + 0.3 + 1.1
            vitamin_c_mg: 9.7, // 0.0 + 9.7 + 0.0
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 1.0, // 1.0 + 0.0 + 0.0
            folate_mcg: 27.2, // 14.0 + 6.0 + 13.2
            omega3_g: 0.1,   // 0.0 + 0.1 + 0.0
            omega6_g: 3.7,   // 0.0 + 0.1 + 3.7
        },
        allergens: vec![Allergen::Dairy, Allergen::TreeNuts],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::GlutenFree],
        rating: Some(4.5),
        cost_per_serving: Some(3.30), // $1.50*2 + $3.00*1 + $8.00*0.3
    });

    // Scrambled Eggs with Spinach
    recipes.push(Recipe {
        id: "scrambled_eggs_spinach".to_string(),
        name: "Scrambled Eggs with Spinach".to_string(),
        description: "Protein-rich breakfast with fresh spinach and olive oil".to_string(),
        ingredients: vec![
            Ingredient {
                food_id: "eggs".to_string(),
                amount_g: 150.0, // ~3 large eggs
                preparation: Some("scrambled".to_string()),
                substitutes: vec![],
            },
            Ingredient {
                food_id: "spinach".to_string(),
                amount_g: 100.0,
                preparation: Some("fresh, chopped".to_string()),
                substitutes: vec!["kale".to_string()],
            },
            Ingredient {
                food_id: "olive_oil".to_string(),
                amount_g: 10.0,
                preparation: Some("for cooking".to_string()),
                substitutes: vec!["butter".to_string()],
            },
        ],
        instructions: vec![
            "Heat olive oil in a non-stick pan over medium heat".to_string(),
            "Add chopped spinach and cook until wilted, about 2 minutes".to_string(),
            "Beat eggs in a bowl and add to the pan".to_string(),
            "Scramble eggs gently until cooked through, about 3-4 minutes".to_string(),
            "Season with salt and pepper to taste".to_string(),
        ],
        prep_time_minutes: 5,
        cook_time_minutes: 7,
        servings: 1,
        difficulty: DifficultyLevel::Easy,
        cuisine_type: Some("American".to_string()),
        meal_type: MealType::Breakfast,
        nutrition_per_serving: NutritionFacts {
            calories: 344.0, // 150g eggs (233) + 100g spinach (23) + 10g olive oil (88)
            protein_g: 22.4, // 19.5 + 2.9 + 0.0
            carbs_g: 5.3,    // 1.7 + 3.6 + 0.0
            fat_g: 26.4,     // 16.5 + 0.4 + 10.0
            fiber_g: 2.2,    // 0.0 + 2.2 + 0.0
            sugar_g: 2.1,    // 1.7 + 0.4 + 0.0
            sodium_mg: 265.0, // 186.0 + 79.0 + 0.2
            potassium_mg: 765.0, // 207.0 + 558.0 + 0.1
            calcium_mg: 174.0, // 75.0 + 99.0 + 0.1
            iron_mg: 4.5,    // 1.8 + 2.7 + 0.06
            vitamin_c_mg: 28.1, // 0.0 + 28.1 + 0.0
            vitamin_d_iu: 130.5, // 130.5 + 0.0 + 0.0
            vitamin_b12_mcg: 1.7, // 1.7 + 0.0 + 0.0
            folate_mcg: 260.0, // 66.0 + 194.0 + 0.0
            omega3_g: 0.3,   // 0.2 + 0.1 + 0.08
            omega6_g: 3.1,   // 2.1 + 0.1 + 0.98
        },
        allergens: vec![Allergen::Eggs],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::GlutenFree, DietaryFlag::Keto],
        rating: Some(4.3),
        cost_per_serving: Some(2.00), // $1.20*1.5 + $1.20*1 + $12.00*0.1
    });

    // === LUNCH RECIPES ===

    // Grilled Chicken Salad
    recipes.push(Recipe {
        id: "grilled_chicken_salad".to_string(),
        name: "Grilled Chicken Spinach Salad".to_string(),
        description: "Lean protein salad with fresh spinach and olive oil dressing".to_string(),
        ingredients: vec![
            Ingredient {
                food_id: "chicken_breast".to_string(),
                amount_g: 150.0,
                preparation: Some("grilled, sliced".to_string()),
                substitutes: vec!["turkey_breast".to_string()],
            },
            Ingredient {
                food_id: "spinach".to_string(),
                amount_g: 150.0,
                preparation: Some("fresh, washed".to_string()),
                substitutes: vec!["mixed_greens".to_string()],
            },
            Ingredient {
                food_id: "olive_oil".to_string(),
                amount_g: 15.0,
                preparation: Some("extra virgin".to_string()),
                substitutes: vec![],
            },
        ],
        instructions: vec![
            "Season chicken breast with salt and pepper".to_string(),
            "Grill chicken for 6-7 minutes per side until cooked through".to_string(),
            "Let chicken rest for 5 minutes, then slice".to_string(),
            "Arrange spinach in a large bowl".to_string(),
            "Top with sliced chicken".to_string(),
            "Drizzle with olive oil and season to taste".to_string(),
        ],
        prep_time_minutes: 10,
        cook_time_minutes: 15,
        servings: 1,
        difficulty: DifficultyLevel::Medium,
        cuisine_type: Some("Mediterranean".to_string()),
        meal_type: MealType::Lunch,
        nutrition_per_serving: NutritionFacts {
            calories: 414.0, // 150g chicken (248) + 150g spinach (35) + 15g olive oil (133)
            protein_g: 51.0, // 46.5 + 4.4 + 0.0
            carbs_g: 5.4,    // 0.0 + 5.4 + 0.0
            fat_g: 20.0,     // 5.4 + 0.6 + 15.0
            fiber_g: 3.3,    // 0.0 + 3.3 + 0.0
            sugar_g: 0.6,    // 0.0 + 0.6 + 0.0
            sodium_mg: 229.5, // 111.0 + 118.5 + 0.3
            potassium_mg: 1221.0, // 384.0 + 837.0 + 0.2
            calcium_mg: 171.0, // 22.5 + 148.5 + 0.2
            iron_mg: 5.4,    // 1.4 + 4.1 + 0.09
            vitamin_c_mg: 42.2, // 0.0 + 42.2 + 0.0
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.5, // 0.5 + 0.0 + 0.0
            folate_mcg: 297.0, // 6.0 + 291.0 + 0.0
            omega3_g: 0.3,   // 0.2 + 0.2 + 0.12
            omega6_g: 2.9,   // 1.2 + 0.2 + 1.47
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::GlutenFree, DietaryFlag::DairyFree, DietaryFlag::Keto, DietaryFlag::Paleo],
        rating: Some(4.6),
        cost_per_serving: Some(5.55), // $2.50*1.5 + $1.20*1.5 + $12.00*0.15
    });

    // Salmon Rice Bowl
    recipes.push(Recipe {
        id: "salmon_rice_bowl".to_string(),
        name: "Baked Salmon with Brown Rice".to_string(),
        description: "Omega-3 rich salmon with nutritious brown rice and broccoli".to_string(),
        ingredients: vec![
            Ingredient {
                food_id: "salmon".to_string(),
                amount_g: 150.0,
                preparation: Some("baked fillet".to_string()),
                substitutes: vec!["tuna".to_string()],
            },
            Ingredient {
                food_id: "brown_rice".to_string(),
                amount_g: 150.0, // cooked
                preparation: Some("cooked".to_string()),
                substitutes: vec!["quinoa".to_string()],
            },
            Ingredient {
                food_id: "broccoli".to_string(),
                amount_g: 100.0,
                preparation: Some("steamed".to_string()),
                substitutes: vec!["asparagus".to_string()],
            },
            Ingredient {
                food_id: "olive_oil".to_string(),
                amount_g: 10.0,
                preparation: Some("for cooking".to_string()),
                substitutes: vec![],
            },
        ],
        instructions: vec![
            "Preheat oven to 400°F (200°C)".to_string(),
            "Cook brown rice according to package instructions".to_string(),
            "Season salmon with salt, pepper, and a drizzle of olive oil".to_string(),
            "Bake salmon for 12-15 minutes until flakes easily".to_string(),
            "Steam broccoli for 5-7 minutes until tender".to_string(),
            "Serve salmon over rice with steamed broccoli".to_string(),
        ],
        prep_time_minutes: 15,
        cook_time_minutes: 20,
        servings: 1,
        difficulty: DifficultyLevel::Medium,
        cuisine_type: Some("Asian-Fusion".to_string()),
        meal_type: MealType::Lunch,
        nutrition_per_serving: NutritionFacts {
            calories: 584.0, // 150g salmon (312) + 150g rice (167) + 100g broccoli (34) + 10g oil (88)
            protein_g: 42.5, // 38.1 + 3.9 + 2.8 + 0.0
            carbs_g: 41.5,   // 0.0 + 34.5 + 7.0 + 0.0
            fat_g: 29.2,     // 18.6 + 1.4 + 0.4 + 10.0
            fiber_g: 4.4,    // 0.0 + 2.7 + 2.6 + 0.0
            sugar_g: 2.1,    // 0.0 + 0.6 + 1.5 + 0.0
            sodium_mg: 154.0, // 66.0 + 7.5 + 33.0 + 0.2
            potassium_mg: 1128.5, // 544.5 + 64.5 + 316.0 + 0.1
            calcium_mg: 76.5, // 13.5 + 15.0 + 47.0 + 0.1
            iron_mg: 1.65,   // 0.45 + 0.6 + 0.7 + 0.06
            vitamin_c_mg: 89.2, // 0.0 + 0.0 + 89.2 + 0.0
            vitamin_d_iu: 789.0, // 789.0 + 0.0 + 0.0 + 0.0
            vitamin_b12_mcg: 4.2, // 4.2 + 0.0 + 0.0 + 0.0
            folate_mcg: 105.0, // 39.0 + 6.0 + 63.0 + 0.0
            omega3_g: 3.53,  // 3.45 + 0.0 + 0.1 + 0.08
            omega6_g: 2.63,  // 1.35 + 0.3 + 0.1 + 0.98
        },
        allergens: vec![Allergen::Fish],
        dietary_flags: vec![DietaryFlag::GlutenFree, DietaryFlag::DairyFree],
        rating: Some(4.7),
        cost_per_serving: Some(10.30), // $6.00*1.5 + $0.30*1.5 + $0.80*1 + $12.00*0.1
    });

    // === DINNER RECIPES ===

    // Chicken and Vegetable Stir-fry
    recipes.push(Recipe {
        id: "chicken_vegetable_stir_fry".to_string(),
        name: "Chicken and Vegetable Stir-fry".to_string(),
        description: "Quick and healthy stir-fry with chicken, broccoli, and brown rice".to_string(),
        ingredients: vec![
            Ingredient {
                food_id: "chicken_breast".to_string(),
                amount_g: 200.0,
                preparation: Some("diced".to_string()),
                substitutes: vec!["tofu".to_string()],
            },
            Ingredient {
                food_id: "broccoli".to_string(),
                amount_g: 150.0,
                preparation: Some("cut into florets".to_string()),
                substitutes: vec!["bell_peppers".to_string()],
            },
            Ingredient {
                food_id: "brown_rice".to_string(),
                amount_g: 150.0,
                preparation: Some("cooked".to_string()),
                substitutes: vec!["cauliflower_rice".to_string()],
            },
            Ingredient {
                food_id: "olive_oil".to_string(),
                amount_g: 15.0,
                preparation: Some("for stir-frying".to_string()),
                substitutes: vec!["sesame_oil".to_string()],
            },
        ],
        instructions: vec![
            "Cook brown rice according to package directions".to_string(),
            "Heat olive oil in a large wok or skillet over high heat".to_string(),
            "Add diced chicken and cook for 5-6 minutes until golden".to_string(),
            "Add broccoli florets and stir-fry for 3-4 minutes".to_string(),
            "Season with soy sauce, garlic, and ginger to taste".to_string(),
            "Serve over brown rice".to_string(),
        ],
        prep_time_minutes: 10,
        cook_time_minutes: 15,
        servings: 1,
        difficulty: DifficultyLevel::Medium,
        cuisine_type: Some("Asian".to_string()),
        meal_type: MealType::Dinner,
        nutrition_per_serving: NutritionFacts {
            calories: 730.0, // 200g chicken (330) + 150g broccoli (51) + 150g rice (167) + 15g oil (133)
            protein_g: 66.0, // 62.0 + 4.2 + 3.9 + 0.0
            carbs_g: 45.0,   // 0.0 + 10.5 + 34.5 + 0.0
            fat_g: 25.6,     // 7.2 + 0.6 + 1.4 + 15.0
            fiber_g: 6.6,    // 0.0 + 3.9 + 2.7 + 0.0
            sugar_g: 2.9,    // 0.0 + 2.3 + 0.6 + 0.0
            sodium_mg: 197.5, // 148.0 + 49.5 + 7.5 + 0.3
            potassium_mg: 1058.0, // 512.0 + 474.0 + 64.5 + 0.2
            calcium_mg: 93.0, // 30.0 + 70.5 + 15.0 + 0.2
            iron_mg: 2.4,    // 1.8 + 1.05 + 0.6 + 0.09
            vitamin_c_mg: 133.8, // 0.0 + 133.8 + 0.0 + 0.0
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.6, // 0.6 + 0.0 + 0.0 + 0.0
            folate_mcg: 100.5, // 8.0 + 94.5 + 6.0 + 0.0
            omega3_g: 0.32,  // 0.2 + 0.15 + 0.0 + 0.12
            omega6_g: 2.87,  // 1.6 + 0.15 + 0.3 + 1.47
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::GlutenFree, DietaryFlag::DairyFree],
        rating: Some(4.4),
        cost_per_serving: Some(6.25), // $2.50*2 + $0.80*1.5 + $0.30*1.5 + $12.00*0.15
    });

    // === SNACK RECIPES ===

    // Almond Banana Snack
    recipes.push(Recipe {
        id: "almond_banana_snack".to_string(),
        name: "Banana with Almonds".to_string(),
        description: "Simple, healthy snack with natural sweetness and protein".to_string(),
        ingredients: vec![
            Ingredient {
                food_id: "banana".to_string(),
                amount_g: 120.0, // 1 medium banana
                preparation: Some("sliced".to_string()),
                substitutes: vec!["apple".to_string()],
            },
            Ingredient {
                food_id: "almonds".to_string(),
                amount_g: 25.0,
                preparation: Some("raw".to_string()),
                substitutes: vec!["peanuts".to_string()],
            },
        ],
        instructions: vec![
            "Slice banana into rounds".to_string(),
            "Serve with raw almonds".to_string(),
            "Enjoy immediately".to_string(),
        ],
        prep_time_minutes: 2,
        cook_time_minutes: 0,
        servings: 1,
        difficulty: DifficultyLevel::Easy,
        cuisine_type: Some("American".to_string()),
        meal_type: MealType::Snack,
        nutrition_per_serving: NutritionFacts {
            calories: 252.0, // 120g banana (107) + 25g almonds (145)
            protein_g: 6.6,  // 1.3 + 5.3
            carbs_g: 32.9,   // 27.6 + 5.4
            fat_g: 12.8,     // 0.4 + 12.5
            fiber_g: 6.2,    // 3.1 + 3.1
            sugar_g: 16.9,   // 14.6 + 1.1
            sodium_mg: 1.5,  // 1.2 + 0.3
            potassium_mg: 612.8, // 429.6 + 183.3
            calcium_mg: 73.3, // 6.0 + 67.3
            iron_mg: 1.2,    // 0.4 + 0.9
            vitamin_c_mg: 10.4, // 10.4 + 0.0
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 35.0, // 24.0 + 11.0
            omega3_g: 0.0,
            omega6_g: 3.2,   // 0.1 + 3.1
        },
        allergens: vec![Allergen::TreeNuts],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree, DietaryFlag::Paleo],
        rating: Some(4.2),
        cost_per_serving: Some(2.48), // $0.40*1.2 + $8.00*0.25
    });

    recipes
}

pub fn get_sample_recipe_count() -> usize {
    create_sample_recipes().len()
}