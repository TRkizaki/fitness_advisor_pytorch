// src/models/food.rs - Food and nutrition data models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Food {
    pub id: String,
    pub name: String,
    pub category: FoodCategory,
    pub nutrition_per_100g: NutritionFacts,
    pub allergens: Vec<Allergen>,
    pub dietary_flags: Vec<DietaryFlag>,
    pub seasonality: Option<Seasonality>,
    pub cost_per_100g: Option<f64>, // In local currency
    pub availability_score: f64,    // 0.0 to 1.0
    pub taste_profile: TasteProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NutritionFacts {
    pub calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
    pub sugar_g: f64,
    pub sodium_mg: f64,
    pub potassium_mg: f64,
    pub calcium_mg: f64,
    pub iron_mg: f64,
    pub vitamin_c_mg: f64,
    pub vitamin_d_iu: f64,
    pub vitamin_b12_mcg: f64,
    pub folate_mcg: f64,
    pub omega3_g: f64,
    pub omega6_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FoodCategory {
    Protein,
    Grains,
    Vegetables,
    Fruits,
    Dairy,
    Nuts,
    Legumes,
    Oils,
    Spices,
    Beverages,
    Supplements,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Allergen {
    Gluten,
    Dairy,
    Eggs,
    Fish,
    Shellfish,
    TreeNuts,
    Peanuts,
    Soy,
    Sesame,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DietaryFlag {
    Vegetarian,
    Vegan,
    Kosher,
    Halal,
    GlutenFree,
    DairyFree,
    LowCarb,
    Keto,
    Paleo,
    Organic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Seasonality {
    pub peak_months: Vec<u8>,     // 1-12 for months
    pub available_months: Vec<u8>, // 1-12 for months
    pub quality_by_month: HashMap<u8, f64>, // Month -> quality score (0.0-1.0)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TasteProfile {
    pub sweetness: f64,     // 0.0 to 1.0
    pub saltiness: f64,     // 0.0 to 1.0
    pub sourness: f64,      // 0.0 to 1.0
    pub bitterness: f64,    // 0.0 to 1.0
    pub umami: f64,         // 0.0 to 1.0
    pub spiciness: f64,     // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: u32,
    pub cook_time_minutes: u32,
    pub servings: u32,
    pub difficulty: DifficultyLevel,
    pub cuisine_type: Option<String>,
    pub meal_type: MealType,
    pub nutrition_per_serving: NutritionFacts,
    pub allergens: Vec<Allergen>,
    pub dietary_flags: Vec<DietaryFlag>,
    pub rating: Option<f64>,
    pub cost_per_serving: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub food_id: String,
    pub amount_g: f64,
    pub preparation: Option<String>, // "chopped", "cooked", "raw", etc.
    pub substitutes: Vec<String>,    // Alternative food IDs
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Dessert,
    Beverage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealPlan {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub date_range: DateRange,
    pub meals: Vec<PlannedMeal>,
    pub total_nutrition: NutritionFacts,
    pub total_cost: Option<f64>,
    pub optimization_score: f64,
    pub constraints_met: bool,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedMeal {
    pub date: chrono::NaiveDate,
    pub meal_type: MealType,
    pub recipe: Recipe,
    pub portion_size: f64, // Multiplier for recipe servings
    pub nutrition: NutritionFacts,
    pub cost: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
}

impl Food {
    pub fn get_nutrition_for_amount(&self, grams: f64) -> NutritionFacts {
        let multiplier = grams / 100.0;
        NutritionFacts {
            calories: self.nutrition_per_100g.calories * multiplier,
            protein_g: self.nutrition_per_100g.protein_g * multiplier,
            carbs_g: self.nutrition_per_100g.carbs_g * multiplier,
            fat_g: self.nutrition_per_100g.fat_g * multiplier,
            fiber_g: self.nutrition_per_100g.fiber_g * multiplier,
            sugar_g: self.nutrition_per_100g.sugar_g * multiplier,
            sodium_mg: self.nutrition_per_100g.sodium_mg * multiplier,
            potassium_mg: self.nutrition_per_100g.potassium_mg * multiplier,
            calcium_mg: self.nutrition_per_100g.calcium_mg * multiplier,
            iron_mg: self.nutrition_per_100g.iron_mg * multiplier,
            vitamin_c_mg: self.nutrition_per_100g.vitamin_c_mg * multiplier,
            vitamin_d_iu: self.nutrition_per_100g.vitamin_d_iu * multiplier,
            vitamin_b12_mcg: self.nutrition_per_100g.vitamin_b12_mcg * multiplier,
            folate_mcg: self.nutrition_per_100g.folate_mcg * multiplier,
            omega3_g: self.nutrition_per_100g.omega3_g * multiplier,
            omega6_g: self.nutrition_per_100g.omega6_g * multiplier,
        }
    }

    pub fn is_compatible_with_diet(&self, dietary_requirements: &[DietaryFlag]) -> bool {
        dietary_requirements.iter().all(|req| self.dietary_flags.contains(req))
    }

    pub fn has_allergen(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn get_seasonal_quality(&self, month: u8) -> f64 {
        self.seasonality
            .as_ref()
            .and_then(|s| s.quality_by_month.get(&month))
            .copied()
            .unwrap_or(0.5) // Default quality if no seasonal data
    }
}

impl NutritionFacts {
    pub fn new() -> Self {
        Self {
            calories: 0.0,
            protein_g: 0.0,
            carbs_g: 0.0,
            fat_g: 0.0,
            fiber_g: 0.0,
            sugar_g: 0.0,
            sodium_mg: 0.0,
            potassium_mg: 0.0,
            calcium_mg: 0.0,
            iron_mg: 0.0,
            vitamin_c_mg: 0.0,
            vitamin_d_iu: 0.0,
            vitamin_b12_mcg: 0.0,
            folate_mcg: 0.0,
            omega3_g: 0.0,
            omega6_g: 0.0,
        }
    }

    pub fn add(&mut self, other: &NutritionFacts) {
        self.calories += other.calories;
        self.protein_g += other.protein_g;
        self.carbs_g += other.carbs_g;
        self.fat_g += other.fat_g;
        self.fiber_g += other.fiber_g;
        self.sugar_g += other.sugar_g;
        self.sodium_mg += other.sodium_mg;
        self.potassium_mg += other.potassium_mg;
        self.calcium_mg += other.calcium_mg;
        self.iron_mg += other.iron_mg;
        self.vitamin_c_mg += other.vitamin_c_mg;
        self.vitamin_d_iu += other.vitamin_d_iu;
        self.vitamin_b12_mcg += other.vitamin_b12_mcg;
        self.folate_mcg += other.folate_mcg;
        self.omega3_g += other.omega3_g;
        self.omega6_g += other.omega6_g;
    }

    pub fn get_macro_ratio(&self) -> (f64, f64, f64) {
        let protein_cal = self.protein_g * 4.0;
        let carbs_cal = self.carbs_g * 4.0;
        let fat_cal = self.fat_g * 9.0;
        let total_cal = protein_cal + carbs_cal + fat_cal;

        if total_cal == 0.0 {
            return (0.0, 0.0, 0.0);
        }

        (
            protein_cal / total_cal,
            carbs_cal / total_cal,
            fat_cal / total_cal,
        )
    }

    pub fn calculate_nutrition_score(&self) -> f64 {
        // Simple nutrition quality score based on micronutrient density
        let micronutrient_score = (
            (self.vitamin_c_mg / 90.0).min(1.0) +
            (self.calcium_mg / 1000.0).min(1.0) +
            (self.iron_mg / 18.0).min(1.0) +
            (self.folate_mcg / 400.0).min(1.0) +
            (self.fiber_g / 25.0).min(1.0)
        ) / 5.0;

        // Penalty for excess sodium and sugar
        let sodium_penalty = if self.sodium_mg > 2300.0 {
            (self.sodium_mg - 2300.0) / 2300.0 * 0.3
        } else { 0.0 };

        let sugar_penalty = if self.sugar_g > 50.0 {
            (self.sugar_g - 50.0) / 50.0 * 0.2
        } else { 0.0 };

        (micronutrient_score - sodium_penalty - sugar_penalty).max(0.0).min(1.0)
    }
}

impl Default for NutritionFacts {
    fn default() -> Self {
        Self::new()
    }
}

impl TasteProfile {
    pub fn new() -> Self {
        Self {
            sweetness: 0.0,
            saltiness: 0.0,
            sourness: 0.0,
            bitterness: 0.0,
            umami: 0.0,
            spiciness: 0.0,
        }
    }

    pub fn similarity(&self, other: &TasteProfile) -> f64 {
        let diff = (self.sweetness - other.sweetness).abs() +
                  (self.saltiness - other.saltiness).abs() +
                  (self.sourness - other.sourness).abs() +
                  (self.bitterness - other.bitterness).abs() +
                  (self.umami - other.umami).abs() +
                  (self.spiciness - other.spiciness).abs();
        
        1.0 - (diff / 6.0) // Normalize to 0-1 range
    }
}

impl Default for TasteProfile {
    fn default() -> Self {
        Self::new()
    }
}