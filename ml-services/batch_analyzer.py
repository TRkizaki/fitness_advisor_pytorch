#!/usr/bin/env python3
"""
Batch Workout Session Analyzer - Functional Programming Approach
Analyzes entire workout videos (30-60 minutes) for comprehensive insights
"""

import sys
import json
import cv2
import numpy as np
from typing import Dict, List, Tuple, Optional
import mediapipe as mp
from pathlib import Path
import time

# Initialize MediaPipe
mp_pose = mp.solutions.pose
pose_model = mp_pose.Pose(
    static_image_mode=False,
    model_complexity=1,
    smooth_landmarks=True,
    enable_segmentation=False,
    min_detection_confidence=0.7,
    min_tracking_confidence=0.5
)

def extract_frames_from_video(video_path: str, sample_rate: int = 30) -> List[np.ndarray]:
    """Extract frames from video at specified sample rate (every Nth frame)"""
    cap = cv2.VideoCapture(video_path)
    frames = []
    frame_count = 0
    
    while True:
        ret, frame = cap.read()
        if not ret:
            break
            
        if frame_count % sample_rate == 0:  # Sample every Nth frame
            frames.append(frame)
            
        frame_count += 1
    
    cap.release()
    total_duration = frame_count / cap.get(cv2.CAP_PROP_FPS) if cap.get(cv2.CAP_PROP_FPS) > 0 else 0
    
    return frames, total_duration, frame_count

def detect_pose_in_frame(frame: np.ndarray) -> Optional[Dict]:
    """Detect pose landmarks in a single frame"""
    frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
    results = pose_model.process(frame_rgb)
    
    if not results.pose_landmarks:
        return None
        
    # Extract key landmarks
    landmarks = {}
    landmark_map = {
        'nose': 0, 'left_shoulder': 11, 'right_shoulder': 12,
        'left_elbow': 13, 'right_elbow': 14, 'left_wrist': 15, 'right_wrist': 16,
        'left_hip': 23, 'right_hip': 24, 'left_knee': 25, 'right_knee': 26,
        'left_ankle': 27, 'right_ankle': 28
    }
    
    for name, idx in landmark_map.items():
        if idx < len(results.pose_landmarks.landmark):
            landmark = results.pose_landmarks.landmark[idx]
            landmarks[name] = (landmark.x, landmark.y, landmark.visibility)
    
    return landmarks

def classify_exercise_from_pose(landmarks: Dict) -> str:
    """Classify current exercise based on pose landmarks"""
    if not landmarks:
        return 'unknown'
    
    # Get key positions (using only x, y coordinates)
    try:
        left_shoulder = landmarks['left_shoulder'][:2]
        right_shoulder = landmarks['right_shoulder'][:2]
        left_hip = landmarks['left_hip'][:2]
        right_hip = landmarks['right_hip'][:2]
        left_knee = landmarks['left_knee'][:2]
        right_knee = landmarks['right_knee'][:2]
        
        # Calculate average positions
        shoulder_y = (left_shoulder[1] + right_shoulder[1]) / 2
        hip_y = (left_hip[1] + right_hip[1]) / 2
        knee_y = (left_knee[1] + right_knee[1]) / 2
        
        # Exercise classification logic
        if hip_y > shoulder_y + 0.15 and knee_y > hip_y - 0.1:
            return 'squat'
        elif abs(shoulder_y - hip_y) < 0.2 and 'left_wrist' in landmarks:
            wrist_y = landmarks['left_wrist'][1]
            if abs(wrist_y - shoulder_y) < 0.15:
                return 'pushup'
        elif abs(shoulder_y - hip_y) < 0.15:
            return 'plank'
        else:
            return 'standing'
            
    except (KeyError, IndexError):
        return 'unknown'

def count_reps_in_sequence(poses: List[Dict], exercise_type: str) -> int:
    """Count repetitions based on pose sequence for specific exercise"""
    if exercise_type == 'unknown' or not poses:
        return 0
    
    rep_count = 0
    in_rep = False
    
    for pose in poses:
        if not pose:
            continue
            
        # Squat rep counting
        if exercise_type == 'squat':
            try:
                hip_y = (pose['left_hip'][1] + pose['right_hip'][1]) / 2
                knee_y = (pose['left_knee'][1] + pose['right_knee'][1]) / 2
                
                # Deep squat position
                if hip_y > knee_y - 0.05 and not in_rep:
                    in_rep = True
                # Return to standing
                elif hip_y < knee_y - 0.15 and in_rep:
                    rep_count += 1
                    in_rep = False
            except KeyError:
                continue
                
        # Pushup rep counting
        elif exercise_type == 'pushup':
            try:
                shoulder_y = pose['left_shoulder'][1]
                wrist_y = pose['left_wrist'][1]
                
                # Bottom position
                if abs(shoulder_y - wrist_y) < 0.1 and not in_rep:
                    in_rep = True
                # Top position
                elif abs(shoulder_y - wrist_y) > 0.2 and in_rep:
                    rep_count += 1
                    in_rep = False
            except KeyError:
                continue
    
    return rep_count

def detect_fatigue_indicators(poses: List[Dict], exercise_type: str) -> Dict:
    """Detect signs of fatigue based on form degradation"""
    if len(poses) < 10:
        return {'fatigue_score': 0.0, 'indicators': []}
    
    # Split into early and late portions
    early_poses = poses[:len(poses)//3]
    late_poses = poses[-len(poses)//3:]
    
    fatigue_indicators = []
    fatigue_score = 0.0
    
    # Analyze form consistency
    early_consistency = calculate_form_consistency(early_poses, exercise_type)
    late_consistency = calculate_form_consistency(late_poses, exercise_type)
    
    if late_consistency < early_consistency - 0.15:
        fatigue_indicators.append("Form degradation detected")
        fatigue_score += 0.3
    
    # Check for tremor/instability (simplified)
    if len(late_poses) > 5:
        late_stability = calculate_pose_stability(late_poses)
        early_stability = calculate_pose_stability(early_poses)
        
        if late_stability < early_stability - 0.1:
            fatigue_indicators.append("Increased movement instability")
            fatigue_score += 0.2
    
    return {
        'fatigue_score': min(fatigue_score, 1.0),
        'indicators': fatigue_indicators,
        'early_consistency': early_consistency,
        'late_consistency': late_consistency
    }

def calculate_form_consistency(poses: List[Dict], exercise_type: str) -> float:
    """Calculate how consistent the form is across poses"""
    if not poses or exercise_type == 'unknown':
        return 0.0
    
    # Simplified consistency check based on key joint positions
    consistencies = []
    
    for i in range(1, len(poses)):
        if poses[i] and poses[i-1]:
            try:
                # Compare shoulder width consistency
                curr_shoulder_width = abs(poses[i]['left_shoulder'][0] - poses[i]['right_shoulder'][0])
                prev_shoulder_width = abs(poses[i-1]['left_shoulder'][0] - poses[i-1]['right_shoulder'][0])
                
                consistency = 1.0 - abs(curr_shoulder_width - prev_shoulder_width)
                consistencies.append(max(0.0, consistency))
            except KeyError:
                continue
    
    return np.mean(consistencies) if consistencies else 0.0

def calculate_pose_stability(poses: List[Dict]) -> float:
    """Calculate pose stability (less movement = more stable)"""
    if len(poses) < 2:
        return 1.0
    
    movements = []
    
    for i in range(1, len(poses)):
        if poses[i] and poses[i-1]:
            try:
                # Calculate movement of key points
                nose_movement = np.sqrt(
                    (poses[i]['nose'][0] - poses[i-1]['nose'][0])**2 +
                    (poses[i]['nose'][1] - poses[i-1]['nose'][1])**2
                )
                movements.append(nose_movement)
            except KeyError:
                continue
    
    avg_movement = np.mean(movements) if movements else 0.0
    return max(0.0, 1.0 - avg_movement * 10)  # Scale movement to stability score

def segment_workout_into_exercises(poses: List[Dict], timestamps: List[float]) -> List[Dict]:
    """Segment the workout into different exercise periods"""
    segments = []
    current_exercise = None
    segment_start = 0
    segment_poses = []
    
    for i, pose in enumerate(poses):
        exercise = classify_exercise_from_pose(pose)
        
        if exercise != current_exercise:
            # Save previous segment if it has enough data
            if current_exercise and len(segment_poses) > 5:
                segments.append({
                    'exercise': current_exercise,
                    'start_time': timestamps[segment_start],
                    'end_time': timestamps[i-1] if i > 0 else timestamps[segment_start],
                    'duration': timestamps[i-1] - timestamps[segment_start] if i > 0 else 0,
                    'poses': segment_poses.copy(),
                    'frame_count': len(segment_poses)
                })
            
            # Start new segment
            current_exercise = exercise
            segment_start = i
            segment_poses = [pose]
        else:
            segment_poses.append(pose)
    
    # Add final segment
    if current_exercise and len(segment_poses) > 5:
        segments.append({
            'exercise': current_exercise,
            'start_time': timestamps[segment_start],
            'end_time': timestamps[-1],
            'duration': timestamps[-1] - timestamps[segment_start],
            'poses': segment_poses,
            'frame_count': len(segment_poses)
        })
    
    return segments

def analyze_workout_session(video_path: str) -> Dict:
    """Main function to analyze a complete workout session"""
    print(f"🎬 Analyzing workout session: {video_path}")
    
    # Extract frames from video
    frames, total_duration, total_frames = extract_frames_from_video(video_path, sample_rate=30)
    print(f"📊 Extracted {len(frames)} frames from {total_duration:.1f}s video")
    
    if not frames:
        return {'error': 'No frames extracted from video'}
    
    # Analyze poses in all frames
    poses = []
    timestamps = []
    
    for i, frame in enumerate(frames):
        pose = detect_pose_in_frame(frame)
        poses.append(pose)
        timestamps.append(i * (total_duration / len(frames)))
    
    valid_poses = [p for p in poses if p is not None]
    print(f"🤸 Detected poses in {len(valid_poses)}/{len(poses)} frames")
    
    if len(valid_poses) < 10:
        return {'error': 'Insufficient pose data for analysis'}
    
    # Segment workout into exercises
    segments = segment_workout_into_exercises(poses, timestamps)
    print(f"🏃 Segmented into {len(segments)} exercise periods")
    
    # Analyze each segment
    segment_analysis = []
    total_reps = 0
    
    for segment in segments:
        if segment['exercise'] == 'unknown':
            continue
            
        reps = count_reps_in_sequence(segment['poses'], segment['exercise'])
        fatigue = detect_fatigue_indicators(segment['poses'], segment['exercise'])
        
        segment_analysis.append({
            'exercise': segment['exercise'],
            'duration': segment['duration'],
            'start_time': segment['start_time'],
            'end_time': segment['end_time'],
            'reps_counted': reps,
            'fatigue_analysis': fatigue,
            'frame_count': segment['frame_count']
        })
        
        total_reps += reps
    
    # Generate session summary
    exercise_counts = {}
    total_exercise_time = 0
    
    for analysis in segment_analysis:
        exercise = analysis['exercise']
        exercise_counts[exercise] = exercise_counts.get(exercise, 0) + analysis['reps_counted']
        total_exercise_time += analysis['duration']
    
    return {
        'session_summary': {
            'total_duration': total_duration,
            'total_exercise_time': total_exercise_time,
            'rest_time': total_duration - total_exercise_time,
            'total_reps': total_reps,
            'exercises_performed': list(exercise_counts.keys()),
            'exercise_breakdown': exercise_counts
        },
        'detailed_analysis': segment_analysis,
        'video_stats': {
            'total_frames': total_frames,
            'analyzed_frames': len(frames),
            'pose_detection_rate': len(valid_poses) / len(poses) if poses else 0
        }
    }

def main():
    """Command line interface for batch analysis"""
    if len(sys.argv) != 2:
        print("Usage: python3 batch_analyzer.py <video_path>")
        sys.exit(1)
    
    video_path = sys.argv[1]
    
    if not Path(video_path).exists():
        print(f"Error: Video file {video_path} not found")
        sys.exit(1)
    
    try:
        start_time = time.time()
        result = analyze_workout_session(video_path)
        processing_time = time.time() - start_time
        
        result['processing_time'] = processing_time
        
        print(json.dumps(result, indent=2))
        
    except Exception as e:
        error_response = {
            'error': f'Batch analysis failed: {str(e)}',
            'video_path': video_path
        }
        print(json.dumps(error_response, indent=2))
        sys.exit(1)

if __name__ == "__main__":
    main()