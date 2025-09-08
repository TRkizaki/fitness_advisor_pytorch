import { useState, useEffect } from 'react';
import { FitnessApiClient, User, userToMCPProfile, WorkoutPlanRequest } from '../api/client';
import { WebSocketStatus } from './WebSocketStatus';
import { MCPErrorDisplay, useMCPOperation } from './MCPErrorDisplay';
import { ErrorMonitoring } from './ErrorMonitoring';
import { MCPErrorHandler } from '../utils/mcpErrorHandler';

interface HealthResponse {
  success: boolean;
  data: string;
  message: string;
}

interface UsersResponse {
  success: boolean;
  data: User[];
  message: string;
}

export function ApiTest() {
  const [health, setHealth] = useState<string>('');
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');
  
  // MCP-specific state
  const [mcpHealth, setMcpHealth] = useState<boolean | null>(null);
  const [mcpTools, setMcpTools] = useState<any[]>([]);
  const [mcpStatus, setMcpStatus] = useState<'connected' | 'connecting' | 'disconnected' | 'error'>('disconnected');
  const [workoutPlan, setWorkoutPlan] = useState<any>(null);
  const [mcpLoading, setMcpLoading] = useState(false);

  const testBackendConnection = async () => {
    setLoading(true);
    setError('');
    
    try {
      // Test health endpoint
      const healthResponse = await fetch('http://localhost:3000/api/health');
      const healthData: HealthResponse = await healthResponse.json();
      
      if (healthData.success) {
        setHealth(healthData.data);
      }

      // Test users endpoint
      const usersResponse = await fetch('http://localhost:3000/api/users');
      const usersData: UsersResponse = await usersResponse.json();
      
      if (usersData.success) {
        setUsers(usersData.data);
      }
      
    } catch (err) {
      setError(`Failed to connect to backend: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const testMCPConnection = async () => {
    setMcpLoading(true);
    setError('');
    
    try {
      // Initialize MCP connection
      await FitnessApiClient.initializeMCP();
      
      // Check MCP health
      const healthStatus = await FitnessApiClient.checkMCPHealth();
      setMcpHealth(healthStatus);
      
      // Get available tools
      const toolsResponse = await FitnessApiClient.listMCPTools();
      setMcpTools(toolsResponse.tools || []);
      
      // Update connection status
      setMcpStatus(FitnessApiClient.getMCPConnectionStatus());
      
    } catch (err) {
      const mcpError = MCPErrorHandler.handleMCPError(err, 'MCP Connection Test');
      setError(MCPErrorHandler.getErrorMessage(mcpError));
      setMcpHealth(false);
      setMcpStatus('error');
    } finally {
      setMcpLoading(false);
    }
  };

  const testWorkoutPlanGeneration = async () => {
    if (users.length === 0) {
      setError('No users available for workout plan generation');
      return;
    }

    setMcpLoading(true);
    try {
      const testUser = users[0];
      const mcpProfile = userToMCPProfile(testUser);
      
      const workoutRequest: WorkoutPlanRequest = {
        user_profile: mcpProfile,
        workout_preferences: {
          duration_minutes: 45,
          difficulty_level: 'intermediate',
          equipment_available: ['dumbbells', 'barbell'],
          workout_type: 'strength'
        }
      };

      const result = await FitnessApiClient.createWorkoutPlan(workoutRequest);
      setWorkoutPlan(result);
      
    } catch (err) {
      const mcpError = MCPErrorHandler.handleMCPError(err, 'Workout Plan Generation');
      setError(MCPErrorHandler.getErrorMessage(mcpError));
    } finally {
      setMcpLoading(false);
    }
  };

  const initializeMCPWebSocket = () => {
    try {
      FitnessApiClient.createMCPWebSocket();
      // Update status periodically
      const interval = setInterval(() => {
        setMcpStatus(FitnessApiClient.getMCPConnectionStatus());
        if (FitnessApiClient.getMCPConnectionStatus() === 'connected') {
          clearInterval(interval);
        }
      }, 1000);
    } catch (err) {
      const mcpError = MCPErrorHandler.handleMCPError(err, 'MCP WebSocket Creation');
      setError(MCPErrorHandler.getErrorMessage(mcpError));
    }
  };

  useEffect(() => {
    testBackendConnection();
  }, []);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'connected': return 'text-green-300 bg-green-600/20 border-green-500/30';
      case 'connecting': return 'text-yellow-300 bg-yellow-600/20 border-yellow-500/30';
      case 'error': return 'text-red-300 bg-red-600/20 border-red-500/30';
      default: return 'text-gray-300 bg-gray-600/20 border-gray-500/30';
    }
  };

  return (
    <div className="space-y-6">
      {/* WebSocket Status */}
      <WebSocketStatus />
      
      {/* Original Backend API Test */}
      <div className="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white p-6">
        <h3 className="text-lg font-semibold mb-4">Backend API Test</h3>
        
        <button 
          onClick={testBackendConnection}
          disabled={loading}
          className="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 px-4 py-2 rounded-lg mb-4"
        >
          {loading ? 'Testing...' : 'Test Backend Connection'}
        </button>

        {error && (
          <MCPErrorDisplay
            error={error}
            context="Backend API Test"
            onDismiss={() => setError('')}
          />
        )}

        {health && (
          <div className="bg-green-600/20 border border-green-500/30 rounded-lg p-3 mb-4">
            <p className="text-green-300">Backend Health: {health}</p>
          </div>
        )}

        {users.length > 0 && (
          <div>
            <h4 className="text-md font-medium mb-2">Users in Database ({users.length})</h4>
            <div className="space-y-2">
              {users.map((user) => (
                <div key={user.id} className="bg-white/5 rounded-lg p-3">
                  <p className="font-medium">{user.name}</p>
                  <p className="text-sm text-white/70">
                    {user.age}yo • {user.fitness_level} • {user.goals.join(', ')}
                  </p>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* MCP Server Test */}
      <div className="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white p-6">
        <h3 className="text-lg font-semibold mb-4">MCP Server Test</h3>
        
        <div className="flex flex-wrap gap-3 mb-4">
          <button 
            onClick={testMCPConnection}
            disabled={mcpLoading}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 px-4 py-2 rounded-lg"
          >
            {mcpLoading ? 'Testing...' : 'Test MCP Connection'}
          </button>
          
          <button 
            onClick={initializeMCPWebSocket}
            disabled={mcpLoading}
            className="bg-green-600 hover:bg-green-700 disabled:bg-gray-600 px-4 py-2 rounded-lg"
          >
            Connect WebSocket
          </button>

          <button 
            onClick={testWorkoutPlanGeneration}
            disabled={mcpLoading || users.length === 0}
            className="bg-orange-600 hover:bg-orange-700 disabled:bg-gray-600 px-4 py-2 rounded-lg"
          >
            {mcpLoading ? 'Generating...' : 'Generate Workout Plan'}
          </button>
        </div>

        {/* MCP Status */}
        <div className={`rounded-lg p-3 mb-4 border ${getStatusColor(mcpStatus)}`}>
          <p>MCP Status: <span className="font-medium">{mcpStatus}</span></p>
          {mcpHealth !== null && (
            <p>Health Check: <span className={mcpHealth ? 'text-green-300' : 'text-red-300'}>
              {mcpHealth ? 'Healthy' : 'Failed'}
            </span></p>
          )}
        </div>

        {/* Available MCP Tools */}
        {mcpTools.length > 0 && (
          <div className="mb-4">
            <h4 className="text-md font-medium mb-2">Available MCP Tools ({mcpTools.length})</h4>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
              {mcpTools.map((tool, index) => (
                <div key={index} className="bg-white/5 rounded-lg p-3">
                  <p className="font-medium text-sm">{tool.name}</p>
                  <p className="text-xs text-white/70">{tool.description}</p>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Generated Workout Plan */}
        {workoutPlan && (
          <div className="bg-blue-600/10 border border-blue-500/30 rounded-lg p-4">
            <h4 className="text-md font-medium mb-2">Generated Workout Plan</h4>
            <div className="bg-black/20 rounded p-3">
              <pre className="text-xs whitespace-pre-wrap text-white/80 overflow-auto max-h-64">
                {typeof workoutPlan === 'string' 
                  ? workoutPlan 
                  : JSON.stringify(workoutPlan, null, 2)
                }
              </pre>
            </div>
          </div>
        )}
      </div>

      {/* Error Monitoring Dashboard */}
      <ErrorMonitoring />
    </div>
  );
}