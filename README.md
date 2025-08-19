# Fitness Advisor AI

A high-performance fitness application with hybrid Rust/Python architecture, combining real-time pose analysis with personalized workout recommendations. Features advanced ML capabilities through MediaPipe and PyTorch integration.

## Architecture Overview

```
┌─────────────────────┐    HTTP/JSON    ┌───────────────────────┐
│   Rust Core API     │◄──────────────►│  Python ML Service   │
│ • WebSocket Handler │                 │ • FastAPI Server      │
│ • Database          │                 │ • ML Models           │ 
│ • Business Logic    │                 │ • MediaPipe/PyTorch   │
│ • Real-time API     │                 │ • Form Analysis       │
└─────────────────────┘                 └───────────────────────┘
        Port 3000                              Port 8001
```

## Key Features

### 🚀 **Performance & Architecture**
- **Hybrid Design**: Rust for performance-critical operations, Python for ML processing
- **Real-time Analysis**: <50ms latency pose estimation with WebSocket streaming
- **Concurrent Processing**: High-throughput API with async Rust backend
- **SQLite Integration**: Persistent storage with auto-migration

### 🧠 **AI & Machine Learning**
- **Pose Estimation**: MediaPipe-based real-time body tracking
- **Exercise Classification**: Automatic detection of squats, pushups, planks, and more
- **Form Analysis**: Real-time feedback on exercise technique and safety
- **Batch Processing**: Complete workout session analysis (30-60 minutes)
- **Progress Tracking**: ML-powered fitness analytics and recommendations

### 📊 **Analytics & Tracking**
- **Workout Recommendations**: Personalized plans based on fitness level and goals
- **Rep Counting**: Automatic repetition tracking with fatigue detection
- **Progress Analysis**: Comprehensive workout history and improvement metrics
- **Health Monitoring**: BMR/TDEE calculations with macro nutrition guidance

## Quick Start

### Prerequisites
- **Rust** (latest stable)
- **Python 3.8+** with pip
- **SQLite**
- **GPU** (optional, for accelerated ML processing)

### Installation & Setup

1. **Clone and prepare environment**:
   ```bash
   git clone <repository-url>
   cd fitness_advisor_ai
   
   # Install Python dependencies for ML features
   pip install -r requirements.txt
   
   # Build Rust components
   cargo build --release
   ```

2. **Start the hybrid system**:
   ```bash
   # Option 1: Use the startup script (recommended)
   ./start_services.sh
   
   # Option 2: Start services manually
   python3 ml_service.py --host 127.0.0.1 --port 8001 &
   cargo run
   ```

3. **Verify integration**:
   ```bash
   # Run integration tests
   python3 test_integration.py
   
   # Check service health
   curl http://localhost:3000/api/health
   curl http://localhost:8001/health
   ```

### Services Overview
- **Rust API Server**: `http://localhost:3000` - Core application logic and database
- **Python ML Service**: `http://localhost:8001` - Machine learning and computer vision
- **API Documentation**: `http://localhost:8001/docs` - Interactive FastAPI docs

## API Endpoints

### Core Application (Port 3000)

#### User Management
```bash
GET  /api/users                    # List all users  
POST /api/users                    # Create new user
GET  /api/users/:id                # Get user details
GET  /api/users/:id/recommendations # Get personalized workout plan
GET  /api/users/:id/progress       # Progress analytics
GET  /api/users/:id/workouts       # Workout history
```

#### Exercise & Workout Management
```bash
GET  /api/exercises                # List available exercises
POST /api/workouts                 # Log workout session
```

#### ML Integration Endpoints
```bash
POST /api/ml/analyze-frame         # Single frame analysis
POST /api/ml/analyze-video         # Detailed video analysis  
POST /api/ml/analyze-batch         # Batch workout analysis
GET  /api/ml/status                # ML service status
```

#### Real-time Features
```bash
GET  /api/ai/realtime              # WebSocket real-time streaming
POST /api/ai/analyze-form          # Legacy AI analysis endpoint
```

#### System & Monitoring
```bash
GET  /api/health                   # Application health check
GET  /api/database/health          # Database status
GET  /api/gpu-status               # GPU information
```

### Python ML Service (Port 8001)

```bash
GET  /health                       # Service health check
POST /analyze/frame                # Frame analysis (realtime/detailed)
POST /analyze/video                # Video processing
POST /analyze/batch                # Batch file processing
GET  /models/status                # ML models status
GET  /docs                         # Interactive API documentation
```

## Usage Examples

### Real-time Frame Analysis

```bash
# Basic frame analysis
curl -X POST http://localhost:3000/api/ml/analyze-frame \
  -H "Content-Type: application/json" \
  -d '{
    "frame_base64": "'$(base64 -w0 workout_image.jpg)'"
  }' | jq
```

**Response:**
```json
{
  "success": true,
  "data": {
    "score": 85,
    "exercise": "squat",
    "feedback": ["Good squat depth!", "Keep knees aligned"],
    "warnings": ["Minor form adjustment needed"],
    "processing_time_ms": 42.3
  }
}
```

### Video Analysis

```bash
# Process workout video
ffmpeg -i input.mov -t 10 -vf scale=640:480 -c:v libx264 -crf 28 temp.mp4
curl -X POST http://localhost:3000/api/ml/analyze-video \
  -H "Content-Type: application/json" \
  -d '{
    "video_base64": "'$(base64 -w0 temp.mp4)'"
  }' | jq
```

### Batch Workout Analysis

```bash
# Analyze complete workout session
curl -X POST http://localhost:3000/api/ml/analyze-batch \
  -H "Content-Type: application/json" \
  -d '{
    "video_path": "/path/to/workout_session.mp4"
  }' | jq
```

**Batch Analysis Features:**
- **Exercise Segmentation**: Automatic identification of different exercises
- **Rep Counting**: Precise repetition tracking for squats, pushups, planks
- **Fatigue Detection**: Tracks form degradation over time
- **Session Summary**: Complete workout breakdown with timing and performance metrics

### Real-time Streaming Analysis

#### WebSocket Connection
```javascript
const ws = new WebSocket('ws://localhost:3000/api/ai/realtime');

ws.onopen = () => {
    console.log('Connected to real-time analysis');
};

ws.onmessage = (event) => {
    const analysis = JSON.parse(event.data);
    if (analysis.type === 'analysis') {
        console.log(`Score: ${analysis.score}%, Exercise: ${analysis.exercise}`);
        console.log(`Latency: ${analysis.total_latency_ms}ms`);
        
        // Display real-time feedback
        displayFeedback(analysis.feedback, analysis.warnings);
    }
};

// Send frame data from camera
const sendFrame = (imageBase64) => {
    ws.send(JSON.stringify({
        frame_data: imageBase64,
        timestamp: Date.now()
    }));
};
```

#### Live Camera Demo
```bash
# Test with live camera feed
python3 -m http.server 8080
# Navigate to http://localhost:8080/test_realtime.html
```

### Exercise-Specific Analysis

```bash
# Squat form analysis
echo '{"frame_base64":"'$(cat image_b64.txt)'","analysis_type":"detailed"}' > squat_test.json
curl -X POST http://localhost:8001/analyze/frame \
  -H "Content-Type: application/json" \
  -d @squat_test.json | jq

# Performance measurement
time curl -X POST http://localhost:3000/api/ml/analyze-frame \
  -H "Content-Type: application/json" \
  -d @test_request.json
```

## Configuration

### Environment Variables
```bash
# Core application
FITNESS_CONFIG_PATH=config/default.toml
FITNESS_SERVER_HOST=0.0.0.0
FITNESS_SERVER_PORT=3000
FITNESS_DATABASE_URL=sqlite:./fitness_advisor.db

# ML service integration  
FITNESS_ML_SERVICE_URL=http://127.0.0.1:8001
FITNESS_ML_TIMEOUT_SECONDS=30

# Logging
FITNESS_LOG_LEVEL=info
RUST_LOG=info
```

### Configuration File (config/default.toml)
```toml
[server]
host = "0.0.0.0"
port = 3000

[ml_service]
base_url = "http://127.0.0.1:8001"
timeout_seconds = 30
retry_attempts = 3

[ai_analysis.form_thresholds.squat]
knee_angle_min = 70
knee_angle_max = 120
back_straightness = 0.15

[fitness.macro_ratios.muscle_gain]
protein = 0.30
fat = 0.25
carbs = 0.45
```

## Performance Characteristics

### Real-time Analysis
- **Target Latency**: <50ms per frame
- **Processing Speed**: 20-30 FPS capable
- **Model**: MediaPipe Lite for speed optimization
- **Use Case**: Live video streaming, real-time feedback

### Detailed Analysis  
- **Processing Time**: 100-500ms per frame
- **Model**: Full MediaPipe + PyTorch integration
- **Features**: Comprehensive form analysis, exercise classification
- **Use Case**: Post-workout analysis, detailed feedback

### Batch Processing
- **Session Length**: 30-60 minute videos supported
- **Processing**: Background processing with progress tracking
- **Features**: Exercise segmentation, fatigue analysis, rep counting
- **Use Case**: Complete workout session analytics

## Default Demo Data

The application automatically seeds the database with demo data on first run:

### Pre-created Users
```bash
# View demo users
curl -s http://localhost:3000/api/users | jq '.data[] | {id, name, fitness_level}'
```

- **demo_user**: Intermediate (28 years, 175cm, 70kg)
- **beginner_user**: Beginner (25 years, 165cm, 60kg)  
- **advanced_user**: Advanced (35 years, 180cm, 80kg)

### Pre-loaded Exercises
```bash
# View exercises
curl -s http://localhost:3000/api/exercises | jq '.data[] | {id, name, difficulty_level}'
```

- **squat**: Bodyweight squat (difficulty: 2)
- **pushup**: Classic push-up (difficulty: 3)
- **plank**: Core plank hold (difficulty: 3)
- **burpee**: Full-body burpee (difficulty: 8)
- **deadlift**: Barbell deadlift (difficulty: 7)

### Sample Usage with Demo Data
```bash
# Get personalized recommendations
curl -s http://localhost:3000/api/users/demo_user/recommendations | jq

# View workout history  
curl -s http://localhost:3000/api/users/demo_user/workouts | jq

# Check database status
curl -s http://localhost:3000/api/database/health | jq
```

## Development & Testing

### Run Tests
```bash
# Integration tests (recommended)
python3 test_integration.py

# Rust unit tests
cargo test

# Performance benchmarks
cargo run --bin benchmark
```

### Development Mode
```bash
# Auto-reload Rust server
cargo watch -x run

# Auto-reload Python ML service
uvicorn ml_service:app --reload --host 127.0.0.1 --port 8001
```

### Logs & Monitoring
```bash
# View service logs
tail -f logs/rust_server.log
tail -f logs/ml_service.log

# Monitor ML service performance
curl http://localhost:8001/models/status | jq
```

## Performance Optimization

### GPU Acceleration
- **CUDA Support**: Automatic GPU detection for PyTorch operations
- **MediaPipe GPU**: Hardware-accelerated pose estimation
- **Memory Management**: Efficient GPU memory usage

### System Requirements
- **Minimum**: 4GB RAM, 2-core CPU
- **Recommended**: 8GB+ RAM, 4+ core CPU, GPU with 4GB+ VRAM
- **Optimal**: 16GB+ RAM, 8+ core CPU, RTX 4060+ or equivalent

## Troubleshooting

### Common Issues

1. **ML Service Not Available**
   ```bash
   # Check if Python service is running
   curl http://localhost:8001/health
   
   # Verify dependencies
   pip install -r requirements.txt
   
   # Check logs
   tail logs/ml_service.log
   ```

2. **High Latency Issues**  
   ```bash
   # Check GPU availability
   curl http://localhost:3000/api/gpu-status
   
   # Monitor processing times
   curl http://localhost:8001/models/status
   ```

3. **Database Connection Issues**
   ```bash
   # Check database health
   curl http://localhost:3000/api/database/health
   
   # Reset database (if needed)
   rm fitness_advisor.db && cargo run
   ```

### Performance Tuning
- Adjust MediaPipe model complexity in `ml_analyzer.py`
- Configure batch processing sample rates in `config/default.toml`
- Tune WebSocket frame rates for real-time analysis
- Optimize image preprocessing pipeline

## Contributing

### Project Structure
```
fitness_advisor/
├── src/
│   ├── main.rs              # Main Rust application
│   ├── ml_client.rs         # ML service HTTP client
│   ├── config.rs            # Configuration management
│   └── database.rs          # Database operations
├── config/default.toml      # Application configuration
├── ml_service.py            # Python FastAPI ML service
├── requirements.txt         # Python dependencies
├── start_services.sh        # Service startup script
└── test_integration.py      # Integration test suite
```

### Development Guidelines
- Follow Rust conventions and use `cargo fmt`
- Python code follows PEP 8 standards
- Add tests for new features
- Update configuration schema when adding new settings
- Document API changes in both README and code

## License

This project is built for fitness and health applications. Please ensure compliance with applicable data privacy regulations when handling user fitness data.

---

**Ready to get started?** Run `./start_services.sh` and visit `http://localhost:3000/api/health` to verify your installation!