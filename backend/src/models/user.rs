use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub height: f32,
    pub weight: f32,
    pub fitness_level: FitnessLevel,
    pub goals: Vec<FitnessGoal>,
    pub preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FitnessLevel {
    Beginner,
    Intermediate,
    Advanced,
    Elite,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FitnessGoal {
    WeightLoss,
    MuscleGain,
    Endurance,
    Strength,
    Flexibility,
    GeneralHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_exercise_types: Vec<ExerciseType>,
    pub available_equipment: Vec<Equipment>,
    pub workout_duration_minutes: u32,
    pub workouts_per_week: u32,
    pub preferred_time_of_day: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExerciseType {
    Cardio,
    Strength,
    Flexibility,
    Balance,
    Sports,
    Yoga,
    Pilates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Equipment {
    None,
    Dumbbells,
    Barbells,
    ResistanceBands,
    PullUpBar,
    Bench,
    TreadMill,
    StationaryBike,
}