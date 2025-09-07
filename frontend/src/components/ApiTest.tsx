import { useState, useEffect } from 'react';

interface HealthResponse {
  success: boolean;
  data: string;
  message: string;
}

interface User {
  id: string;
  name: string;
  age: number;
  height: number;
  weight: number;
  fitness_level: string;
  goals: string[];
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

  useEffect(() => {
    testBackendConnection();
  }, []);

  return (
    <div className="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white p-6">
      <h3 className="text-lg font-semibold mb-4">🔗 Backend API Test</h3>
      
      <button 
        onClick={testBackendConnection}
        disabled={loading}
        className="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 px-4 py-2 rounded-lg mb-4"
      >
        {loading ? 'Testing...' : 'Test Backend Connection'}
      </button>

      {error && (
        <div className="bg-red-600/20 border border-red-500/30 rounded-lg p-3 mb-4">
          <p className="text-red-300">{error}</p>
        </div>
      )}

      {health && (
        <div className="bg-green-600/20 border border-green-500/30 rounded-lg p-3 mb-4">
          <p className="text-green-300">✅ Backend Health: {health}</p>
        </div>
      )}

      {users.length > 0 && (
        <div>
          <h4 className="text-md font-medium mb-2">👥 Users in Database ({users.length})</h4>
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
  );
}