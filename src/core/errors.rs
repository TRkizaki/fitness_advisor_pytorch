// src/core/errors.rs - Core error types for the fitness advisor system

use thiserror::Error;

/// Core system errors
#[derive(Error, Debug)]
pub enum FitnessError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Optimization error: {0}")]
    Optimization(String),
    
    #[error("Nutrition analysis error: {0}")]
    Nutrition(String),
    
    #[error("User not found: {id}")]
    UserNotFound { id: String },
    
    #[error("Food not found: {id}")]
    FoodNotFound { id: String },
    
    #[error("Recipe not found: {id}")]
    RecipeNotFound { id: String },
    
    #[error("Invalid nutritional data: {reason}")]
    InvalidNutrition { reason: String },
    
    #[error("Optimization constraint violated: {constraint}")]
    ConstraintViolation { constraint: String },
    
    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, FitnessError>;

impl FitnessError {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
    
    pub fn optimization(msg: impl Into<String>) -> Self {
        Self::Optimization(msg.into())
    }
    
    pub fn nutrition(msg: impl Into<String>) -> Self {
        Self::Nutrition(msg.into())
    }
    
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
    
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}