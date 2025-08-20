// src/sample_data/foods.rs - Sample food database with comprehensive nutrition data

use crate::models::food::*;
use std::collections::HashMap;

pub fn create_sample_foods() -> HashMap<String, Food> {
    let mut foods = HashMap::new();

    // === PROTEINS ===
    
    // Chicken Breast
    foods.insert("chicken_breast".to_string(), Food {
        id: "chicken_breast".to_string(),
        name: "Chicken Breast (skinless)".to_string(),
        category: FoodCategory::Protein,
        nutrition_per_100g: NutritionFacts {
            calories: 165.0,
            protein_g: 31.0,
            carbs_g: 0.0,
            fat_g: 3.6,
            fiber_g: 0.0,
            sugar_g: 0.0,
            sodium_mg: 74.0,
            potassium_mg: 256.0,
            calcium_mg: 15.0,
            iron_mg: 0.9,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.3,
            folate_mcg: 4.0,
            omega3_g: 0.1,
            omega6_g: 0.8,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::GlutenFree, DietaryFlag::DairyFree],
        seasonality: None,
        cost_per_100g: Some(2.50),
        availability_score: 0.95,
        taste_profile: TasteProfile {
            sweetness: 0.0,
            saltiness: 0.1,
            sourness: 0.0,
            bitterness: 0.0,
            umami: 0.6,
            spiciness: 0.0,
        },
    });

    // Salmon
    foods.insert("salmon".to_string(), Food {
        id: "salmon".to_string(),
        name: "Atlantic Salmon".to_string(),
        category: FoodCategory::Protein,
        nutrition_per_100g: NutritionFacts {
            calories: 208.0,
            protein_g: 25.4,
            carbs_g: 0.0,
            fat_g: 12.4,
            fiber_g: 0.0,
            sugar_g: 0.0,
            sodium_mg: 44.0,
            potassium_mg: 363.0,
            calcium_mg: 9.0,
            iron_mg: 0.3,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 526.0,
            vitamin_b12_mcg: 2.8,
            folate_mcg: 26.0,
            omega3_g: 2.3,
            omega6_g: 0.9,
        },
        allergens: vec![Allergen::Fish],
        dietary_flags: vec![DietaryFlag::GlutenFree, DietaryFlag::DairyFree],
        seasonality: None,
        cost_per_100g: Some(6.00),
        availability_score: 0.85,
        taste_profile: TasteProfile {
            sweetness: 0.1,
            saltiness: 0.2,
            sourness: 0.0,
            bitterness: 0.0,
            umami: 0.8,
            spiciness: 0.0,
        },
    });

    // Eggs
    foods.insert("eggs".to_string(), Food {
        id: "eggs".to_string(),
        name: "Large Eggs".to_string(),
        category: FoodCategory::Protein,
        nutrition_per_100g: NutritionFacts {
            calories: 155.0,
            protein_g: 13.0,
            carbs_g: 1.1,
            fat_g: 11.0,
            fiber_g: 0.0,
            sugar_g: 1.1,
            sodium_mg: 124.0,
            potassium_mg: 138.0,
            calcium_mg: 50.0,
            iron_mg: 1.2,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 87.0,
            vitamin_b12_mcg: 1.1,
            folate_mcg: 44.0,
            omega3_g: 0.1,
            omega6_g: 1.4,
        },
        allergens: vec![Allergen::Eggs],
        dietary_flags: vec![DietaryFlag::GlutenFree, DietaryFlag::Vegetarian],
        seasonality: None,
        cost_per_100g: Some(1.20),
        availability_score: 0.98,
        taste_profile: TasteProfile {
            sweetness: 0.0,
            saltiness: 0.2,
            sourness: 0.0,
            bitterness: 0.0,
            umami: 0.4,
            spiciness: 0.0,
        },
    });

    // === GRAINS ===

    // Brown Rice
    foods.insert("brown_rice".to_string(), Food {
        id: "brown_rice".to_string(),
        name: "Brown Rice (cooked)".to_string(),
        category: FoodCategory::Grains,
        nutrition_per_100g: NutritionFacts {
            calories: 111.0,
            protein_g: 2.6,
            carbs_g: 23.0,
            fat_g: 0.9,
            fiber_g: 1.8,
            sugar_g: 0.4,
            sodium_mg: 5.0,
            potassium_mg: 43.0,
            calcium_mg: 10.0,
            iron_mg: 0.4,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 4.0,
            omega3_g: 0.0,
            omega6_g: 0.2,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::GlutenFree, DietaryFlag::Vegan, DietaryFlag::Vegetarian],
        seasonality: None,
        cost_per_100g: Some(0.30),
        availability_score: 0.95,
        taste_profile: TasteProfile {
            sweetness: 0.1,
            saltiness: 0.0,
            sourness: 0.0,
            bitterness: 0.0,
            umami: 0.2,
            spiciness: 0.0,
        },
    });

    // Oats
    foods.insert("oats".to_string(), Food {
        id: "oats".to_string(),
        name: "Rolled Oats (dry)".to_string(),
        category: FoodCategory::Grains,
        nutrition_per_100g: NutritionFacts {
            calories: 389.0,
            protein_g: 16.9,
            carbs_g: 66.3,
            fat_g: 6.9,
            fiber_g: 10.6,
            sugar_g: 0.0,
            sodium_mg: 2.0,
            potassium_mg: 429.0,
            calcium_mg: 54.0,
            iron_mg: 4.7,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 56.0,
            omega3_g: 0.1,
            omega6_g: 2.4,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan],
        seasonality: None,
        cost_per_100g: Some(0.25),
        availability_score: 0.98,
        taste_profile: TasteProfile {
            sweetness: 0.1,
            saltiness: 0.0,
            sourness: 0.0,
            bitterness: 0.0,
            umami: 0.1,
            spiciness: 0.0,
        },
    });

    // === VEGETABLES ===

    // Broccoli
    foods.insert("broccoli".to_string(), Food {
        id: "broccoli".to_string(),
        name: "Broccoli (fresh)".to_string(),
        category: FoodCategory::Vegetables,
        nutrition_per_100g: NutritionFacts {
            calories: 34.0,
            protein_g: 2.8,
            carbs_g: 7.0,
            fat_g: 0.4,
            fiber_g: 2.6,
            sugar_g: 1.5,
            sodium_mg: 33.0,
            potassium_mg: 316.0,
            calcium_mg: 47.0,
            iron_mg: 0.7,
            vitamin_c_mg: 89.2,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 63.0,
            omega3_g: 0.1,
            omega6_g: 0.1,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree],
        seasonality: Some(Seasonality {
            peak_months: vec![10, 11, 12, 1, 2, 3],
            available_months: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            quality_by_month: {
                let mut quality = HashMap::new();
                for month in 1..=12 {
                    let quality_score = if [10, 11, 12, 1, 2, 3].contains(&month) { 0.9 } else { 0.6 };
                    quality.insert(month, quality_score);
                }
                quality
            },
        }),
        cost_per_100g: Some(0.80),
        availability_score: 0.90,
        taste_profile: TasteProfile {
            sweetness: 0.2,
            saltiness: 0.0,
            sourness: 0.0,
            bitterness: 0.3,
            umami: 0.2,
            spiciness: 0.0,
        },
    });

    // Spinach
    foods.insert("spinach".to_string(), Food {
        id: "spinach".to_string(),
        name: "Fresh Spinach".to_string(),
        category: FoodCategory::Vegetables,
        nutrition_per_100g: NutritionFacts {
            calories: 23.0,
            protein_g: 2.9,
            carbs_g: 3.6,
            fat_g: 0.4,
            fiber_g: 2.2,
            sugar_g: 0.4,
            sodium_mg: 79.0,
            potassium_mg: 558.0,
            calcium_mg: 99.0,
            iron_mg: 2.7,
            vitamin_c_mg: 28.1,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 194.0,
            omega3_g: 0.1,
            omega6_g: 0.1,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree],
        seasonality: Some(Seasonality {
            peak_months: vec![3, 4, 5, 9, 10, 11],
            available_months: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            quality_by_month: {
                let mut quality = HashMap::new();
                for month in 1..=12 {
                    let quality_score = if [3, 4, 5, 9, 10, 11].contains(&month) { 0.95 } else { 0.7 };
                    quality.insert(month, quality_score);
                }
                quality
            },
        }),
        cost_per_100g: Some(1.20),
        availability_score: 0.85,
        taste_profile: TasteProfile {
            sweetness: 0.1,
            saltiness: 0.1,
            sourness: 0.0,
            bitterness: 0.2,
            umami: 0.3,
            spiciness: 0.0,
        },
    });

    // === FRUITS ===

    // Banana
    foods.insert("banana".to_string(), Food {
        id: "banana".to_string(),
        name: "Banana (medium)".to_string(),
        category: FoodCategory::Fruits,
        nutrition_per_100g: NutritionFacts {
            calories: 89.0,
            protein_g: 1.1,
            carbs_g: 23.0,
            fat_g: 0.3,
            fiber_g: 2.6,
            sugar_g: 12.2,
            sodium_mg: 1.0,
            potassium_mg: 358.0,
            calcium_mg: 5.0,
            iron_mg: 0.3,
            vitamin_c_mg: 8.7,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 20.0,
            omega3_g: 0.0,
            omega6_g: 0.1,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree],
        seasonality: None, // Available year-round (imported)
        cost_per_100g: Some(0.40),
        availability_score: 0.98,
        taste_profile: TasteProfile {
            sweetness: 0.8,
            saltiness: 0.0,
            sourness: 0.0,
            bitterness: 0.0,
            umami: 0.0,
            spiciness: 0.0,
        },
    });

    // Blueberries
    foods.insert("blueberries".to_string(), Food {
        id: "blueberries".to_string(),
        name: "Fresh Blueberries".to_string(),
        category: FoodCategory::Fruits,
        nutrition_per_100g: NutritionFacts {
            calories: 57.0,
            protein_g: 0.7,
            carbs_g: 14.5,
            fat_g: 0.3,
            fiber_g: 2.4,
            sugar_g: 10.0,
            sodium_mg: 1.0,
            potassium_mg: 77.0,
            calcium_mg: 6.0,
            iron_mg: 0.3,
            vitamin_c_mg: 9.7,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 6.0,
            omega3_g: 0.1,
            omega6_g: 0.1,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree],
        seasonality: Some(Seasonality {
            peak_months: vec![6, 7, 8],
            available_months: vec![5, 6, 7, 8, 9],
            quality_by_month: {
                let mut quality = HashMap::new();
                for month in 1..=12 {
                    let quality_score = match month {
                        6 | 7 | 8 => 0.95,
                        5 | 9 => 0.7,
                        _ => 0.3, // Frozen/imported
                    };
                    quality.insert(month, quality_score);
                }
                quality
            },
        }),
        cost_per_100g: Some(3.00),
        availability_score: 0.70,
        taste_profile: TasteProfile {
            sweetness: 0.7,
            saltiness: 0.0,
            sourness: 0.3,
            bitterness: 0.1,
            umami: 0.0,
            spiciness: 0.0,
        },
    });

    // === DAIRY ===

    // Greek Yogurt
    foods.insert("greek_yogurt".to_string(), Food {
        id: "greek_yogurt".to_string(),
        name: "Plain Greek Yogurt (non-fat)".to_string(),
        category: FoodCategory::Dairy,
        nutrition_per_100g: NutritionFacts {
            calories: 59.0,
            protein_g: 10.0,
            carbs_g: 3.6,
            fat_g: 0.4,
            fiber_g: 0.0,
            sugar_g: 3.6,
            sodium_mg: 36.0,
            potassium_mg: 141.0,
            calcium_mg: 110.0,
            iron_mg: 0.0,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.5,
            folate_mcg: 7.0,
            omega3_g: 0.0,
            omega6_g: 0.0,
        },
        allergens: vec![Allergen::Dairy],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::GlutenFree],
        seasonality: None,
        cost_per_100g: Some(1.50),
        availability_score: 0.95,
        taste_profile: TasteProfile {
            sweetness: 0.2,
            saltiness: 0.1,
            sourness: 0.6,
            bitterness: 0.0,
            umami: 0.1,
            spiciness: 0.0,
        },
    });

    // === NUTS ===

    // Almonds
    foods.insert("almonds".to_string(), Food {
        id: "almonds".to_string(),
        name: "Raw Almonds".to_string(),
        category: FoodCategory::Nuts,
        nutrition_per_100g: NutritionFacts {
            calories: 579.0,
            protein_g: 21.2,
            carbs_g: 21.6,
            fat_g: 49.9,
            fiber_g: 12.5,
            sugar_g: 4.4,
            sodium_mg: 1.0,
            potassium_mg: 733.0,
            calcium_mg: 269.0,
            iron_mg: 3.7,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 44.0,
            omega3_g: 0.0,
            omega6_g: 12.3,
        },
        allergens: vec![Allergen::TreeNuts],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree, DietaryFlag::Keto, DietaryFlag::Paleo],
        seasonality: None,
        cost_per_100g: Some(8.00),
        availability_score: 0.95,
        taste_profile: TasteProfile {
            sweetness: 0.3,
            saltiness: 0.0,
            sourness: 0.0,
            bitterness: 0.2,
            umami: 0.2,
            spiciness: 0.0,
        },
    });

    // === LEGUMES ===

    // Black Beans
    foods.insert("black_beans".to_string(), Food {
        id: "black_beans".to_string(),
        name: "Black Beans (cooked)".to_string(),
        category: FoodCategory::Legumes,
        nutrition_per_100g: NutritionFacts {
            calories: 132.0,
            protein_g: 8.9,
            carbs_g: 23.7,
            fat_g: 0.5,
            fiber_g: 8.7,
            sugar_g: 0.3,
            sodium_mg: 2.0,
            potassium_mg: 355.0,
            calcium_mg: 23.0,
            iron_mg: 2.1,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 149.0,
            omega3_g: 0.1,
            omega6_g: 0.1,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree],
        seasonality: None,
        cost_per_100g: Some(0.40),
        availability_score: 0.95,
        taste_profile: TasteProfile {
            sweetness: 0.1,
            saltiness: 0.0,
            sourness: 0.0,
            bitterness: 0.1,
            umami: 0.4,
            spiciness: 0.0,
        },
    });

    // === OILS ===

    // Olive Oil
    foods.insert("olive_oil".to_string(), Food {
        id: "olive_oil".to_string(),
        name: "Extra Virgin Olive Oil".to_string(),
        category: FoodCategory::Oils,
        nutrition_per_100g: NutritionFacts {
            calories: 884.0,
            protein_g: 0.0,
            carbs_g: 0.0,
            fat_g: 100.0,
            fiber_g: 0.0,
            sugar_g: 0.0,
            sodium_mg: 2.0,
            potassium_mg: 1.0,
            calcium_mg: 1.0,
            iron_mg: 0.6,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 0.0,
            omega3_g: 0.8,
            omega6_g: 9.8,
        },
        allergens: vec![],
        dietary_flags: vec![DietaryFlag::Vegetarian, DietaryFlag::Vegan, DietaryFlag::GlutenFree, DietaryFlag::Keto, DietaryFlag::Paleo],
        seasonality: None,
        cost_per_100g: Some(12.00),
        availability_score: 0.95,
        taste_profile: TasteProfile {
            sweetness: 0.0,
            saltiness: 0.0,
            sourness: 0.1,
            bitterness: 0.3,
            umami: 0.0,
            spiciness: 0.0,
        },
    });

    foods
}

pub fn get_sample_food_count() -> usize {
    create_sample_foods().len()
}