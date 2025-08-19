 🔧 How WebSocket Works in the Project

  1. Architecture Overview

  Client (Browser/App) ←→ WebSocket ←→ Rust Server ←→ Python ML Analyzer

  2. Server Side (Rust)

  The WebSocket endpoint is available at:
  ws://localhost:3000/api/ai/realtime

  How it works:
  1. Client connects to WebSocket endpoint
  2. Rust receives frames and spawns Python subprocess
  3. Python analyzes frame and returns JSON
  4. Rust sends analysis results back to client

  3. Client Side Usage

  Basic JavaScript WebSocket Client:

  // Connect to WebSocket
  const ws = new WebSocket('ws://localhost:3000/api/ai/realtime');

  ws.onopen = () => {
      console.log('Connected to real-time analysis');
  };

  ws.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (data.type === 'welcome') {
          console.log('Ready for analysis');
      }

      if (data.type === 'analysis') {
          console.log(`Score: ${data.score}`);
          console.log(`Exercise: ${data.exercise}`);
          console.log(`Latency: ${data.total_latency_ms}ms`);
          console.log(`Feedback: ${data.feedback.join(', ')}`);
      }
  };

  // Send frame data
  function sendFrame(imageBase64) {
      const message = {
          frame_data: imageBase64,
          timestamp: Date.now()
      };
      ws.send(JSON.stringify(message));
  }

  4. Testing the WebSocket

  Option A: Use the HTML Test Client

  # Start the server
  cargo run

  # In another terminal, serve the test page
  python3 -m http.server 8080

  # Open browser to http://localhost:8080/test_realtime.html
  # Click "Start Camera" → "Connect WebSocket"

  Option B: Command Line Testing

  # Install wscat for testing
  npm install -g wscat

  # Connect to WebSocket
  wscat -c ws://localhost:3000/api/ai/realtime

  # Send test message
  {"frame_data":"iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==","timestam
  p":1234567890}

  Option C: Performance Testing

  # Test with an image
  cargo run --bin test_realtime test_image.jpg

  5. Message Formats

  Client → Server (JSON):

  {
      "frame_data": "base64_encoded_image_data",
      "timestamp": 1234567890
  }

  Client → Server (Binary):

  You can also send raw image bytes directly as binary WebSocket message.

  Server → Client Response:

  {
      "type": "analysis",
      "success": true,
      "score": 85,
      "exercise": "squat",
      "feedback": ["Good squat depth!", "Keep knees aligned"],
      "warnings": ["Minor knee cave detected"],
      "processing_time_ms": 38.2,
      "total_latency_ms": 42,
      "timestamp": "2025-08-19T10:30:00Z",
      "performance": {
          "target_latency_ms": 50,
          "actual_latency_ms": 42,
          "within_target": true
      }
  }

  6. Live Camera Integration

  HTML5 Camera Capture:

  // Get camera stream
  const video = document.getElementById('video');
  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('2d');

  const stream = await navigator.mediaDevices.getUserMedia({
      video: { width: 640, height: 480 }
  });
  video.srcObject = stream;

  // Capture and send frames (30 FPS)
  function captureAndSend() {
      // Draw video frame to canvas
      ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

      // Convert to base64
      canvas.toBlob((blob) => {
          const reader = new FileReader();
          reader.onload = () => {
              const base64 = reader.result.split(',')[1];
              sendFrame(base64);
          };
          reader.readAsDataURL(blob);
      }, 'image/jpeg', 0.8);

      // Schedule next frame
      setTimeout(captureAndSend, 33); // ~30 FPS
  }

  7. Performance Monitoring

  The WebSocket provides real-time performance metrics:
  - Processing time: Python ML analysis time
  - Total latency: End-to-end frame processing
  - Target achievement: Whether <50ms target is met

  8. Error Handling

  ws.onerror = (error) => {
      console.error('WebSocket error:', error);
  };

  ws.onclose = (event) => {
      console.log('Connection closed:', event.code, event.reason);
      // Implement reconnection logic if needed
  };

  9. Production Considerations

  1. Frame Rate Control: Limit to 15-30 FPS to avoid overloading
  2. Quality vs Speed: Use JPEG compression (0.7-0.8 quality)
  3. Connection Management: Handle reconnections gracefully
  4. Buffering: Avoid sending frames if previous analysis isn't complete

