// src/api/client.ts - API client for Rust backend integration

const API_BASE_URL = '/api'; // Proxied to localhost:3000
const MCP_BASE_URL = '/mcp'; // MCP server endpoint

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

// MCP-related interfaces
export interface MCPUserProfile {
  id?: string;
  age: number;
  weight_kg: number;
  height_cm: number;
  gender: 'male' | 'female';
  activity_level: 'sedentary' | 'lightly_active' | 'moderately_active' | 'very_active' | 'super_active';
  fitness_goals: string[];
  dietary_restrictions?: string[];
  health_conditions?: string[];
}

export interface WorkoutPlanRequest {
  user_profile: MCPUserProfile;
  workout_preferences?: {
    duration_minutes?: number;
    difficulty_level?: 'beginner' | 'intermediate' | 'advanced' | 'expert';
    equipment_available?: string[];
    workout_type?: 'strength' | 'cardio' | 'flexibility' | 'mixed';
  };
}

export interface NutritionAnalysisRequest {
  foods: Array<{
    name: string;
    quantity: number;
    unit: string;
    meal_timing?: string;
  }>;
  analysis_type?: 'basic' | 'micronutrients' | 'interactions' | 'timing';
}

export interface NutritionPlanRequest {
  user_profile: MCPUserProfile;
  calorie_target?: number;
  meal_preferences?: {
    meals_per_day?: number;
    prep_time_minutes?: number;
    cuisine_preferences?: string[];
    avoid_ingredients?: string[];
    macro_split?: {
      protein_percent: number;
      carbohydrate_percent: number;
      fat_percent: number;
    };
  };
}

export interface ProgressMetric {
  name: string;
  value: number;
  unit: string;
  date: string;
  notes?: string;
}

export interface ProgressTrackingRequest {
  user_id: string;
  metrics: ProgressMetric[];
  time_range_days?: number;
}

export interface SeasonalOptimizationRequest {
  location: string;
  season: 'spring' | 'summer' | 'fall' | 'winter';
  indoor_preference: boolean;
  user_profile: MCPUserProfile;
}

export interface JsonRpcRequest {
  jsonrpc: '2.0';
  method: string;
  params?: any;
  id: string | number;
}

export interface JsonRpcResponse {
  jsonrpc: '2.0';
  result?: any;
  error?: {
    code: number;
    message: string;
    data?: any;
  };
  id: string | number;
}

// Helper function to convert existing User to MCPUserProfile
export function userToMCPProfile(user: User): MCPUserProfile {
  return {
    id: user.id,
    age: user.age,
    weight_kg: user.weight,
    height_cm: user.height,
    gender: 'male', // Default - could be enhanced to store in User interface
    activity_level: user.fitness_level.toLowerCase() === 'beginner' ? 'lightly_active' :
                   user.fitness_level.toLowerCase() === 'intermediate' ? 'moderately_active' :
                   user.fitness_level.toLowerCase() === 'advanced' ? 'very_active' : 'super_active',
    fitness_goals: user.goals,
    dietary_restrictions: [],
    health_conditions: []
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

  // MCP (Model Context Protocol) Integration
  private static requestId = 1;
  private static mcpWebSocket: WebSocket | null = null;
  private static pendingMCPRequests = new Map<string | number, {
    resolve: (value: any) => void;
    reject: (error: any) => void;
    timeout?: NodeJS.Timeout;
  }>();

  // Initialize MCP connection
  static async initializeMCP(): Promise<void> {
    try {
      const response = await this.sendMCPRequest('initialize', {
        protocolVersion: '2025-06-18',
        capabilities: {
          experimental: {},
          sampling: {}
        },
        clientInfo: {
          name: 'Fitness Advisor AI Frontend',
          version: '1.0.0'
        }
      });
      console.log('MCP initialized:', response);
    } catch (error) {
      console.error('Failed to initialize MCP:', error);
      throw error;
    }
  }

  // Create MCP WebSocket connection
  static createMCPWebSocket(): WebSocket {
    if (this.mcpWebSocket && this.mcpWebSocket.readyState === WebSocket.OPEN) {
      return this.mcpWebSocket;
    }

    this.mcpWebSocket = new WebSocket('ws://localhost:8080/mcp');
    
    this.mcpWebSocket.onopen = () => {
      console.log('MCP WebSocket connected');
    };

    this.mcpWebSocket.onmessage = (event) => {
      try {
        const response: JsonRpcResponse = JSON.parse(event.data);
        this.handleMCPResponse(response);
      } catch (error) {
        console.error('Failed to parse MCP WebSocket message:', error);
      }
    };

    this.mcpWebSocket.onclose = () => {
      console.log('MCP WebSocket connection closed');
      // Attempt reconnection after 3 seconds
      setTimeout(() => {
        if (!this.mcpWebSocket || this.mcpWebSocket.readyState === WebSocket.CLOSED) {
          this.createMCPWebSocket();
        }
      }, 3000);
    };

    this.mcpWebSocket.onerror = (error) => {
      console.error('MCP WebSocket error:', error);
    };

    return this.mcpWebSocket;
  }

  private static handleMCPResponse(response: JsonRpcResponse): void {
    const pending = this.pendingMCPRequests.get(response.id);
    if (pending) {
      if (pending.timeout) {
        clearTimeout(pending.timeout);
      }
      
      if (response.error) {
        pending.reject(new Error(`MCP Error ${response.error.code}: ${response.error.message}`));
      } else {
        pending.resolve(response.result);
      }
      
      this.pendingMCPRequests.delete(response.id);
    }
  }

  private static async sendMCPRequest(method: string, params?: any): Promise<any> {
    const requestId = this.requestId++;
    const request: JsonRpcRequest = {
      jsonrpc: '2.0',
      method,
      params,
      id: requestId
    };

    return new Promise((resolve, reject) => {
      // Set up timeout (30 seconds)
      const timeout = setTimeout(() => {
        this.pendingMCPRequests.delete(requestId);
        reject(new Error(`MCP request timeout for method: ${method}`));
      }, 30000);

      this.pendingMCPRequests.set(requestId, { resolve, reject, timeout });

      // Try WebSocket first, fallback to HTTP
      if (this.mcpWebSocket && this.mcpWebSocket.readyState === WebSocket.OPEN) {
        this.mcpWebSocket.send(JSON.stringify(request));
      } else {
        // HTTP fallback
        this.sendMCPHttpRequest(request).then(resolve).catch(reject);
      }
    });
  }

  private static async sendMCPHttpRequest(request: JsonRpcRequest): Promise<any> {
    const response = await fetch(`${MCP_BASE_URL}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const jsonResponse: JsonRpcResponse = await response.json();
    
    if (jsonResponse.error) {
      throw new Error(`MCP Error ${jsonResponse.error.code}: ${jsonResponse.error.message}`);
    }

    return jsonResponse.result;
  }

  // MCP Tool Methods

  static async listMCPTools(): Promise<any> {
    return await this.sendMCPRequest('tools/list');
  }

  static async createWorkoutPlan(request: WorkoutPlanRequest): Promise<any> {
    return await this.sendMCPRequest('tools/call', {
      name: 'create_workout_plan',
      arguments: request
    });
  }

  static async analyzeNutrition(request: NutritionAnalysisRequest): Promise<any> {
    return await this.sendMCPRequest('tools/call', {
      name: 'analyze_nutrition',
      arguments: request
    });
  }

  static async createNutritionPlan(request: NutritionPlanRequest): Promise<any> {
    return await this.sendMCPRequest('tools/call', {
      name: 'create_nutrition_plan',
      arguments: request
    });
  }

  static async trackProgress(request: ProgressTrackingRequest): Promise<any> {
    return await this.sendMCPRequest('tools/call', {
      name: 'track_progress',
      arguments: request
    });
  }

  static async optimizeForSeason(request: SeasonalOptimizationRequest): Promise<any> {
    return await this.sendMCPRequest('tools/call', {
      name: 'optimize_for_season',
      arguments: request
    });
  }

  static async queryRAGFitness(query: string): Promise<any> {
    return await this.sendMCPRequest('tools/call', {
      name: 'rag_fitness_query',
      arguments: { query }
    });
  }

  // MCP Connection Management
  static getMCPConnectionStatus(): 'connected' | 'connecting' | 'disconnected' | 'error' {
    if (this.mcpWebSocket) {
      switch (this.mcpWebSocket.readyState) {
        case WebSocket.OPEN:
          return 'connected';
        case WebSocket.CONNECTING:
          return 'connecting';
        case WebSocket.CLOSED:
        case WebSocket.CLOSING:
          return 'disconnected';
        default:
          return 'error';
      }
    }
    return 'disconnected';
  }

  static async checkMCPHealth(): Promise<boolean> {
    try {
      await this.sendMCPRequest('tools/list');
      return true;
    } catch (error) {
      console.error('MCP health check failed:', error);
      return false;
    }
  }
}