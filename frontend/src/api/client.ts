// src/api/client.ts - API client for Rust backend integration

const API_BASE_URL = '/api'; // Proxied to localhost:3000

export interface User {
  id: string;
  name: string;
  age: number;
  height: number;
  weight: number;
  fitness_level: 'Beginner' | 'Intermediate' | 'Advanced' | 'Elite';
  goals: string[];
}

export interface OptimizationRequest {
  user_id: string;
  time_horizon_days: number;
  objectives: string[];
  constraints: {
    daily_calories: {
      min: number;
      max: number;
      target: number;
    };
    macros: {
      protein_g: { min: number; max: number };
      carbs_g: { min: number; max: number };
      fat_g: { min: number; max: number };
    };
  };
}

export class FitnessApiClient {
  // User management
  static async getUsers(): Promise<User[]> {
    const response = await fetch(`${API_BASE_URL}/users`);
    const data = await response.json();
    return data.data || [];
  }

  static async getUser(userId: string): Promise<User | null> {
    try {
      const response = await fetch(`${API_BASE_URL}/users/${userId}`);
      const data = await response.json();
      return data.data || null;
    } catch (error) {
      console.error('Failed to fetch user:', error);
      return null;
    }
  }

  static async getUserRecommendations(userId: string) {
    const response = await fetch(`${API_BASE_URL}/users/${userId}/recommendations`);
    const data = await response.json();
    return data.data || [];
  }

  static async getUserProgress(userId: string) {
    const response = await fetch(`${API_BASE_URL}/users/${userId}/progress`);
    const data = await response.json();
    return data.data || {};
  }

  // Menu optimization
  static async optimizeMealPlan(request: OptimizationRequest) {
    const response = await fetch(`${API_BASE_URL}/menu/optimize`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });
    const data = await response.json();
    return data.data || null;
  }

  // ML Analysis
  static async analyzeFrame(frameBase64: string, analysisType = 'realtime') {
    const response = await fetch(`${API_BASE_URL}/ml/analyze-frame`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        frame_base64: frameBase64,
        analysis_type: analysisType,
      }),
    });
    const data = await response.json();
    return data.data || null;
  }

  // Health checks
  static async checkHealth() {
    try {
      const response = await fetch(`${API_BASE_URL}/health`);
      const data = await response.json();
      return data.success || false;
    } catch (error) {
      console.error('Health check failed:', error);
      return false;
    }
  }

  static async checkMLServiceStatus() {
    try {
      const response = await fetch(`${API_BASE_URL}/ml/status`);
      const data = await response.json();
      return data.data || { available: false };
    } catch (error) {
      console.error('ML service check failed:', error);
      return { available: false };
    }
  }

  // WebSocket for real-time analysis
  static createWebSocket(onMessage: (data: any) => void): WebSocket {
    const ws = new WebSocket('ws://localhost:3000/api/ai/realtime');
    
    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        onMessage(data);
      } catch (error) {
        console.error('WebSocket message parse error:', error);
      }
    };

    return ws;
  }
}