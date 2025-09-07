# Fitness Advisor AI - Hybrid Configuration

This document describes the implemented hybrid architecture combining Rust and Python for optimal performance and ML capabilities.

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

## Components

### Rust Core (Port 3000)
- **Performance-critical operations**: WebSocket connections, database operations, concurrent request handling
- **API endpoints**: User management, workout tracking, progress analysis
- **ML integration**: HTTP client for Python ML service
- **Configuration management**: TOML-based configuration with environment overrides

### Python ML Service (Port 8001)
- **ML models**: MediaPipe pose estimation, PyTorch motion analysis
- **Analysis types**:
  - Real-time frame analysis (<50ms latency)
  - Detailed video analysis
  - Batch workout session analysis
- **FastAPI endpoints**: RESTful API for ML operations

## File Structure

```
fitness_advisor/
├── src/
│   ├── main.rs              # Main Rust application
│   ├── ml_client.rs         # HTTP client for ML service
│   ├── config.rs            # Configuration management
│   └── database.rs          # Database operations
│
├── config/
│   └── default.toml         # Default configuration
│
├── ml_service.py            # Python FastAPI ML service
├── ml_analyzer.py           # Detailed ML analysis
├── batch_analyzer.py        # Batch video analysis
├── realtime_analyzer.py     # Real-time analysis
│
├── start_services.sh        # Startup script for both services
├── test_integration.py      # Integration test suite
└── requirements.txt         # Python dependencies
```

## Configuration

### Environment Variables
- `FITNESS_CONFIG_PATH`: Path to configuration file (default: `config/default.toml`)
- `FITNESS_SERVER_HOST`: Rust server host (default: `0.0.0.0`)
- `FITNESS_SERVER_PORT`: Rust server port (default: `3000`)
- `FITNESS_DATABASE_URL`: Database URL (default: `sqlite:./fitness_advisor.db`)
- `FITNESS_ML_SERVICE_URL`: ML service URL (default: `http://127.0.0.1:8001`)
- `FITNESS_LOG_LEVEL`: Logging level (default: `info`)

### Configuration File (config/default.toml)
```toml
[server]
host = "0.0.0.0"
port = 3000

[ml_service]
base_url = "http://127.0.0.1:8001"
timeout_seconds = 30

[ai_analysis.form_thresholds.squat]
knee_angle_min = 70
knee_angle_max = 120
```

## API Endpoints

### Rust Core API (Port 3000)

#### User Management
- `POST /api/users` - Create user
- `GET /api/users` - Get all users
- `GET /api/users/:id` - Get specific user

#### ML Integration
- `POST /api/ml/analyze-frame` - Analyze single frame
- `POST /api/ml/analyze-video` - Analyze video data
- `POST /api/ml/analyze-batch` - Batch video analysis
- `GET /api/ml/status` - ML service status

#### System
- `GET /api/health` - Health check
- `GET /api/database/health` - Database status

### Python ML Service (Port 8001)

- `GET /health` - Health check
- `POST /analyze/frame` - Frame analysis
- `POST /analyze/video` - Video analysis
- `POST /analyze/batch` - Batch analysis
- `GET /models/status` - Model status
- `GET /docs` - API documentation (FastAPI auto-generated)

## Usage

### 1. Start Both Services
```bash
# Option 1: Use the startup script
./start_services.sh

# Option 2: Start manually
python3 ml_service.py --host 127.0.0.1 --port 8001 &
cargo run
```

### 2. Test Integration
```bash
# Run integration tests
python3 test_integration.py

# Test individual endpoints
curl http://localhost:3000/api/health
curl http://localhost:8001/health
```

### 3. Example Usage

#### Real-time Frame Analysis
```bash
curl -X POST http://localhost:3000/api/ml/analyze-frame \
  -H "Content-Type: application/json" \
  -d '{
    "frame_base64": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAGAWA0ddwAAAABJRU5ErkJggg=="
  }'
```

#### Check ML Service Status
```bash
curl http://localhost:3000/api/ml/status
```

## Performance Characteristics

### Real-time Analysis
- Target latency: <50ms
- Uses MediaPipe Lite model
- Optimized for live video streams

### Detailed Analysis
- Full MediaPipe model with PyTorch
- Exercise detection and form scoring
- Rep counting and fatigue analysis

### Batch Analysis
- Processes full workout videos (30-60 minutes)
- Workout segmentation by exercise type
- Comprehensive session analytics

## Dependencies

### Rust Dependencies
```toml
axum = "0.7"                    # Web framework
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["sqlite", "chrono"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"                    # Configuration
```

### Python Dependencies
```txt
fastapi>=0.100.0               # Web API framework
uvicorn[standard]>=0.23.0       # ASGI server
torch>=2.0.0                   # PyTorch
mediapipe>=0.10.0              # Pose estimation
opencv-python>=4.8.0           # Computer vision
pillow>=9.0.0                  # Image processing
numpy>=1.24.0                  # Numerical computing
```

## Monitoring and Logging

### Log Files
- Rust server: `logs/rust_server.log`
- Python ML service: `logs/ml_service.log`

### Health Checks
Both services provide health endpoints for monitoring:
- Rust API: `/api/health`
- Python ML: `/health`

## Troubleshooting

### Common Issues

1. **ML Service Not Available**
   - Check if Python service is running on port 8001
   - Verify Python dependencies are installed
   - Check ML service logs

2. **Database Connection Issues**
   - Ensure SQLite database file permissions
   - Check database URL configuration
   - Verify database health endpoint

3. **Configuration Issues**
   - Validate TOML configuration file
   - Check environment variable overrides
   - Review startup logs

### Performance Tuning

1. **Real-time Analysis**
   - Adjust MediaPipe model complexity
   - Tune confidence thresholds
   - Optimize image preprocessing

2. **Batch Processing**
   - Configure sample rates
   - Set appropriate timeouts
   - Monitor memory usage

## Future Enhancements

1. **Scalability**
   - Add Redis for caching
   - Implement message queues
   - Horizontal ML service scaling

2. **Security**
   - Add authentication/authorization
   - Implement rate limiting
   - SSL/TLS configuration

3. **Monitoring**
   - Prometheus metrics
   - Grafana dashboards
   - Distributed tracing

## Development

### Running Tests
```bash
# Integration tests
python3 test_integration.py

# Rust unit tests
cargo test

# Python tests (if any)
pytest
```

### Development Mode
```bash
# Auto-reload Rust server
cargo watch -x run

# Auto-reload Python service
uvicorn ml_service:app --reload --host 127.0.0.1 --port 8001
```