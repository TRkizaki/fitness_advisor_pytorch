#!/usr/bin/env python3
"""
ML Service API - FastAPI wrapper for fitness analysis components
Provides HTTP endpoints for Rust integration
"""

import asyncio
import base64
import io
import json
import subprocess
import tempfile
import time
from pathlib import Path
from typing import Dict, Optional

from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
import uvicorn

# Import our existing analyzers
from ml_analyzer import MotionAnalyzer
from realtime_analyzer import analyze_frame_realtime

app = FastAPI(
    title="Fitness Advisor ML Service",
    description="ML analysis endpoints for fitness form and motion tracking",
    version="1.0.0"
)

# Add CORS middleware for Rust integration
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Global analyzer instance for reuse
motion_analyzer = None

# Pydantic models for API contracts
class FrameAnalysisRequest(BaseModel):
    frame_data: str = Field(..., description="Base64 encoded image data")
    analysis_type: str = Field(default="realtime", description="Analysis type: realtime or detailed")

class VideoAnalysisRequest(BaseModel):
    video_data: str = Field(..., description="Base64 encoded video data")
    analysis_type: str = Field(default="detailed", description="Analysis type: detailed or batch")

class BatchAnalysisRequest(BaseModel):
    video_path: str = Field(..., description="Path to video file for batch analysis")

class AnalysisResponse(BaseModel):
    success: bool
    processing_time_ms: float
    result: Dict
    error: Optional[str] = None

@app.on_event("startup")
async def startup_event():
    """Initialize ML models on startup"""
    global motion_analyzer
    try:
        motion_analyzer = MotionAnalyzer()
        print("ML Service started - Motion analyzer initialized")
    except Exception as e:
        print(f"Failed to initialize motion analyzer: {e}")

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {
        "status": "healthy",
        "service": "fitness_ml_service",
        "timestamp": time.time(),
        "models_loaded": motion_analyzer is not None
    }

@app.post("/analyze/frame", response_model=AnalysisResponse)
async def analyze_frame(request: FrameAnalysisRequest):
    """Analyze single frame for real-time feedback"""
    start_time = time.time()
    
    try:
        # Decode base64 image data
        try:
            frame_data = base64.b64decode(request.frame_data)
        except Exception as e:
            raise HTTPException(status_code=400, detail=f"Invalid base64 data: {e}")
        
        if request.analysis_type == "realtime":
            # Use optimized real-time analyzer
            result = analyze_frame_realtime(frame_data)
        else:
            # Use detailed motion analyzer
            if motion_analyzer is None:
                raise HTTPException(status_code=503, detail="Motion analyzer not initialized")
            
            from PIL import Image
            import numpy as np
            
            # Convert to frame format
            image = Image.open(io.BytesIO(frame_data))
            frame = np.array(image)
            
            result = motion_analyzer.analyze_video_frame(frame)
            
        processing_time = (time.time() - start_time) * 1000
        
        return AnalysisResponse(
            success=True,
            processing_time_ms=processing_time,
            result=result
        )
        
    except HTTPException:
        raise
    except Exception as e:
        processing_time = (time.time() - start_time) * 1000
        return AnalysisResponse(
            success=False,
            processing_time_ms=processing_time,
            result={},
            error=str(e)
        )

@app.post("/analyze/video", response_model=AnalysisResponse)
async def analyze_video(request: VideoAnalysisRequest):
    """Analyze video data for detailed motion analysis"""
    start_time = time.time()
    
    try:
        if motion_analyzer is None:
            raise HTTPException(status_code=503, detail="Motion analyzer not initialized")
        
        # Decode base64 video data
        try:
            video_data = base64.b64decode(request.video_data)
        except Exception as e:
            raise HTTPException(status_code=400, detail=f"Invalid base64 data: {e}")
        
        # Analyze video using detailed motion analyzer
        result = motion_analyzer.analyze_video_data(video_data)
        
        processing_time = (time.time() - start_time) * 1000
        
        return AnalysisResponse(
            success=True,
            processing_time_ms=processing_time,
            result=result
        )
        
    except HTTPException:
        raise
    except Exception as e:
        processing_time = (time.time() - start_time) * 1000
        return AnalysisResponse(
            success=False,
            processing_time_ms=processing_time,
            result={},
            error=str(e)
        )

@app.post("/analyze/batch", response_model=AnalysisResponse)
async def analyze_batch(request: BatchAnalysisRequest):
    """Analyze full workout session from video file"""
    start_time = time.time()
    
    try:
        # Verify video file exists
        video_path = Path(request.video_path)
        if not video_path.exists():
            raise HTTPException(status_code=404, detail=f"Video file not found: {request.video_path}")
        
        # Run batch analyzer as subprocess
        try:
            cmd = ["python3", "batch_analyzer.py", str(video_path)]
            result = subprocess.run(
                cmd, 
                capture_output=True, 
                text=True, 
                timeout=300  # 5 minute timeout
            )
            
            if result.returncode != 0:
                raise Exception(f"Batch analysis failed: {result.stderr}")
            
            # Parse JSON result
            analysis_result = json.loads(result.stdout)
            
        except subprocess.TimeoutExpired:
            raise HTTPException(status_code=408, detail="Analysis timeout - video too long")
        except json.JSONDecodeError as e:
            raise HTTPException(status_code=500, detail=f"Invalid analysis output: {e}")
        
        processing_time = (time.time() - start_time) * 1000
        
        return AnalysisResponse(
            success=True,
            processing_time_ms=processing_time,
            result=analysis_result
        )
        
    except HTTPException:
        raise
    except Exception as e:
        processing_time = (time.time() - start_time) * 1000
        return AnalysisResponse(
            success=False,
            processing_time_ms=processing_time,
            result={},
            error=str(e)
        )

@app.get("/models/status")
async def models_status():
    """Get status of loaded ML models"""
    return {
        "motion_analyzer": motion_analyzer is not None,
        "realtime_analyzer": True,  # Always available
        "batch_analyzer": Path("batch_analyzer.py").exists(),
        "mediapipe_available": True,
        "pytorch_available": motion_analyzer is not None
    }

@app.get("/")
async def root():
    """Root endpoint with service information"""
    return {
        "service": "Fitness Advisor ML Service",
        "version": "1.0.0",
        "endpoints": {
            "health": "/health",
            "analyze_frame": "/analyze/frame",
            "analyze_video": "/analyze/video", 
            "analyze_batch": "/analyze/batch",
            "models_status": "/models/status"
        },
        "documentation": "/docs"
    }

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Start ML Service")
    parser.add_argument("--host", default="127.0.0.1", help="Host to bind to")
    parser.add_argument("--port", type=int, default=8001, help="Port to bind to")
    parser.add_argument("--workers", type=int, default=1, help="Number of workers")
    
    args = parser.parse_args()
    
    print(f"Starting Fitness ML Service on {args.host}:{args.port}")
    
    uvicorn.run(
        "ml_service:app",
        host=args.host,
        port=args.port,
        workers=args.workers,
        reload=False  # Disable reload in production
    )