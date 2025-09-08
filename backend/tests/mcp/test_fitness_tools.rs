#[cfg(test)]
mod fitness_tools_tests {
    use fitness_advisor_ai::mcp::{
        FitnessTools, UserProfile, Gender, ActivityLevel, FitnessGoal
    };
    use serde_json::json;
    use uuid::Uuid;

    fn create_test_user_profile() -> UserProfile {
        UserProfile {
            id: Uuid::new_v4(),
            age: 25,
            weight_kg: 70.0,
            height_cm: 175,
            gender: Gender::Male,
            activity_level: ActivityLevel::ModeratelyActive,
            fitness_goals: vec![FitnessGoal::MuscleGain, FitnessGoal::StrengthGain],
            dietary_restrictions: vec!["gluten-free".to_string()],
            health_conditions: vec![],
        }
    }

    #[tokio::test]
    async fn test_create_workout_plan_with_parameters() {
        let fitness_tools = FitnessTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 25,
                "weight_kg": 70.0,
                "height_cm": 175,
                "gender": "male",
                "activity_level": "moderately_active",
                "fitness_goals": ["muscle_gain"],
                "dietary_restrictions": [],
                "health_conditions": []
            },
            "workout_preferences": {
                "duration_minutes": 45,
                "difficulty_level": "intermediate",
                "equipment_available": ["dumbbells", "barbell"],
                "workout_type": "strength"
            }
        });

        let result = fitness_tools.create_workout_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Workout Plan"));
            assert!(text.contains("45 minutes"));
            assert!(text.contains("Intermediate"));
            assert!(text.contains("Exercise"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_create_workout_plan_without_parameters() {
        let fitness_tools = FitnessTools::new();
        
        let result = fitness_tools.create_workout_plan(None).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Error"));
        } else {
            panic!("Expected error text content");
        }
        
        assert_eq!(result.is_error, Some(true));
    }

    #[tokio::test]
    async fn test_create_bodyweight_workout() {
        let fitness_tools = FitnessTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 30,
                "weight_kg": 65.0,
                "height_cm": 165,
                "gender": "female",
                "activity_level": "lightly_active",
                "fitness_goals": ["general_fitness"],
                "dietary_restrictions": [],
                "health_conditions": []
            },
            "workout_preferences": {
                "duration_minutes": 30,
                "difficulty_level": "beginner",
                "equipment_available": ["bodyweight"],
                "workout_type": "mixed"
            }
        });

        let result = fitness_tools.create_workout_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("30 minutes"));
            assert!(text.contains("Beginner"));
            assert!(text.contains("Push-ups") || text.contains("Squats"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_seasonal_optimization() {
        let fitness_tools = FitnessTools::new();
        
        let args = json!({
            "location": "New York",
            "season": "winter",
            "indoor_preference": true,
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 28,
                "weight_kg": 75.0,
                "height_cm": 180,
                "gender": "male",
                "activity_level": "very_active",
                "fitness_goals": ["strength_gain"],
                "dietary_restrictions": [],
                "health_conditions": []
            }
        });

        let result = fitness_tools.optimize_for_season(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Winter"));
            assert!(text.contains("New York"));
            assert!(text.contains("indoor"));
            assert!(text.contains("strength"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_progress_tracking() {
        let fitness_tools = FitnessTools::new();
        
        let args = json!({
            "user_id": Uuid::new_v4(),
            "metrics": [
                {
                    "name": "body_weight",
                    "value": 70.0,
                    "unit": "kg",
                    "date": "2024-01-01T00:00:00Z",
                    "notes": "Starting weight"
                },
                {
                    "name": "body_weight",
                    "value": 68.5,
                    "unit": "kg", 
                    "date": "2024-02-01T00:00:00Z",
                    "notes": "After 1 month"
                }
            ],
            "time_range_days": 30
        });

        let result = fitness_tools.track_progress(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Progress Analysis"));
            assert!(text.contains("body_weight"));
            assert!(text.contains("70.0"));
            assert!(text.contains("68.5"));
            assert!(text.contains("Recommendations"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_progress_tracking_no_metrics() {
        let fitness_tools = FitnessTools::new();
        
        let args = json!({
            "user_id": Uuid::new_v4(),
            "metrics": [],
            "time_range_days": 30
        });

        let result = fitness_tools.track_progress(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("No metrics"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_evidence_based_recommendation() {
        let fitness_tools = FitnessTools::new();
        
        let recommendation = fitness_tools
            .get_evidence_based_recommendation("How often should I train for muscle gain?")
            .await
            .unwrap();
        
        assert!(recommendation.contains("Evidence-based recommendation"));
        assert!(recommendation.contains("muscle gain"));
    }

    #[tokio::test]
    async fn test_cardio_workout_generation() {
        let fitness_tools = FitnessTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 35,
                "weight_kg": 80.0,
                "height_cm": 175,
                "gender": "male",
                "activity_level": "moderately_active",
                "fitness_goals": ["weight_loss", "endurance"],
                "dietary_restrictions": [],
                "health_conditions": []
            },
            "workout_preferences": {
                "duration_minutes": 30,
                "difficulty_level": "intermediate",
                "workout_type": "cardio"
            }
        });

        let result = fitness_tools.create_workout_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Cardio") || text.contains("cardiovascular"));
            assert!(text.contains("Burpees") || text.contains("High Knees") || text.contains("Jumping"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_flexibility_workout_generation() {
        let fitness_tools = FitnessTools::new();
        
        let args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 45,
                "weight_kg": 60.0,
                "height_cm": 160,
                "gender": "female",
                "activity_level": "lightly_active",
                "fitness_goals": ["general_fitness"],
                "dietary_restrictions": [],
                "health_conditions": ["lower_back_pain"]
            },
            "workout_preferences": {
                "duration_minutes": 20,
                "difficulty_level": "beginner",
                "workout_type": "flexibility"
            }
        });

        let result = fitness_tools.create_workout_plan(Some(args)).await.unwrap();
        
        assert!(result.content.len() == 1);
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("flexibility") || text.contains("stretch"));
            assert!(text.contains("Cat-Cow") || text.contains("Downward Dog") || text.contains("Pigeon"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_difficulty_level_determination() {
        let fitness_tools = FitnessTools::new();
        
        // Test with sedentary user - should get beginner
        let sedentary_args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 25,
                "weight_kg": 70.0,
                "height_cm": 175,
                "gender": "male",
                "activity_level": "sedentary",
                "fitness_goals": ["general_fitness"],
                "dietary_restrictions": [],
                "health_conditions": []
            }
        });

        let result = fitness_tools.create_workout_plan(Some(sedentary_args)).await.unwrap();
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Beginner"));
        }
        
        // Test with super active user - should get expert
        let super_active_args = json!({
            "user_profile": {
                "id": Uuid::new_v4(),
                "age": 25,
                "weight_kg": 70.0,
                "height_cm": 175,
                "gender": "male",
                "activity_level": "super_active",
                "fitness_goals": ["strength_gain"],
                "dietary_restrictions": [],
                "health_conditions": []
            }
        });

        let result = fitness_tools.create_workout_plan(Some(super_active_args)).await.unwrap();
        if let fitness_advisor_ai::mcp::types::ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Expert"));
        }
    }
}