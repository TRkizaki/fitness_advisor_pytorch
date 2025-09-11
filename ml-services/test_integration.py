#!/usr/bin/env python3
"""
Integration test for Fitness Advisor AI hybrid system
Tests the communication between Rust API and Python ML service
"""

import asyncio
import aiohttp
import base64
import json
import time
from pathlib import Path

# Test configuration
RUST_API_BASE = "http://localhost:3000"
PYTHON_ML_BASE = "http://localhost:8001"

async def test_ml_service_health():
    """Test Python ML service health endpoint"""
    print("Testing ML service health...")
    
    async with aiohttp.ClientSession() as session:
        try:
            async with session.get(f"{PYTHON_ML_BASE}/health") as response:
                if response.status == 200:
                    data = await response.json()
                    print(f"✅ ML service health: {data['status']}")
                    print(f"   Models loaded: {data['models_loaded']}")
                    return True
                else:
                    print(f"❌ ML service health check failed: {response.status}")
                    return False
        except Exception as e:
            print(f"❌ ML service not reachable: {e}")
            return False

async def test_rust_api_health():
    """Test Rust API health endpoint"""
    print("Testing Rust API health...")
    
    async with aiohttp.ClientSession() as session:
        try:
            async with session.get(f"{RUST_API_BASE}/api/health") as response:
                if response.status == 200:
                    data = await response.json()
                    print(f"✅ Rust API health: {data['status']}")
                    return True
                else:
                    print(f"❌ Rust API health check failed: {response.status}")
                    return False
        except Exception as e:
            print(f"❌ Rust API not reachable: {e}")
            return False

async def test_ml_integration():
    """Test ML integration through Rust API"""
    print("Testing ML integration through Rust API...")
    
    # Create a simple test image (base64 encoded 1x1 pixel)
    test_image_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAGAWA0ddwAAAABJRU5ErkJggg=="
    
    payload = {
        "frame_base64": test_image_data
    }
    
    async with aiohttp.ClientSession() as session:
        try:
            async with session.post(f"{RUST_API_BASE}/api/ml/analyze-frame", json=payload) as response:
                if response.status == 200:
                    data = await response.json()
                    if data['success']:
                        result = data['data']
                        print(f"✅ ML frame analysis successful")
                        print(f"   Processing time: {result.get('processing_time_ms', 'N/A')}ms")
                        print(f"   Result keys: {list(result.keys())}")
                        return True
                    else:
                        print(f"❌ ML analysis failed: {data.get('message', 'Unknown error')}")
                        return False
                else:
                    error_text = await response.text()
                    print(f"❌ ML integration test failed: {response.status}")
                    print(f"   Error: {error_text}")
                    return False
        except Exception as e:
            print(f"❌ ML integration test error: {e}")
            return False

async def test_ml_service_status():
    """Test ML service status through Rust API"""
    print("Testing ML service status through Rust API...")
    
    async with aiohttp.ClientSession() as session:
        try:
            async with session.get(f"{RUST_API_BASE}/api/ml/status") as response:
                if response.status == 200:
                    data = await response.json()
                    if data['success']:
                        status = data['data']
                        print(f"✅ ML service status retrieved")
                        print(f"   Motion analyzer: {status.get('motion_analyzer', False)}")
                        print(f"   Realtime analyzer: {status.get('realtime_analyzer', False)}")
                        print(f"   PyTorch available: {status.get('pytorch_available', False)}")
                        return True
                    else:
                        print(f"❌ ML status check failed: {data.get('message', 'Unknown error')}")
                        return False
                else:
                    print(f"❌ ML status request failed: {response.status}")
                    return False
        except Exception as e:
            print(f"❌ ML status test error: {e}")
            return False

async def test_user_creation():
    """Test user creation through Rust API"""
    print("Testing user creation...")
    
    user_data = {
        "user": {
            "id": "test_user_integration",
            "name": "Integration Test User",
            "age": 25,
            "height": 170.0,
            "weight": 65.0,
            "fitness_level": "Intermediate",
            "goals": ["MuscleGain"],
            "preferences": {
                "preferred_exercise_types": ["Strength"],
                "available_equipment": ["None"],
                "workout_duration_minutes": 30,
                "workouts_per_week": 3,
                "preferred_time_of_day": "morning"
            }
        }
    }
    
    async with aiohttp.ClientSession() as session:
        try:
            async with session.post(f"{RUST_API_BASE}/api/users", json=user_data) as response:
                if response.status == 200:
                    data = await response.json()
                    if data['success']:
                        print(f"✅ User created successfully")
                        return True
                    else:
                        print(f"❌ User creation failed: {data.get('message', 'Unknown error')}")
                        return False
                else:
                    error_text = await response.text()
                    print(f"❌ User creation request failed: {response.status}")
                    print(f"   Error: {error_text}")
                    return False
        except Exception as e:
            print(f"❌ User creation test error: {e}")
            return False

async def run_integration_tests():
    """Run all integration tests"""
    print("🚀 Starting Fitness Advisor AI Integration Tests")
    print("=" * 60)
    
    tests = [
        ("ML Service Health", test_ml_service_health),
        ("Rust API Health", test_rust_api_health),
        ("User Creation", test_user_creation),
        ("ML Service Status", test_ml_service_status),
        ("ML Integration", test_ml_integration),
    ]
    
    results = []
    
    for test_name, test_func in tests:
        print(f"\n📋 Running: {test_name}")
        print("-" * 40)
        start_time = time.time()
        
        try:
            result = await test_func()
            duration = time.time() - start_time
            results.append((test_name, result, duration))
            
            status = "✅ PASSED" if result else "❌ FAILED"
            print(f"   {status} ({duration:.2f}s)")
            
        except Exception as e:
            duration = time.time() - start_time
            results.append((test_name, False, duration))
            print(f"   ❌ FAILED ({duration:.2f}s): {e}")
    
    # Summary
    print("\n" + "=" * 60)
    print("📊 Test Results Summary")
    print("=" * 60)
    
    passed = sum(1 for _, result, _ in results if result)
    total = len(results)
    
    for test_name, result, duration in results:
        status = "✅ PASS" if result else "❌ FAIL"
        print(f"{status:8} {test_name:25} ({duration:.2f}s)")
    
    print("-" * 60)
    print(f"Total: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed! Integration is working correctly.")
        return True
    else:
        print(f"⚠️  {total - passed} tests failed. Check service configuration.")
        return False

if __name__ == "__main__":
    print("Fitness Advisor AI - Integration Test Suite")
    print("Make sure both services are running:")
    print("  - Rust API server: http://localhost:3000")
    print("  - Python ML service: http://localhost:8001")
    print()
    
    success = asyncio.run(run_integration_tests())
    exit(0 if success else 1)