import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";
import { LineChart, Line, AreaChart, Area, XAxis, YAxis, CartesianGrid, ResponsiveContainer, Tooltip } from "recharts";
import { TrendingUp, Weight, Zap, Target, BarChart3, Calendar, Plus } from "lucide-react";
import { FitnessApiClient, User, ProgressMetric, ProgressTrackingRequest } from "../api/client";

const weightData = [
  { month: "Jan", weight: 180 },
  { month: "Feb", weight: 178 },
  { month: "Mar", weight: 175 },
  { month: "Apr", weight: 173 },
  { month: "May", weight: 171 },
  { month: "Jun", weight: 169 },
];

const workoutData = [
  { day: "Mon", calories: 420 },
  { day: "Tue", calories: 380 },
  { day: "Wed", calories: 450 },
  { day: "Thu", calories: 320 },
  { day: "Fri", calories: 490 },
  { day: "Sat", calories: 380 },
  { day: "Sun", calories: 410 },
];

const strengthData = [
  { month: "Jan", bench: 185, squat: 225, deadlift: 275 },
  { month: "Feb", bench: 190, squat: 235, deadlift: 285 },
  { month: "Mar", bench: 195, squat: 245, deadlift: 295 },
  { month: "Apr", bench: 200, squat: 255, deadlift: 305 },
  { month: "May", bench: 205, squat: 265, deadlift: 315 },
  { month: "Jun", bench: 210, squat: 275, deadlift: 325 },
];

export function ProgressCharts() {
  const [users, setUsers] = useState<User[]>([]);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [progressData, setProgressData] = useState<any>(null);
  const [isTracking, setIsTracking] = useState(false);
  const [error, setError] = useState<string>('');
  const [newMetric, setNewMetric] = useState<ProgressMetric>({
    name: '',
    value: 0,
    unit: '',
    date: new Date().toISOString().split('T')[0],
    notes: ''
  });

  // Load users on component mount
  useEffect(() => {
    const loadUsers = async () => {
      try {
        const userList = await FitnessApiClient.getUsers();
        setUsers(userList);
        if (userList.length > 0) {
          setSelectedUser(userList[0]);
        }
      } catch (err) {
        setError('Failed to load users');
      }
    };
    loadUsers();
  }, []);

  const trackProgress = async (metrics: ProgressMetric[]) => {
    if (!selectedUser) {
      setError('No user selected');
      return;
    }

    setIsTracking(true);
    setError('');

    try {
      const trackingRequest: ProgressTrackingRequest = {
        user_id: selectedUser.id,
        metrics: metrics,
        time_range_days: 30
      };

      const result = await FitnessApiClient.trackProgress(trackingRequest);
      setProgressData(result);
      
    } catch (err) {
      setError(`Failed to track progress: ${err}`);
    } finally {
      setIsTracking(false);
    }
  };

  const addMetric = () => {
    if (!newMetric.name || !newMetric.value || !newMetric.unit) {
      setError('Please fill in all metric fields');
      return;
    }

    trackProgress([newMetric]);
    
    // Reset form
    setNewMetric({
      name: '',
      value: 0,
      unit: '',
      date: new Date().toISOString().split('T')[0],
      notes: ''
    });
  };

  const renderProgressAnalysis = () => {
    if (!progressData) return null;

    const analysisText = progressData.content?.[0]?.text || JSON.stringify(progressData, null, 2);
    
    return (
      <div className="bg-gradient-to-r from-blue-600/10 to-green-600/10 border border-blue-500/30 rounded-lg p-4">
        <div className="flex items-center justify-between mb-3">
          <h4 className="text-md font-medium">Progress Analysis</h4>
          <Badge variant="outline" className="text-blue-300 border-blue-400/50">
            MCP Analyzed
          </Badge>
        </div>
        <div className="bg-black/20 rounded p-3 max-h-64 overflow-auto">
          <pre className="text-xs whitespace-pre-wrap text-white/90">
            {analysisText}
          </pre>
        </div>
      </div>
    );
  };

  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <BarChart3 className="h-5 w-5" />
            Progress Analytics
          </div>
          {selectedUser && (
            <div className="flex items-center gap-2">
              <span className="text-sm text-white/70">Active User:</span>
              <Badge variant="outline" className="text-blue-300 border-blue-400/50">
                {selectedUser.name}
              </Badge>
            </div>
          )}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <Tabs defaultValue="charts" className="w-full">
          <TabsList className="grid w-full grid-cols-3 bg-white/10">
            <TabsTrigger value="charts" className="data-[state=active]:bg-white/20">
              <BarChart3 className="h-4 w-4 mr-2" />
              Charts
            </TabsTrigger>
            <TabsTrigger value="tracking" className="data-[state=active]:bg-white/20">
              <Target className="h-4 w-4 mr-2" />
              Track Progress
            </TabsTrigger>
            <TabsTrigger value="goals" className="data-[state=active]:bg-white/20">
              <Calendar className="h-4 w-4 mr-2" />
              Goals
            </TabsTrigger>
          </TabsList>

          {/* Charts Tab */}
          <TabsContent value="charts" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
      {/* Weight Progress */}
      <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Weight className="h-5 w-5" />
            Weight Progress
          </CardTitle>
        </CardHeader>
        <CardContent>
          <ResponsiveContainer width="100%" height={200}>
            <AreaChart data={weightData}>
              <defs>
                <linearGradient id="weightGradient" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#8b5cf6" stopOpacity={0.3}/>
                  <stop offset="95%" stopColor="#8b5cf6" stopOpacity={0}/>
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff20" />
              <XAxis dataKey="month" stroke="#ffffff60" />
              <YAxis stroke="#ffffff60" />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: '#000000aa', 
                  border: '1px solid #ffffff20',
                  borderRadius: '8px',
                  color: '#ffffff'
                }} 
              />
              <Area 
                type="monotone" 
                dataKey="weight" 
                stroke="#8b5cf6" 
                strokeWidth={2}
                fill="url(#weightGradient)" 
              />
            </AreaChart>
          </ResponsiveContainer>
          <div className="mt-4 flex items-center gap-2 text-green-400">
            <TrendingUp className="h-4 w-4" />
            <span className="text-sm">-11 lbs this year</span>
          </div>
        </CardContent>
      </Card>

      {/* Weekly Calories Burned */}
      <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Zap className="h-5 w-5" />
            Calories Burned
          </CardTitle>
        </CardHeader>
        <CardContent>
          <ResponsiveContainer width="100%" height={200}>
            <LineChart data={workoutData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff20" />
              <XAxis dataKey="day" stroke="#ffffff60" />
              <YAxis stroke="#ffffff60" />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: '#000000aa', 
                  border: '1px solid #ffffff20',
                  borderRadius: '8px',
                  color: '#ffffff'
                }} 
              />
              <Line 
                type="monotone" 
                dataKey="calories" 
                stroke="#3b82f6" 
                strokeWidth={3}
                dot={{ fill: '#3b82f6', strokeWidth: 2, r: 4 }}
                activeDot={{ r: 6, stroke: '#3b82f6', strokeWidth: 2, fill: '#ffffff' }}
              />
            </LineChart>
          </ResponsiveContainer>
          <div className="mt-4 flex items-center gap-2 text-blue-400">
            <TrendingUp className="h-4 w-4" />
            <span className="text-sm">2,850 kcal this week</span>
          </div>
        </CardContent>
      </Card>

      {/* Strength Progress */}
      <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white lg:col-span-2 xl:col-span-1">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <TrendingUp className="h-5 w-5" />
            Strength Progress
          </CardTitle>
        </CardHeader>
        <CardContent>
          <ResponsiveContainer width="100%" height={200}>
            <LineChart data={strengthData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff20" />
              <XAxis dataKey="month" stroke="#ffffff60" />
              <YAxis stroke="#ffffff60" />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: '#000000aa', 
                  border: '1px solid #ffffff20',
                  borderRadius: '8px',
                  color: '#ffffff'
                }} 
              />
              <Line 
                type="monotone" 
                dataKey="bench" 
                stroke="#8b5cf6" 
                strokeWidth={2}
                dot={{ fill: '#8b5cf6', strokeWidth: 2, r: 3 }}
              />
              <Line 
                type="monotone" 
                dataKey="squat" 
                stroke="#3b82f6" 
                strokeWidth={2}
                dot={{ fill: '#3b82f6', strokeWidth: 2, r: 3 }}
              />
              <Line 
                type="monotone" 
                dataKey="deadlift" 
                stroke="#10b981" 
                strokeWidth={2}
                dot={{ fill: '#10b981', strokeWidth: 2, r: 3 }}
              />
            </LineChart>
          </ResponsiveContainer>
          <div className="mt-4 grid grid-cols-3 gap-2 text-sm">
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 bg-purple-500 rounded"></div>
              <span className="text-white/80">Bench</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 bg-blue-500 rounded"></div>
              <span className="text-white/80">Squat</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 bg-green-500 rounded"></div>
              <span className="text-white/80">Deadlift</span>
            </div>
          </div>
        </CardContent>
      </Card>
            </div>
          </TabsContent>

          {/* Progress Tracking Tab */}
          <TabsContent value="tracking" className="space-y-4">
            {error && (
              <div className="bg-red-600/20 border border-red-500/30 rounded-lg p-3">
                <p className="text-red-300">{error}</p>
              </div>
            )}

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div className="space-y-4">
                <h3 className="text-lg">Add Progress Metric</h3>
                
                <div className="space-y-3">
                  <div>
                    <label className="text-sm text-white/70">Metric Name</label>
                    <select
                      value={newMetric.name}
                      onChange={(e) => setNewMetric({...newMetric, name: e.target.value})}
                      className="w-full bg-black/40 border border-white/20 rounded px-3 py-2 text-white text-sm mt-1"
                    >
                      <option value="">Select metric...</option>
                      <option value="weight">Weight</option>
                      <option value="body_fat">Body Fat %</option>
                      <option value="muscle_mass">Muscle Mass</option>
                      <option value="bench_press">Bench Press</option>
                      <option value="squat">Squat</option>
                      <option value="deadlift">Deadlift</option>
                      <option value="run_distance">Running Distance</option>
                      <option value="resting_heart_rate">Resting HR</option>
                    </select>
                  </div>

                  <div className="grid grid-cols-2 gap-2">
                    <div>
                      <label className="text-sm text-white/70">Value</label>
                      <input
                        type="number"
                        value={newMetric.value}
                        onChange={(e) => setNewMetric({...newMetric, value: parseFloat(e.target.value) || 0})}
                        className="w-full bg-black/40 border border-white/20 rounded px-3 py-2 text-white text-sm mt-1"
                        placeholder="0"
                      />
                    </div>
                    <div>
                      <label className="text-sm text-white/70">Unit</label>
                      <select
                        value={newMetric.unit}
                        onChange={(e) => setNewMetric({...newMetric, unit: e.target.value})}
                        className="w-full bg-black/40 border border-white/20 rounded px-3 py-2 text-white text-sm mt-1"
                      >
                        <option value="">Unit...</option>
                        <option value="kg">kg</option>
                        <option value="lbs">lbs</option>
                        <option value="%">%</option>
                        <option value="km">km</option>
                        <option value="miles">miles</option>
                        <option value="bpm">bpm</option>
                        <option value="reps">reps</option>
                      </select>
                    </div>
                  </div>

                  <div>
                    <label className="text-sm text-white/70">Date</label>
                    <input
                      type="date"
                      value={newMetric.date}
                      onChange={(e) => setNewMetric({...newMetric, date: e.target.value})}
                      className="w-full bg-black/40 border border-white/20 rounded px-3 py-2 text-white text-sm mt-1"
                    />
                  </div>

                  <div>
                    <label className="text-sm text-white/70">Notes (optional)</label>
                    <textarea
                      value={newMetric.notes}
                      onChange={(e) => setNewMetric({...newMetric, notes: e.target.value})}
                      className="w-full bg-black/40 border border-white/20 rounded px-3 py-2 text-white text-sm mt-1"
                      placeholder="Add any notes about this measurement..."
                      rows={2}
                    />
                  </div>

                  <Button
                    onClick={addMetric}
                    disabled={isTracking || !selectedUser}
                    className="w-full bg-gradient-to-r from-blue-600 to-green-600 hover:from-blue-700 hover:to-green-700"
                  >
                    <Plus className="h-4 w-4 mr-2" />
                    {isTracking ? 'Tracking...' : 'Track Progress'}
                  </Button>
                </div>

                {users.length > 1 && (
                  <div className="space-y-2">
                    <label className="text-sm text-white/70">Select User:</label>
                    <select 
                      value={selectedUser?.id || ''}
                      onChange={(e) => {
                        const user = users.find(u => u.id === e.target.value);
                        setSelectedUser(user || null);
                      }}
                      className="w-full bg-black/40 border border-white/20 rounded px-3 py-2 text-white text-sm"
                    >
                      {users.map(user => (
                        <option key={user.id} value={user.id} className="bg-gray-800">
                          {user.name} - {user.fitness_level}
                        </option>
                      ))}
                    </select>
                  </div>
                )}
              </div>

              <div className="space-y-4">
                {renderProgressAnalysis()}
                
                {!progressData && !isTracking && (
                  <div className="bg-gradient-to-br from-gray-800/50 to-gray-900/50 rounded-lg border border-white/10 p-6 text-center">
                    <Target className="h-12 w-12 text-white/40 mx-auto mb-3" />
                    <p className="text-white/60">Track your progress metrics</p>
                    <p className="text-white/40 text-sm mt-2">
                      AI-powered analysis to identify trends and provide insights
                    </p>
                  </div>
                )}
              </div>
            </div>
          </TabsContent>

          {/* Goals Tab */}
          <TabsContent value="goals" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
              <Card className="bg-green-600/10 border-green-500/30">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-green-300">Weight Loss Goal</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span>Progress</span>
                      <span>11/15 lbs</span>
                    </div>
                    <div className="w-full bg-green-900/30 rounded-full h-2">
                      <div className="bg-green-500 h-2 rounded-full" style={{width: '73%'}}></div>
                    </div>
                    <p className="text-xs text-green-400">4 lbs to go • 73% complete</p>
                  </div>
                </CardContent>
              </Card>
              
              <Card className="bg-blue-600/10 border-blue-500/30">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-blue-300">Bench Press Goal</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span>Current</span>
                      <span>210/225 lbs</span>
                    </div>
                    <div className="w-full bg-blue-900/30 rounded-full h-2">
                      <div className="bg-blue-500 h-2 rounded-full" style={{width: '93%'}}></div>
                    </div>
                    <p className="text-xs text-blue-400">15 lbs to go • 93% complete</p>
                  </div>
                </CardContent>
              </Card>
              
              <Card className="bg-purple-600/10 border-purple-500/30">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-purple-300">Weekly Workouts</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span>This Week</span>
                      <span>4/5 sessions</span>
                    </div>
                    <div className="w-full bg-purple-900/30 rounded-full h-2">
                      <div className="bg-purple-500 h-2 rounded-full" style={{width: '80%'}}></div>
                    </div>
                    <p className="text-xs text-purple-400">1 session to go • 80% complete</p>
                  </div>
                </CardContent>
              </Card>
            </div>

            <div className="space-y-4">
              <h3 className="text-lg">Upcoming Milestones</h3>
              
              <div className="space-y-3">
                <div className="flex items-center gap-4 p-3 bg-yellow-600/10 border border-yellow-500/30 rounded-lg">
                  <div className="w-12 h-12 bg-yellow-600/20 rounded-full flex items-center justify-center">
                    <Target className="h-6 w-6 text-yellow-400" />
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-yellow-300">Target Weight Achievement</p>
                        <p className="text-xs text-white/60">Estimated completion: 3 weeks</p>
                      </div>
                      <Badge variant="outline" className="text-yellow-300 border-yellow-400/50">
                        165 lbs
                      </Badge>
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-4 p-3 bg-green-600/10 border border-green-500/30 rounded-lg">
                  <div className="w-12 h-12 bg-green-600/20 rounded-full flex items-center justify-center">
                    <Weight className="h-6 w-6 text-green-400" />
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-green-300">Strength Milestone</p>
                        <p className="text-xs text-white/60">2 plate bench press goal</p>
                      </div>
                      <Badge variant="outline" className="text-green-300 border-green-400/50">
                        225 lbs
                      </Badge>
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-4 p-3 bg-blue-600/10 border border-blue-500/30 rounded-lg">
                  <div className="w-12 h-12 bg-blue-600/20 rounded-full flex items-center justify-center">
                    <Calendar className="h-6 w-6 text-blue-400" />
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-blue-300">90-Day Challenge</p>
                        <p className="text-xs text-white/60">Complete transformation program</p>
                      </div>
                      <Badge variant="outline" className="text-blue-300 border-blue-400/50">
                        45 days left
                      </Badge>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
}