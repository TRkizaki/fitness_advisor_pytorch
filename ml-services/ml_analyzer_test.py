#!/usr/bin/env python3
"""
Simple test version of the ML analyzer for testing Rust integration
"""

import sys
import json
import base64

def main():
    try:
        # Read JSON input from stdin
        input_data = sys.stdin.read().strip()
        if not input_data:
            print(json.dumps({"error": "No input data provided"}))
            sys.exit(1)
        
        # Parse JSON request
        request = json.loads(input_data)
        
        # Extract video data
        video_base64 = request.get('video_base64', '')
        if not video_base64:
            print(json.dumps({"error": "No video data provided"}))
            sys.exit(1)
        
        # Decode base64 video data (just for validation)
        try:
            video_data = base64.b64decode(video_base64)
            data_size = len(video_data)
        except Exception as e:
            print(json.dumps({"error": f"Failed to decode video data: {str(e)}"}))
            sys.exit(1)
        
        # Simulate analysis results
        result = {
            "overall_score": 0.85,
            "recommendations": [
                "Good squat form detected!",
                "Keep knees aligned over toes", 
                "Maintain straight back posture"
            ],
            "detected_errors": [
                "Minor knee cave detected"
            ],
            "confidence": 0.92,
            "keypoints_detected": 17,
            "exercise_type": "squat",
            "analysis_method": "test_mode",
            "data_size_bytes": data_size,
            "python_test": "SUCCESS"
        }
        
        # Output result as JSON
        print(json.dumps(result, indent=2))
        
    except Exception as e:
        error_response = {
            "error": f"Motion analysis failed: {str(e)}",
            "overall_score": 0.0,
            "recommendations": [],
            "detected_errors": [str(e)],
            "confidence": 0.0
        }
        print(json.dumps(error_response, indent=2))
        sys.exit(1)


if __name__ == "__main__":
    main()