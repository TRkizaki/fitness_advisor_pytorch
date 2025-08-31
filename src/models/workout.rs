use serde::{Deserialize, Serialize};
use crate::models::exercise::ExerciseSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSession {
    pub id: String,
    pub user_id: String,
    pub date: String,
    pub exercises: Vec<ExerciseSet>,
    pub total_duration_minutes: u32,
    pub calories_burned: Option<f32>,
    pub user_rating: Option<u32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressAnalysis {
    pub total_workouts: u32,
    pub average_duration_minutes: f32,
    pub total_calories_burned: f32,
    pub consistency_score: f32,
}