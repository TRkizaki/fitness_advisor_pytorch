// src/database.rs - Database integration module

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row, SqlitePool as Pool};
use std::str::FromStr;
use tracing::{info, warn};

use crate::{
    User, Exercise, WorkoutSession, ExerciseSet, ProgressAnalysis,
    FitnessLevel, FitnessGoal, ExerciseType, Equipment, MuscleGroup, UserPreferences
};

// Database connection and management
pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    // Initialize database connection and create tables
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("🗄️  Connecting to database: {}", database_url);
        
        let pool = SqlitePool::connect(database_url).await?;
        
        let manager = Self { pool };
        manager.create_tables().await?;
        manager.seed_exercises().await?;
        
        info!("✅ Database initialized successfully");
        Ok(manager)
    }

    // Create all necessary tables
    async fn create_tables(&self) -> Result<()> {
        info!("📋 Creating database tables...");

        // Users table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                age INTEGER NOT NULL,
                height REAL NOT NULL,
                weight REAL NOT NULL,
                fitness_level TEXT NOT NULL,
                goals TEXT NOT NULL, -- JSON array
                preferences TEXT NOT NULL, -- JSON object
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        "#).execute(&self.pool).await?;

        // Exercises table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS exercises (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                exercise_type TEXT NOT NULL,
                equipment_needed TEXT NOT NULL, -- JSON array
                difficulty_level INTEGER NOT NULL,
                primary_muscles TEXT NOT NULL, -- JSON array
                secondary_muscles TEXT NOT NULL, -- JSON array
                instructions TEXT NOT NULL, -- JSON array
                safety_tips TEXT NOT NULL, -- JSON array
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        "#).execute(&self.pool).await?;

        // Workout sessions table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS workout_sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                date TEXT NOT NULL,
                total_duration_minutes INTEGER NOT NULL,
                calories_burned REAL,
                user_rating INTEGER,
                notes TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
        "#).execute(&self.pool).await?;

        // Exercise sets table (many-to-many: sessions <-> exercises)
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS exercise_sets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                workout_session_id TEXT NOT NULL,
                exercise_id TEXT NOT NULL,
                sets INTEGER NOT NULL,
                reps INTEGER NOT NULL,
                weight_kg REAL,
                duration_seconds INTEGER,
                rest_seconds INTEGER NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE,
                FOREIGN KEY (workout_session_id) REFERENCES workout_sessions (id),
                FOREIGN KEY (exercise_id) REFERENCES exercises (id)
            )
        "#).execute(&self.pool).await?;

        // User progress tracking table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS user_progress (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL,
                date TEXT NOT NULL,
                weight_kg REAL,
                body_fat_percentage REAL,
                muscle_mass_kg REAL,
                notes TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
        "#).execute(&self.pool).await?;

        info!("✅ All tables created successfully");
        Ok(())
    }

    // Seed initial exercise data
    async fn seed_exercises(&self) -> Result<()> {
        // Check if exercises already exist
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM exercises")
            .fetch_one(&self.pool).await?;

        if count > 0 {
            info!("📚 Exercises already seeded, skipping...");
            return Ok(());
        }

        info!("🌱 Seeding initial exercises...");

        let exercises = vec![
            Exercise {
                id: "pushup".to_string(),
                name: "Push-up".to_string(),
                description: "Classic bodyweight chest exercise".to_string(),
                exercise_type: ExerciseType::Strength,
                equipment_needed: vec![Equipment::None],
                difficulty_level: 3,
                primary_muscles: vec![MuscleGroup::Chest],
                secondary_muscles: vec![MuscleGroup::Arms, MuscleGroup::Core],
                instructions: vec![
                    "Start in plank position".to_string(),
                    "Lower body until chest nearly touches floor".to_string(),
                    "Push back up to starting position".to_string(),
                ],
                safety_tips: vec![
                    "Keep body straight".to_string(),
                    "Don't let hips sag".to_string(),
                ],
            },
            Exercise {
                id: "squat".to_string(),
                name: "Squat".to_string(),
                description: "Fundamental lower body exercise".to_string(),
                exercise_type: ExerciseType::Strength,
                equipment_needed: vec![Equipment::None],
                difficulty_level: 2,
                primary_muscles: vec![MuscleGroup::Legs, MuscleGroup::Glutes],
                secondary_muscles: vec![MuscleGroup::Core],
                instructions: vec![
                    "Stand with feet shoulder-width apart".to_string(),
                    "Lower hips back and down".to_string(),
                    "Return to standing position".to_string(),
                ],
                safety_tips: vec![
                    "Keep knees behind toes".to_string(),
                    "Maintain neutral spine".to_string(),
                ],
            },
            Exercise {
                id: "plank".to_string(),
                name: "Plank".to_string(),
                description: "Core strengthening exercise".to_string(),
                exercise_type: ExerciseType::Strength,
                equipment_needed: vec![Equipment::None],
                difficulty_level: 3,
                primary_muscles: vec![MuscleGroup::Core],
                secondary_muscles: vec![MuscleGroup::Shoulders, MuscleGroup::Back],
                instructions: vec![
                    "Start in push-up position".to_string(),
                    "Keep body straight from head to heels".to_string(),
                    "Hold position".to_string(),
                ],
                safety_tips: vec![
                    "Don't let hips sag or rise".to_string(),
                    "Breathe normally".to_string(),
                ],
            },
            // Add more exercises
            Exercise {
                id: "burpee".to_string(),
                name: "Burpee".to_string(),
                description: "Full body explosive exercise".to_string(),
                exercise_type: ExerciseType::Cardio,
                equipment_needed: vec![Equipment::None],
                difficulty_level: 8,
                primary_muscles: vec![MuscleGroup::Legs, MuscleGroup::Core],
                secondary_muscles: vec![MuscleGroup::Chest, MuscleGroup::Arms],
                instructions: vec![
                    "Start standing".to_string(),
                    "Drop to squat, hands on floor".to_string(),
                    "Jump feet back to plank".to_string(),
                    "Do a push-up".to_string(),
                    "Jump feet forward".to_string(),
                    "Explosive jump up".to_string(),
                ],
                safety_tips: vec![
                    "Land softly".to_string(),
                    "Keep core tight throughout".to_string(),
                    "Start slowly and build intensity".to_string(),
                ],
            },
            Exercise {
                id: "deadlift".to_string(),
                name: "Deadlift".to_string(),
                description: "Compound pulling exercise".to_string(),
                exercise_type: ExerciseType::Strength,
                equipment_needed: vec![Equipment::Barbells],
                difficulty_level: 7,
                primary_muscles: vec![MuscleGroup::Back, MuscleGroup::Legs],
                secondary_muscles: vec![MuscleGroup::Core, MuscleGroup::Arms],
                instructions: vec![
                    "Stand with feet hip-width apart".to_string(),
                    "Grip barbell with hands outside legs".to_string(),
                    "Keep back straight, chest up".to_string(),
                    "Drive through heels to lift".to_string(),
                    "Lower with control".to_string(),
                ],
                safety_tips: vec![
                    "Never round your back".to_string(),
                    "Start with light weight".to_string(),
                    "Keep bar close to body".to_string(),
                ],
            },
        ];

        for exercise in exercises {
            self.save_exercise(&exercise).await?;
        }

        info!("✅ {} exercises seeded successfully", 5);
        Ok(())
    }

    // === USER OPERATIONS ===

    pub async fn save_user(&self, user: &User) -> Result<()> {
        sqlx::query(r#"
            INSERT OR REPLACE INTO users 
            (id, name, age, height, weight, fitness_level, goals, preferences, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        "#)
        .bind(&user.id)
        .bind(&user.name)
        .bind(user.age as i64)
        .bind(user.height)
        .bind(user.weight)
        .bind(serde_json::to_string(&user.fitness_level)?)
        .bind(serde_json::to_string(&user.goals)?)
        .bind(serde_json::to_string(&user.preferences)?)
        .execute(&self.pool).await?;

        info!("💾 User {} saved to database", user.id);
        Ok(())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<User>> {
        let row = sqlx::query(r#"
            SELECT id, name, age, height, weight, fitness_level, goals, preferences
            FROM users WHERE id = ?
        "#)
        .bind(user_id)
        .fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    name: row.get("name"),
                    age: row.get::<i64, _>("age") as u32,
                    height: row.get("height"),
                    weight: row.get("weight"),
                    fitness_level: serde_json::from_str(&row.get::<String, _>("fitness_level"))?,
                    goals: serde_json::from_str(&row.get::<String, _>("goals"))?,
                    preferences: serde_json::from_str(&row.get::<String, _>("preferences"))?,
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let rows = sqlx::query(r#"
            SELECT id, name, age, height, weight, fitness_level, goals, preferences
            FROM users ORDER BY created_at DESC
        "#)
        .fetch_all(&self.pool).await?;

        let mut users = Vec::new();
        for row in rows {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                age: row.get::<i64, _>("age") as u32,
                height: row.get("height"),
                weight: row.get("weight"),
                fitness_level: serde_json::from_str(&row.get::<String, _>("fitness_level"))?,
                goals: serde_json::from_str(&row.get::<String, _>("goals"))?,
                preferences: serde_json::from_str(&row.get::<String, _>("preferences"))?,
            };
            users.push(user);
        }

        Ok(users)
    }

    // === EXERCISE OPERATIONS ===

    pub async fn save_exercise(&self, exercise: &Exercise) -> Result<()> {
        sqlx::query(r#"
            INSERT OR REPLACE INTO exercises 
            (id, name, description, exercise_type, equipment_needed, difficulty_level, 
             primary_muscles, secondary_muscles, instructions, safety_tips)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(&exercise.id)
        .bind(&exercise.name)
        .bind(&exercise.description)
        .bind(serde_json::to_string(&exercise.exercise_type)?)
        .bind(serde_json::to_string(&exercise.equipment_needed)?)
        .bind(exercise.difficulty_level as i64)
        .bind(serde_json::to_string(&exercise.primary_muscles)?)
        .bind(serde_json::to_string(&exercise.secondary_muscles)?)
        .bind(serde_json::to_string(&exercise.instructions)?)
        .bind(serde_json::to_string(&exercise.safety_tips)?)
        .execute(&self.pool).await?;

        Ok(())
    }

    pub async fn get_exercise(&self, exercise_id: &str) -> Result<Option<Exercise>> {
        let row = sqlx::query(r#"
            SELECT id, name, description, exercise_type, equipment_needed, difficulty_level,
                   primary_muscles, secondary_muscles, instructions, safety_tips
            FROM exercises WHERE id = ?
        "#)
        .bind(exercise_id)
        .fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let exercise = Exercise {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    exercise_type: serde_json::from_str(&row.get::<String, _>("exercise_type"))?,
                    equipment_needed: serde_json::from_str(&row.get::<String, _>("equipment_needed"))?,
                    difficulty_level: row.get::<i64, _>("difficulty_level") as u32,
                    primary_muscles: serde_json::from_str(&row.get::<String, _>("primary_muscles"))?,
                    secondary_muscles: serde_json::from_str(&row.get::<String, _>("secondary_muscles"))?,
                    instructions: serde_json::from_str(&row.get::<String, _>("instructions"))?,
                    safety_tips: serde_json::from_str(&row.get::<String, _>("safety_tips"))?,
                };
                Ok(Some(exercise))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_exercises(&self) -> Result<Vec<Exercise>> {
        let rows = sqlx::query(r#"
            SELECT id, name, description, exercise_type, equipment_needed, difficulty_level,
                   primary_muscles, secondary_muscles, instructions, safety_tips
            FROM exercises ORDER BY name
        "#)
        .fetch_all(&self.pool).await?;

        let mut exercises = Vec::new();
        for row in rows {
            let exercise = Exercise {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                exercise_type: serde_json::from_str(&row.get::<String, _>("exercise_type"))?,
                equipment_needed: serde_json::from_str(&row.get::<String, _>("equipment_needed"))?,
                difficulty_level: row.get::<i64, _>("difficulty_level") as u32,
                primary_muscles: serde_json::from_str(&row.get::<String, _>("primary_muscles"))?,
                secondary_muscles: serde_json::from_str(&row.get::<String, _>("secondary_muscles"))?,
                instructions: serde_json::from_str(&row.get::<String, _>("instructions"))?,
                safety_tips: serde_json::from_str(&row.get::<String, _>("safety_tips"))?,
            };
            exercises.push(exercise);
        }

        Ok(exercises)
    }

    // === WORKOUT OPERATIONS ===

    pub async fn save_workout(&self, workout: &WorkoutSession) -> Result<()> {
        // Start transaction
        let mut tx = self.pool.begin().await?;

        // Insert workout session
        sqlx::query(r#"
            INSERT OR REPLACE INTO workout_sessions 
            (id, user_id, date, total_duration_minutes, calories_burned, user_rating, notes)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(&workout.id)
        .bind(&workout.user_id)
        .bind(&workout.date)
        .bind(workout.total_duration_minutes as i64)
        .bind(workout.calories_burned)
        .bind(workout.user_rating.map(|r| r as i64))
        .bind(&workout.notes)
        .execute(&mut *tx).await?;

        // Delete existing exercise sets for this workout
        sqlx::query("DELETE FROM exercise_sets WHERE workout_session_id = ?")
            .bind(&workout.id)
            .execute(&mut *tx).await?;

        // Insert exercise sets
        for exercise_set in &workout.exercises {
            sqlx::query(r#"
                INSERT INTO exercise_sets 
                (workout_session_id, exercise_id, sets, reps, weight_kg, duration_seconds, rest_seconds, completed)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#)
            .bind(&workout.id)
            .bind(&exercise_set.exercise_id)
            .bind(exercise_set.sets as i64)
            .bind(exercise_set.reps as i64)
            .bind(exercise_set.weight_kg)
            .bind(exercise_set.duration_seconds.map(|d| d as i64))
            .bind(exercise_set.rest_seconds as i64)
            .bind(exercise_set.completed)
            .execute(&mut *tx).await?;
        }

        // Commit transaction
        tx.commit().await?;

        info!("💾 Workout {} saved to database", workout.id);
        Ok(())
    }

    pub async fn get_user_workouts(&self, user_id: &str) -> Result<Vec<WorkoutSession>> {
        let rows = sqlx::query(r#"
            SELECT id, user_id, date, total_duration_minutes, calories_burned, user_rating, notes
            FROM workout_sessions 
            WHERE user_id = ? 
            ORDER BY date DESC
        "#)
        .bind(user_id)
        .fetch_all(&self.pool).await?;

        let mut workouts = Vec::new();
        for row in rows {
            let workout_id: String = row.get("id");
            
            // Get exercise sets for this workout
            let exercise_rows = sqlx::query(r#"
                SELECT exercise_id, sets, reps, weight_kg, duration_seconds, rest_seconds, completed
                FROM exercise_sets 
                WHERE workout_session_id = ?
            "#)
            .bind(&workout_id)
            .fetch_all(&self.pool).await?;

            let mut exercises = Vec::new();
            for ex_row in exercise_rows {
                let exercise_set = ExerciseSet {
                    exercise_id: ex_row.get("exercise_id"),
                    sets: ex_row.get::<i64, _>("sets") as u32,
                    reps: ex_row.get::<i64, _>("reps") as u32,
                    weight_kg: ex_row.get("weight_kg"),
                    duration_seconds: ex_row.get::<Option<i64>, _>("duration_seconds").map(|d| d as u32),
                    rest_seconds: ex_row.get::<i64, _>("rest_seconds") as u32,
                    completed: ex_row.get("completed"),
                };
                exercises.push(exercise_set);
            }

            let workout = WorkoutSession {
                id: workout_id,
                user_id: row.get("user_id"),
                date: row.get("date"),
                exercises,
                total_duration_minutes: row.get::<i64, _>("total_duration_minutes") as u32,
                calories_burned: row.get("calories_burned"),
                user_rating: row.get::<Option<i64>, _>("user_rating").map(|r| r as u32),
                notes: row.get("notes"),
            };
            workouts.push(workout);
        }

        Ok(workouts)
    }

    // === ANALYTICS ===

    pub async fn get_user_progress_analysis(&self, user_id: &str) -> Result<ProgressAnalysis> {
        let workouts = self.get_user_workouts(user_id).await?;
        
        let total_workouts = workouts.len() as u32;
        let avg_duration = if total_workouts > 0 {
            workouts.iter()
                .map(|w| w.total_duration_minutes as f32)
                .sum::<f32>() / total_workouts as f32
        } else {
            0.0
        };

        let total_calories = workouts.iter()
            .filter_map(|w| w.calories_burned)
            .sum::<f32>();

        // Calculate consistency score based on recent workout frequency
        let consistency_score = if total_workouts >= 5 { 0.9 }
        else if total_workouts >= 3 { 0.7 }
        else if total_workouts >= 1 { 0.5 }
        else { 0.0 };

        Ok(ProgressAnalysis {
            total_workouts,
            average_duration_minutes: avg_duration,
            total_calories_burned: total_calories,
            consistency_score,
        })
    }

    // Database health check
    pub async fn health_check(&self) -> Result<DatabaseHealth> {
        let users_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool).await?;
        
        let exercises_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM exercises")
            .fetch_one(&self.pool).await?;
            
        let workouts_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM workout_sessions")
            .fetch_one(&self.pool).await?;

        Ok(DatabaseHealth {
            connected: true,
            users_count: users_count as u32,
            exercises_count: exercises_count as u32,
            workouts_count: workouts_count as u32,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct DatabaseHealth {
    pub connected: bool,
    pub users_count: u32,
    pub exercises_count: u32,
    pub workouts_count: u32,
}
