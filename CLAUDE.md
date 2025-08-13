# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Fitness Advisor AI application that combines:
- SQLite database for user/workout data persistence
- RESTful API using Axum web framework  
- AI motion analysis capabilities (designed for RTX 5070 GPU integration)
- Personalized workout recommendations based on user fitness levels

## Development Commands

### Build and Run
```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run with logging
RUST_LOG=info cargo run
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_health_check
```

### Database
- SQLite database file: `fitness_advisor.db` (auto-created)
- Tables are auto-migrated on startup
- Sample users and exercises are seeded automatically

### Development Server
- Server runs on `http://0.0.0.0:3000`
- API endpoints documented in startup logs

## Code Architecture

### Core Components

**src/main.rs** - Main application logic containing:
- Data structures (User, Exercise, WorkoutSession, etc.)
- FitnessAdvisor business logic with database integration
- Web API handlers and routing
- AI motion analysis placeholder (RTX 5070 ready)

**src/database.rs** - Database layer:
- DatabaseManager handles all SQLite operations
- Auto-migration of tables on startup
- CRUD operations for users, exercises, workouts
- Progress analytics and health checks

**src/api.rs** - Standalone API module (appears to be alternative implementation):
- Contains similar API handlers but with Mutex-based state management
- Includes comprehensive test suite

### Key Data Models

- **User**: Profile with fitness level, goals, preferences
- **Exercise**: Exercise definitions with instructions and safety tips  
- **WorkoutSession**: User workout records with exercise sets
- **ExerciseSet**: Individual exercise within a workout (sets, reps, weight)
- **ProgressAnalysis**: User fitness progress metrics

### Database Schema

Tables auto-created:
- `users` - User profiles and preferences (JSON columns)
- `exercises` - Exercise library with instructions
- `workout_sessions` - User workout records
- `exercise_sets` - Individual exercises within workouts
- `user_progress` - Progress tracking data

### API Endpoints

The application exposes REST endpoints for:
- User management (`/api/users`)
- Workout recommendations (`/api/users/:id/recommendations`)
- Workout logging (`/api/workouts`)
- Progress analysis (`/api/users/:id/progress`)
- AI form analysis (`/api/ai/analyze-form`) - RTX 5070 integration
- Health checks (`/api/health`, `/api/database/health`)
- GPU status (`/api/gpu-status`)

## Architecture Notes

- Uses async Rust with Tokio runtime
- Database operations use SQLx with compile-time checked queries
- JSON serialization with Serde for API responses
- Base64 encoding for video data in AI analysis
- CORS enabled for web client integration
- Structured logging with tracing crate

## AI Integration

The codebase integrates Python-based ML analysis with MediaPipe:
- **Hybrid Architecture**: Rust spawns Python subprocesses for ML inference
- **Real Pose Estimation**: MediaPipe Pose for joint detection and tracking
- **Exercise Classification**: Auto-detection of squats, pushups, planks
- **Form Analysis**: Joint angle calculations and form scoring (0-100 scale)
- **Performance**: ~21ms avg processing time vs 502ms mock implementation
- **RTX 5070 Ready**: GPU acceleration support in Python components

### Python ML Components
- `ml_analyzer.py` - MediaPipe-based motion analysis with PyTorch
- `requirements.txt` - Python dependencies (torch, mediapipe, opencv-python)
- `ml_analyzer_test.py` - Simplified test version for integration testing

### Testing & Benchmarking
```bash
# Test Rust-Python integration
cargo run --bin test_integration

# Performance benchmark vs mock
cargo run --bin benchmark
```

## Testing Strategy

- Unit tests in `src/api.rs` using `axum-test`
- Tests cover API endpoints and JSON response structures
- Database operations tested through integration scenarios