// src/advisors/menu_optimizer/data_loader.rs - Sample data loading service

use crate::sample_data::SampleDataSet;
use crate::advisors::menu_optimizer::MenuOptimizer;
use crate::core::{FitnessError, Result};
use crate::models::food::{Food, DietaryFlag};
use std::collections::HashMap;
use tracing::{info, error};

/// Service for loading sample data into the menu optimizer
pub struct DataLoader;

impl DataLoader {
    /// Load sample data into a new menu optimizer instance
    pub async fn load_sample_data() -> Result<MenuOptimizer> {
        info!("Loading sample data for menu optimizer...");
        
        let sample_data = SampleDataSet::new();
        let (food_count, recipe_count) = sample_data.get_counts();
        
        info!("Created sample dataset with {} foods and {} recipes", food_count, recipe_count);
        
        // Validate the sample data
        if let Err(errors) = sample_data.validate_data() {
            error!("Sample data validation failed:");
            for error in &errors {
                error!("  - {}", error);
            }
            return Err(FitnessError::validation(format!(
                "Sample data validation failed: {} errors found", 
                errors.len()
            )));
        }
        
        info!("Sample data validation passed");
        
        // Create optimizer with sample data
        let optimizer = MenuOptimizer::with_data(
            sample_data.recipes,
            sample_data.foods
        );
        
        info!("Menu optimizer initialized with sample data");
        Ok(optimizer)
    }
    
    /// Load sample data into an existing menu optimizer
    pub async fn add_sample_data_to_optimizer(optimizer: &MenuOptimizer) -> Result<()> {
        info!("Adding sample data to existing menu optimizer...");
        
        let sample_data = SampleDataSet::new();
        let (food_count, recipe_count) = sample_data.get_counts();
        
        // Validate the sample data first
        if let Err(errors) = sample_data.validate_data() {
            error!("Sample data validation failed:");
            for error in &errors {
                error!("  - {}", error);
            }
            return Err(FitnessError::validation(format!(
                "Sample data validation failed: {} errors found", 
                errors.len()
            )));
        }
        
        // Add the data to the optimizer
        optimizer.add_foods(sample_data.foods).await?;
        optimizer.add_recipes(sample_data.recipes).await?;
        
        info!("Successfully added {} foods and {} recipes to optimizer", food_count, recipe_count);
        Ok(())
    }
    
    /// Get sample data statistics without loading
    pub fn get_sample_data_info() -> (usize, usize) {
        let sample_data = SampleDataSet::new();
        sample_data.get_counts()
    }
    
    /// Create a menu optimizer for testing with minimal data
    pub async fn create_test_optimizer() -> Result<MenuOptimizer> {
        info!("Creating test menu optimizer with minimal sample data...");
        
        let sample_data = SampleDataSet::new();
        
        // Take just a few items for testing
        let test_recipes: Vec<_> = sample_data.recipes.into_iter().take(3).collect();
        let test_foods: HashMap<String, Food> = sample_data.foods.into_iter().take(8).collect();
        
        let recipe_count = test_recipes.len();
        let food_count = test_foods.len();
        
        let optimizer = MenuOptimizer::with_data(test_recipes, test_foods);
        
        info!("Test menu optimizer created with {} recipes and {} foods", 
              recipe_count, food_count);
              
        Ok(optimizer)
    }
    
    /// Load data by category for specialized optimizations
    pub async fn load_data_by_dietary_preference(
        dietary_flags: &[crate::models::food::DietaryFlag]
    ) -> Result<MenuOptimizer> {
        info!("Loading sample data filtered by dietary preferences: {:?}", dietary_flags);
        
        let sample_data = SampleDataSet::new();
        
        // Filter recipes based on dietary flags
        let filtered_recipes: Vec<_> = sample_data.recipes
            .into_iter()
            .filter(|recipe| {
                // Recipe must have ALL requested dietary flags
                dietary_flags.iter().all(|flag| recipe.dietary_flags.contains(flag))
            })
            .collect();
            
        // Filter foods based on dietary flags  
        let filtered_foods: HashMap<String, Food> = sample_data.foods
            .into_iter()
            .filter(|(_, food)| {
                dietary_flags.iter().all(|flag| food.dietary_flags.contains(flag))
            })
            .collect();
            
        let recipe_count = filtered_recipes.len();
        let food_count = filtered_foods.len();
        
        if recipe_count == 0 {
            return Err(FitnessError::validation(
                "No recipes match the specified dietary preferences".to_string()
            ));
        }
        
        if food_count == 0 {
            return Err(FitnessError::validation(
                "No foods match the specified dietary preferences".to_string()
            ));
        }
        
        let optimizer = MenuOptimizer::with_data(filtered_recipes, filtered_foods);
        
        info!("Created specialized optimizer with {} recipes and {} foods for dietary preferences", 
              recipe_count, food_count);
              
        Ok(optimizer)
    }
    
    /// Verify data integrity for the optimizer
    pub async fn verify_optimizer_data(optimizer: &MenuOptimizer) -> Result<String> {
        let recipe_count = optimizer.get_recipe_count().await;
        let food_count = optimizer.get_food_count().await;
        
        if recipe_count == 0 {
            return Err(FitnessError::validation("No recipes loaded in optimizer".to_string()));
        }
        
        if food_count == 0 {
            return Err(FitnessError::validation("No foods loaded in optimizer".to_string()));
        }
        
        let status = format!(
            "Menu optimizer data verification passed:\n\
             - {} recipes loaded\n\
             - {} foods loaded\n\
             - Ready for optimization requests",
            recipe_count, food_count
        );
        
        info!("{}", status);
        Ok(status)
    }
}