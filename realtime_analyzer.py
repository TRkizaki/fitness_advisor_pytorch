#!/usr/bin/env python3
"""
Real-time Motion Analysis - Optimized for <50ms latency
Processes single frames for live video streaming analysis
"""

import sys
import json
import base64
import io
import time
import numpy as np
from typing import Dict, Optional, Tuple

try:
    import cv2
    from PIL import Image
    import mediapipe as mp
except ImportError as e:
    print(json.dumps({"error": f"Missing dependencies: {e}"}))
    sys.exit(1)

# Global MediaPipe instance for reuse (faster than recreating)
mp_pose = mp.solutions.pose
pose_model = mp_pose.Pose(
    static_image_mode=True,  # Single frame processing
    model_complexity=0,      # Fastest model (Lite)
    smooth_landmarks=False,  # No smoothing for single frames
    enable_segmentation=False,
    min_detection_confidence=0.6,  # Lower for speed
    min_tracking_confidence=0.5
)

def calculate_angle_fast(p1: Tuple[float, float], p2: Tuple[float, float], p3: Tuple[float, float]) -> float:
    """Fast angle calculation between three points"""
    try:
        # Vectorized calculation
        a = np.array([p1[0] - p2[0], p1[1] - p2[1]])
        b = np.array([p3[0] - p2[0], p3[1] - p2[1]])
        
        cos_angle = np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b) + 1e-8)
        angle = np.arccos(np.clip(cos_angle, -1.0, 1.0))
        return np.degrees(angle)
    except:
        return 0.0

def extract_key_landmarks(results) -> Optional[Dict]:
    """Extract only essential landmarks for speed"""
    if not results.pose_landmarks:
        return None
    
    landmarks = results.pose_landmarks.landmark
    
    # Only extract essential points for real-time analysis
    key_points = {
        'nose': (landmarks[0].x, landmarks[0].y),
        'left_shoulder': (landmarks[11].x, landmarks[11].y),
        'right_shoulder': (landmarks[12].x, landmarks[12].y),
        'left_elbow': (landmarks[13].x, landmarks[13].y),
        'right_elbow': (landmarks[14].x, landmarks[14].y),
        'left_wrist': (landmarks[15].x, landmarks[15].y),
        'right_wrist': (landmarks[16].x, landmarks[16].y),
        'left_hip': (landmarks[23].x, landmarks[23].y),
        'right_hip': (landmarks[24].x, landmarks[24].y),
        'left_knee': (landmarks[25].x, landmarks[25].y),
        'right_knee': (landmarks[26].x, landmarks[26].y),
        'left_ankle': (landmarks[27].x, landmarks[27].y),
        'right_ankle': (landmarks[28].x, landmarks[28].y),
    }
    
    return key_points

def quick_exercise_detection(landmarks: Dict) -> str:
    """Fast exercise classification for real-time use"""
    try:
        # Quick body position analysis
        shoulder_y = (landmarks['left_shoulder'][1] + landmarks['right_shoulder'][1]) / 2
        hip_y = (landmarks['left_hip'][1] + landmarks['right_hip'][1]) / 2
        knee_y = (landmarks['left_knee'][1] + landmarks['right_knee'][1]) / 2
        wrist_y = (landmarks['left_wrist'][1] + landmarks['right_wrist'][1]) / 2
        
        # Simple classification rules for speed
        if hip_y > shoulder_y + 0.12:  # Hips below shoulders
            if knee_y > hip_y - 0.08:  # Knees near hip level
                return 'squat'
        
        if abs(shoulder_y - hip_y) < 0.18:  # Horizontal body
            if abs(wrist_y - shoulder_y) < 0.15:  # Arms supporting
                return 'pushup'
            else:
                return 'plank'
        
        return 'standing'
        
    except KeyError:
        return 'unknown'

def fast_form_analysis(landmarks: Dict, exercise: str) -> Dict:
    """Lightning-fast form analysis for real-time feedback"""
    score = 85  # Start with good score
    feedback = []
    warnings = []
    
    try:
        if exercise == 'squat':
            # Quick squat form check
            left_knee_angle = calculate_angle_fast(
                landmarks['left_hip'], landmarks['left_knee'], landmarks['left_ankle']
            )
            
            if left_knee_angle < 70:
                warnings.append("Too deep - ease up slightly")
                score -= 10
            elif left_knee_angle > 120:
                feedback.append("Go deeper for better activation")
                score -= 5
            else:
                feedback.append("Good squat depth!")
            
            # Knee alignment check
            knee_distance = abs(landmarks['left_knee'][0] - landmarks['right_knee'][0])
            if knee_distance < 0.08:
                warnings.append("Knees caving in!")
                score -= 15
            
        elif exercise == 'pushup':
            # Quick pushup form check
            body_alignment = abs(landmarks['left_shoulder'][1] - landmarks['left_hip'][1])
            if body_alignment > 0.15:
                warnings.append("Keep body straight!")
                score -= 10
            else:
                feedback.append("Good body alignment")
                
        elif exercise == 'plank':
            # Quick plank check
            hip_shoulder_diff = abs(landmarks['left_hip'][1] - landmarks['left_shoulder'][1])
            if hip_shoulder_diff > 0.1:
                warnings.append("Align hips with shoulders")
                score -= 10
            else:
                feedback.append("Perfect plank position!")
        
        elif exercise == 'standing':
            feedback.append("Ready to exercise!")
        
        else:
            feedback.append("Keep moving!")
    
    except KeyError:
        warnings.append("Pose detection incomplete")
        score = 50
    
    return {
        'score': max(0, min(100, score)),
        'feedback': feedback,
        'warnings': warnings,
        'exercise': exercise
    }

def analyze_frame_realtime(frame_data: bytes) -> Dict:
    """Main real-time analysis function - optimized for speed"""
    start_time = time.time()
    
    try:
        # Decode image data
        image = Image.open(io.BytesIO(frame_data))
        
        # Convert to OpenCV format
        frame = np.array(image)
        if len(frame.shape) == 3:
            if frame.shape[2] == 3:  # RGB
                frame_rgb = frame
            elif frame.shape[2] == 4:  # RGBA
                frame_rgb = cv2.cvtColor(frame, cv2.COLOR_RGBA2RGB)
            else:
                raise ValueError("Unsupported image format")
        else:
            raise ValueError("Invalid image dimensions")
        
        # MediaPipe pose detection
        results = pose_model.process(frame_rgb)
        
        if not results.pose_landmarks:
            return {
                'success': False,
                'error': 'No pose detected',
                'processing_time_ms': (time.time() - start_time) * 1000,
                'feedback': ['Make sure your full body is visible'],
                'warnings': ['No pose detected'],
                'score': 0,
                'exercise': 'unknown'
            }
        
        # Extract landmarks
        landmarks = extract_key_landmarks(results)
        
        # Quick exercise detection
        exercise = quick_exercise_detection(landmarks)
        
        # Fast form analysis
        analysis = fast_form_analysis(landmarks, exercise)
        
        processing_time = (time.time() - start_time) * 1000
        
        return {
            'success': True,
            'processing_time_ms': processing_time,
            'score': analysis['score'],
            'exercise': analysis['exercise'],
            'feedback': analysis['feedback'],
            'warnings': analysis['warnings'],
            'landmarks_count': len(landmarks),
            'performance': {
                'target_latency_ms': 50,
                'actual_latency_ms': processing_time,
                'within_target': processing_time < 50
            }
        }
        
    except Exception as e:
        return {
            'success': False,
            'error': f'Analysis failed: {str(e)}',
            'processing_time_ms': (time.time() - start_time) * 1000,
            'feedback': [],
            'warnings': [f'Processing error: {str(e)}'],
            'score': 0,
            'exercise': 'error'
        }

def main():
    """Command line interface for real-time analysis"""
    try:
        # Read JSON input from stdin
        input_data = sys.stdin.read().strip()
        if not input_data:
            print(json.dumps({"error": "No input data provided"}))
            sys.exit(1)
        
        request = json.loads(input_data)
        
        # Extract frame data
        frame_base64 = request.get('frame_data', '')
        if not frame_base64:
            print(json.dumps({"error": "No frame data provided"}))
            sys.exit(1)
        
        # Decode base64 frame data
        try:
            frame_data = base64.b64decode(frame_base64)
        except Exception as e:
            print(json.dumps({"error": f"Failed to decode frame data: {str(e)}"}))
            sys.exit(1)
        
        # Analyze frame
        result = analyze_frame_realtime(frame_data)
        
        # Output result
        print(json.dumps(result))
        
    except Exception as e:
        error_response = {
            "error": f"Real-time analysis failed: {str(e)}",
            "success": False,
            "processing_time_ms": 0,
            "feedback": [],
            "warnings": [str(e)],
            "score": 0,
            "exercise": "error"
        }
        print(json.dumps(error_response))
        sys.exit(1)

if __name__ == "__main__":
    main()