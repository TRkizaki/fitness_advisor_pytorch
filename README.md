# Fitness Advisor AI

A Rust-based fitness application that combines real-time pose analysis with personalized workout recommendations. Features a hybrid architecture with Python ML integration for advanced motion analysis.

## Features

- **AI Motion Analysis**: Real-time pose estimation and exercise form scoring using MediaPipe
- **Exercise Classification**: Automatically detects squats, pushups, planks, and other exercises
- **Personalized Recommendations**: Workout plans based on fitness level and goals
- **Progress Tracking**: Comprehensive analytics and workout history
- **RESTful API**: Clean HTTP endpoints for all functionality
- **SQLite Database**: Persistent storage for users, exercises, and workout data

## Quick Start

### Prerequisites
- Rust (latest stable)
- Python 3.8+
- SQLite

### Installation

1. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd fitness_advisor_ai
   cargo build --release
   ```

2. **Install Python dependencies** (optional, for full ML features):
   ```bash
   pip install -r requirements.txt
   ```

3. **Run the server**:
   ```bash
   cargo run
   ```

The API server will start on `http://localhost:3000`

## API Endpoints

### Core Endpoints
```bash
GET  /api/health              # Health check
GET  /api/users               # List all users  
POST /api/users               # Create new user
GET  /api/users/:id           # Get user details
GET  /api/exercises           # List exercises
```

### AI & Analytics
```bash
POST /api/ai/analyze-form     # AI motion analysis
GET  /api/users/:id/recommendations  # Get workout plan
GET  /api/users/:id/progress  # Progress analytics
POST /api/workouts            # Log workout session
```

### System
```bash
GET  /api/gpu-status          # GPU information
GET  /api/database/health     # Database status
```

## Usage Examples

### Test AI Motion Analysis
```bash
# Create test request with base64 video data
echo '{"video_base64":"'$(base64 -w0 image.jpg)'"}' > test_request.json

# Analyze form
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @test_request.json | jq
```

### Pretty JSON Output
```bash
# Get formatted analysis results
curl -s http://localhost:3000/api/ai/analyze-form \
  -X POST -H "Content-Type: application/json" \
  -d @test_request.json | jq -r '
"🎯 Form Analysis Results:
📊 Score: \(.data.overall_score * 100)%
✅ Recommendations: \(.data.recommendations | join(", "))
⚠️  Issues: \(.data.detected_errors | join(", "))"'
```

### Create User
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "user": {
      "id": "user123",
      "name": "John Doe", 
      "age": 30,
      "height": 175.0,
      "weight": 70.0,
      "fitness_level": "Intermediate",
      "goals": ["Strength", "GeneralHealth"],
      "preferences": {
        "preferred_exercise_types": ["Strength"],
        "available_equipment": ["None", "Dumbbells"],
        "workout_duration_minutes": 45,
        "workouts_per_week": 3,
        "preferred_time_of_day": "evening"
      }
    }
  }'
```

## Architecture

### Hybrid Design
- **Rust Backend**: High-performance API server with SQLite integration
- **Python ML**: Subprocess-based motion analysis using MediaPipe and PyTorch
- **JSON Communication**: Clean data exchange between Rust and Python components

### Performance
- **~21ms**: Average ML analysis time
- **Async Processing**: Non-blocking Python subprocess execution  
- **GPU Ready**: RTX 5070 acceleration support

## Development

### Run Tests
```bash
# Integration test
cargo run --bin test_integration

# Performance benchmark  
cargo run --bin benchmark
```

### Database
- Auto-creates SQLite database on first run
- Seeds with demo users and exercises
- Tables auto-migrate on startup

### Python ML Components
- `ml_analyzer.py`: Full MediaPipe implementation
- `ml_analyzer_test.py`: Lightweight test version
- `requirements.txt`: Python dependencies

## Configuration

### Environment Variables
```bash
RUST_LOG=info          # Enable logging
DATABASE_URL=sqlite:./fitness_advisor.db  # Database path
```

### Default Data
- 3 demo users (Beginner, Intermediate, Advanced)
- 5 exercises (squats, pushups, planks, burpees, deadlifts)
- Sample workout sessions

## Contributing

The codebase follows standard Rust conventions. Key files:
- `src/main.rs` - Main application logic and API handlers
- `src/database.rs` - SQLite integration and data models
- `ml_analyzer.py` - Python ML analysis engine

