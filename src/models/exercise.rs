use serde::{Deserialize, Serialize};
use crate::models::user::{ExerciseType, Equipment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exercise_type: ExerciseType,
    pub equipment_needed: Vec<Equipment>,
    pub difficulty_level: u32,
    pub primary_muscles: Vec<MuscleGroup>,
    pub secondary_muscles: Vec<MuscleGroup>,
    pub instructions: Vec<String>,
    pub safety_tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MuscleGroup {
    Chest,
    Back,
    Shoulders,
    Arms,
    Core,
    Legs,
    Glutes,
    Calves,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseSet {
    pub exercise_id: String,
    pub sets: u32,
    pub reps: u32,
    pub weight_kg: Option<f32>,
    pub duration_seconds: Option<u32>,
    pub rest_seconds: u32,
    pub completed: bool,
}