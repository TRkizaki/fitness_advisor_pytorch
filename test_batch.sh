#!/bin/bash

# Test script for batch workout analysis
echo "🏋️ Batch Workout Analysis Test"
echo "==============================="

# Check if video file exists
if [ ! -f "$1" ]; then
    echo "❌ Usage: ./test_batch.sh <video_file>"
    echo "📝 Example: ./test_batch.sh workout_video.mp4"
    exit 1
fi

VIDEO_FILE="$1"
echo "🎬 Testing with video: $VIDEO_FILE"

# Test 1: Direct Python analysis
echo ""
echo "1️⃣ Testing Python batch analyzer directly..."
python3 batch_analyzer.py "$VIDEO_FILE" > direct_output.json
if [ $? -eq 0 ]; then
    echo "✅ Python analysis completed"
    echo "📊 Summary: $(cat direct_output.json | jq -r '.session_summary.total_reps // 0') total reps detected"
else
    echo "❌ Python analysis failed"
fi

# Test 2: Rust integration
echo ""
echo "2️⃣ Testing Rust batch processor..."
cargo build --bin batch_processor --quiet
if [ $? -eq 0 ]; then
    echo "✅ Batch processor compiled"
    cargo run --bin batch_processor "$VIDEO_FILE"
else
    echo "❌ Failed to compile batch processor"
fi

# Test 3: Performance comparison
echo ""
echo "3️⃣ Performance comparison..."
echo "⏱️ Timing Python analysis:"
time python3 batch_analyzer.py "$VIDEO_FILE" > /dev/null

echo ""
echo "⏱️ Timing Rust integration:"
time cargo run --bin batch_processor --quiet "$VIDEO_FILE" > /dev/null

echo ""
echo "🎯 Batch analysis test completed!"
echo "📁 Check generated analysis files:"
ls -la *_analysis.json direct_output.json 2>/dev/null || echo "No analysis files generated"