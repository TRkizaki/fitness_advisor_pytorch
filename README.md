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

#### Basic Test
```bash
# Create test request with base64 video data
echo '{"video_base64":"'$(base64 -w0 image.jpg)'"}' > test_request.json

# Analyze form
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @test_request.json | jq
```

#### Video Processing Workflow
```bash
# Complete workflow: MOV file → compression → base64 → API test
ffmpeg -i input.mov -t 10 -vf scale=640:480 -c:v libx264 -crf 28 temp.mp4 && \
base64 -w 0 temp.mp4 > video_b64.txt && \
echo '{"video_base64":"'$(cat video_b64.txt)'","exercise_type":"squat"}' > video_test.json && \
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @video_test.json | jq

# Alternative: Process image files
ffmpeg -i workout_photo.jpg -vf scale=640:480 -q:v 2 processed.jpg && \
base64 -w 0 processed.jpg > image_b64.txt && \
echo '{"video_base64":"'$(cat image_b64.txt)'"}' > image_test.json && \
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @image_test.json | jq

# Batch processing multiple files
for file in *.mov; do
  echo "Processing $file..."
  ffmpeg -i "$file" -t 5 -vf scale=480:360 -c:v libx264 -crf 30 "processed_$file.mp4" && \
  base64 -w 0 "processed_$file.mp4" > "${file%.mov}_b64.txt" && \
  echo '{"video_base64":"'$(cat "${file%.mov}_b64.txt")'"}' > "${file%.mov}_test.json" && \
  curl -s -X POST http://localhost:3000/api/ai/analyze-form \
    -H "Content-Type: application/json" \
    -d @"${file%.mov}_test.json" | jq '.data.overall_score'
done
```

#### Exercise-Specific Tests
```bash
# Pushup analysis test
echo '{"video_base64":"'$(cat test_base64.txt)'","exercise_type":"pushup"}' > pushup_test.json
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @pushup_test.json | jq

# Plank analysis test  
echo '{"video_base64":"'$(cat test_base64.txt)'","exercise_type":"plank"}' > plank_test.json
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @plank_test.json | jq

# Debug mode with detailed information
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @ai_test_request_fixed.json | jq '.'

# Measure response time
time curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @ai_test_request_fixed.json
```

## API Response Examples

### Motion Analysis Response
```bash
curl -X POST http://localhost:3000/api/ai/analyze-form \
  -H "Content-Type: application/json" \
  -d @ai_test_request_fixed.json
```

**Response:**
```json
{
  "success": true,
  "data": {
    "overall_score": 0.85,
    "recommendations": [
      "Good squat form detected!",
      "Keep knees aligned over toes", 
      "Maintain straight back posture"
    ],
    "detected_errors": [
      "Minor knee cave detected"
    ],
    "confidence": 0.92
  },
  "message": "Success"
}
```

### Other Endpoint Responses

#### Health Check
```bash
curl http://localhost:3000/api/health
```
```json
{
  "success": true,
  "data": "Fitness Advisor AI is healthy! 💪",
  "message": "Success"
}
```

#### GPU Status
```bash
curl http://localhost:3000/api/gpu-status
```
```json
{
  "success": true,
  "data": {
    "gpu_available": true,
    "gpu_name": "NVIDIA GeForce RTX 5070 Laptop GPU",
    "compute_capability": "12.0",
    "vram_total_mb": 7716,
    "vram_used_mb": 72,
    "cuda_version": "12.4",
    "ready_for_ai": true,
    "features": [
      "Real-time pose estimation",
      "Form analysis", 
      "Motion tracking",
      "AI workout recommendations",
      "Database-backed analytics"
    ]
  },
  "message": "Success"
}
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

