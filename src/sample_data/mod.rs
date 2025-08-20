// src/sample_data/mod.rs - Sample data management module

pub mod foods;
pub mod recipes;

pub use foods::{create_sample_foods, get_sample_food_count};
pub use recipes::{create_sample_recipes, get_sample_recipe_count};

use crate::models::food::{Food, Recipe};
use std::collections::HashMap;

/// Complete sample data package for menu optimization
pub struct SampleDataSet {
    pub foods: HashMap<String, Food>,
    pub recipes: Vec<Recipe>,
}

impl SampleDataSet {
    /// Create a complete sample dataset for testing and development
    pub fn new() -> Self {
        Self {
            foods: create_sample_foods(),
            recipes: create_sample_recipes(),
        }
    }

    /// Get total count of sample data items
    pub fn get_counts(&self) -> (usize, usize) {
        (self.foods.len(), self.recipes.len())
    }

    /// Validate that all recipe ingredients exist in the food database
    pub fn validate_data(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for recipe in &self.recipes {
            for ingredient in &recipe.ingredients {
                if !self.foods.contains_key(&ingredient.food_id) {
                    errors.push(format!(
                        "Recipe '{}' references missing food '{}'", 
                        recipe.name, 
                        ingredient.food_id
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get recipes by meal type
    pub fn get_recipes_by_meal_type(&self, meal_type: &crate::models::food::MealType) -> Vec<&Recipe> {
        self.recipes
            .iter()
            .filter(|recipe| &recipe.meal_type == meal_type)
            .collect()
    }

    /// Get foods by category
    pub fn get_foods_by_category(&self, category: &crate::models::food::FoodCategory) -> Vec<&Food> {
        self.foods
            .values()
            .filter(|food| &food.category == category)
            .collect()
    }
}

impl Default for SampleDataSet {
    fn default() -> Self {
        Self::new()
    }
}