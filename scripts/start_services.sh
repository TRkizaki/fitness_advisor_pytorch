#!/bin/bash
# Start both Rust API server and Python ML service

set -e

echo "Starting Fitness Advisor AI Services..."

# Function to cleanup processes on exit
cleanup() {
    echo "Shutting down services..."
    if [ ! -z "$RUST_PID" ]; then
        kill $RUST_PID 2>/dev/null || true
    fi
    if [ ! -z "$PYTHON_PID" ]; then
        kill $PYTHON_PID 2>/dev/null || true
    fi
    exit
}

trap cleanup SIGINT SIGTERM

# Create logs directory if it doesn't exist
mkdir -p logs

# Install Python dependencies if requirements.txt exists
if [ -f "ml-services/requirements.txt" ]; then
    echo "Installing Python dependencies..."
    pip install -r ml-services/requirements.txt
fi

# Start Python ML service in background
echo "Starting Python ML service on port 8001..."
cd ml-services && python3 ml_service.py --host 127.0.0.1 --port 8001 > ../logs/ml_service.log 2>&1 &
cd ..
PYTHON_PID=$!
echo "Python ML service started with PID: $PYTHON_PID"

# Wait a moment for ML service to start
sleep 3

# Start Rust API server
echo "Starting Rust API server on port 3000..."
cd backend && cargo run > ../logs/rust_server.log 2>&1 &
cd ..
RUST_PID=$!
echo "Rust API server started with PID: $RUST_PID"

echo ""
echo "Services started successfully!"
echo "- Rust API server: http://localhost:3000"
echo "- Python ML service: http://localhost:8001"
echo "- API docs: http://localhost:8001/docs (FastAPI)"
echo ""
echo "Logs:"
echo "- Rust server: logs/rust_server.log"
echo "- Python ML service: logs/ml_service.log"
echo ""
echo "Press Ctrl+C to stop all services"

# Wait for both processes
wait $RUST_PID $PYTHON_PID