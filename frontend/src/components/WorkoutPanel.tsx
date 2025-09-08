import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Progress } from "./ui/progress";
import { Badge } from "./ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";
import { Camera, Play, Square, RotateCcw, Dumbbell, Plus, RefreshCw } from "lucide-react";
import { FitnessApiClient, User, userToMCPProfile, WorkoutPlanRequest } from "../api/client";

export function WorkoutPanel() {
  const [users, setUsers] = useState<User[]>([]);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [workoutPlan, setWorkoutPlan] = useState<any>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState<string>('');

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

  const generateWorkoutPlan = async (workoutType: 'strength' | 'cardio' | 'flexibility' | 'mixed' = 'mixed') => {
    if (!selectedUser) {
      setError('No user selected');
      return;
    }

    setIsGenerating(true);
    setError('');

    try {
      const mcpProfile = userToMCPProfile(selectedUser);
      
      const workoutRequest: WorkoutPlanRequest = {
        user_profile: mcpProfile,
        workout_preferences: {
          duration_minutes: 45,
          difficulty_level: selectedUser.fitness_level.toLowerCase() === 'beginner' ? 'beginner' :
                           selectedUser.fitness_level.toLowerCase() === 'intermediate' ? 'intermediate' : 'advanced',
          equipment_available: ['dumbbells', 'barbell', 'resistance_bands', 'bodyweight'],
          workout_type: workoutType
        }
      };

      const result = await FitnessApiClient.createWorkoutPlan(workoutRequest);
      setWorkoutPlan(result);
      
    } catch (err) {
      setError(`Failed to generate workout plan: ${err}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const renderWorkoutPlan = () => {
    if (!workoutPlan) return null;

    const planText = workoutPlan.content?.[0]?.text || JSON.stringify(workoutPlan, null, 2);
    
    return (
      <div className="bg-gradient-to-r from-blue-600/10 to-purple-600/10 border border-blue-500/30 rounded-lg p-4">
        <div className="flex items-center justify-between mb-3">
          <h4 className="text-md font-medium">Generated Workout Plan</h4>
          <Badge variant="outline" className="text-blue-300 border-blue-400/50">
            MCP Generated
          </Badge>
        </div>
        <div className="bg-black/20 rounded p-3 max-h-64 overflow-auto">
          <pre className="text-xs whitespace-pre-wrap text-white/90">
            {planText}
          </pre>
        </div>
        <div className="flex gap-2 mt-3">
          <Button 
            size="sm" 
            onClick={() => generateWorkoutPlan('strength')}
            disabled={isGenerating}
            className="bg-red-600/80 hover:bg-red-600"
          >
            Strength Focus
          </Button>
          <Button 
            size="sm" 
            onClick={() => generateWorkoutPlan('cardio')}
            disabled={isGenerating}
            className="bg-green-600/80 hover:bg-green-600"
          >
            Cardio Focus
          </Button>
          <Button 
            size="sm" 
            onClick={() => generateWorkoutPlan('flexibility')}
            disabled={isGenerating}
            className="bg-purple-600/80 hover:bg-purple-600"
          >
            Flexibility
          </Button>
        </div>
      </div>
    );
  };

  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Dumbbell className="h-5 w-5" />
            Workout Center
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
        <Tabs defaultValue="tracking" className="w-full">
          <TabsList className="grid w-full grid-cols-3 bg-white/10">
            <TabsTrigger value="tracking" className="data-[state=active]:bg-white/20">
              <Camera className="h-4 w-4 mr-2" />
              Live Tracking
            </TabsTrigger>
            <TabsTrigger value="planner" className="data-[state=active]:bg-white/20">
              <Plus className="h-4 w-4 mr-2" />
              Plan Generator
            </TabsTrigger>
            <TabsTrigger value="analysis" className="data-[state=active]:bg-white/20">
              <RefreshCw className="h-4 w-4 mr-2" />
              Form Analysis
            </TabsTrigger>
          </TabsList>

          {/* Live Tracking Tab */}
          <TabsContent value="tracking" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {/* Camera Feed Area */}
              <div className="space-y-4">
                <div className="aspect-video bg-gradient-to-br from-gray-800 to-gray-900 rounded-lg border border-white/20 flex items-center justify-center relative overflow-hidden">
                  <div className="absolute inset-0 bg-gradient-to-br from-purple-500/20 to-blue-500/20"></div>
                  <div className="relative z-10 text-center">
                    <Camera className="h-12 w-12 text-white/60 mx-auto mb-2" />
                    <p className="text-white/80">Camera Feed Active</p>
                    <p className="text-white/60 text-sm">AI Form Analysis Running</p>
                  </div>
                  <div className="absolute top-4 right-4 flex gap-2">
                    <div className="w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
                    <span className="text-xs text-white/80">LIVE</span>
                  </div>
                </div>
                
                <div className="flex gap-2">
                  <Button className="flex-1 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700">
                    <Play className="h-4 w-4 mr-2" />
                    Start Recording
                  </Button>
                  <Button variant="outline" className="border-white/20 text-white hover:bg-white/10">
                    <Square className="h-4 w-4" />
                  </Button>
                  <Button variant="outline" className="border-white/20 text-white hover:bg-white/10">
                    <RotateCcw className="h-4 w-4" />
                  </Button>
                </div>
              </div>
              
              {/* Form Analysis Metrics */}
              <div className="space-y-4">
                <h3 className="text-lg">Form Analysis</h3>
                
                <div className="space-y-4">
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-white/80">Squat Depth</span>
                      <span className="text-green-400">92%</span>
                    </div>
                    <Progress value={92} className="h-2" />
                  </div>
                  
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-white/80">Knee Alignment</span>
                      <span className="text-yellow-400">78%</span>
                    </div>
                    <Progress value={78} className="h-2" />
                  </div>
                  
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-white/80">Back Posture</span>
                      <span className="text-green-400">88%</span>
                    </div>
                    <Progress value={88} className="h-2" />
                  </div>
                  
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-white/80">Rep Tempo</span>
                      <span className="text-blue-400">85%</span>
                    </div>
                    <Progress value={85} className="h-2" />
                  </div>
                </div>
                
                <div className="mt-6 p-4 bg-gradient-to-r from-green-500/20 to-blue-500/20 rounded-lg border border-white/10">
                  <h4 className="text-green-400 mb-2">Current Exercise: Squats</h4>
                  <p className="text-white/80 text-sm">Great form! Focus on knee tracking for optimal performance.</p>
                </div>
              </div>
            </div>
          </TabsContent>

          {/* Plan Generator Tab */}
          <TabsContent value="planner" className="space-y-4">
            {error && (
              <div className="bg-red-600/20 border border-red-500/30 rounded-lg p-3">
                <p className="text-red-300">{error}</p>
              </div>
            )}

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div className="space-y-4">
                <h3 className="text-lg">Workout Plan Generator</h3>
                <p className="text-white/70 text-sm">
                  Generate personalized workout plans using AI-powered MCP tools based on your profile and preferences.
                </p>
                
                <div className="flex flex-wrap gap-2">
                  <Button 
                    onClick={() => generateWorkoutPlan('mixed')}
                    disabled={isGenerating || !selectedUser}
                    className="bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700"
                  >
                    {isGenerating ? <RefreshCw className="h-4 w-4 mr-2 animate-spin" /> : <Plus className="h-4 w-4 mr-2" />}
                    {isGenerating ? 'Generating...' : 'Generate Mixed Workout'}
                  </Button>
                </div>
                
                <div className="grid grid-cols-2 gap-2">
                  <Button 
                    variant="outline"
                    onClick={() => generateWorkoutPlan('strength')}
                    disabled={isGenerating || !selectedUser}
                    className="border-red-500/50 text-red-300 hover:bg-red-600/20"
                  >
                    Strength Focus
                  </Button>
                  <Button 
                    variant="outline"
                    onClick={() => generateWorkoutPlan('cardio')}
                    disabled={isGenerating || !selectedUser}
                    className="border-green-500/50 text-green-300 hover:bg-green-600/20"
                  >
                    Cardio Focus
                  </Button>
                  <Button 
                    variant="outline"
                    onClick={() => generateWorkoutPlan('flexibility')}
                    disabled={isGenerating || !selectedUser}
                    className="border-purple-500/50 text-purple-300 hover:bg-purple-600/20"
                  >
                    Flexibility
                  </Button>
                  <Button 
                    variant="outline"
                    onClick={() => generateWorkoutPlan('mixed')}
                    disabled={isGenerating || !selectedUser}
                    className="border-blue-500/50 text-blue-300 hover:bg-blue-600/20"
                  >
                    Mixed Training
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
                {renderWorkoutPlan()}
                
                {!workoutPlan && !isGenerating && (
                  <div className="bg-gradient-to-br from-gray-800/50 to-gray-900/50 rounded-lg border border-white/10 p-6 text-center">
                    <Dumbbell className="h-12 w-12 text-white/40 mx-auto mb-3" />
                    <p className="text-white/60">Generate a workout plan to get started</p>
                    <p className="text-white/40 text-sm mt-2">
                      AI-powered plans personalized to your fitness level and goals
                    </p>
                  </div>
                )}
              </div>
            </div>
          </TabsContent>

          {/* Form Analysis Tab */}
          <TabsContent value="analysis" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Exercise Recognition</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-white">Squats</div>
                  <p className="text-xs text-white/60">Detected with 95% confidence</p>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Rep Count</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-white">12</div>
                  <p className="text-xs text-white/60">Current set</p>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Form Score</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-green-400">86%</div>
                  <p className="text-xs text-white/60">Overall form quality</p>
                </CardContent>
              </Card>
            </div>

            <div className="space-y-4">
              <h3 className="text-lg">Real-time Feedback</h3>
              
              <div className="space-y-3">
                <div className="flex items-start gap-3 p-3 bg-green-600/10 border border-green-500/30 rounded-lg">
                  <div className="w-2 h-2 bg-green-400 rounded-full mt-2"></div>
                  <div>
                    <p className="text-green-300 text-sm font-medium">Good depth achieved</p>
                    <p className="text-white/60 text-xs">Your squat depth is within the optimal range</p>
                  </div>
                </div>
                
                <div className="flex items-start gap-3 p-3 bg-yellow-600/10 border border-yellow-500/30 rounded-lg">
                  <div className="w-2 h-2 bg-yellow-400 rounded-full mt-2"></div>
                  <div>
                    <p className="text-yellow-300 text-sm font-medium">Knee tracking needs attention</p>
                    <p className="text-white/60 text-xs">Keep knees aligned with toes during descent</p>
                  </div>
                </div>
                
                <div className="flex items-start gap-3 p-3 bg-blue-600/10 border border-blue-500/30 rounded-lg">
                  <div className="w-2 h-2 bg-blue-400 rounded-full mt-2"></div>
                  <div>
                    <p className="text-blue-300 text-sm font-medium">Tempo recommendation</p>
                    <p className="text-white/60 text-xs">Try 2 seconds down, 1 second up for better control</p>
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
              <div className="absolute inset-0 bg-gradient-to-br from-purple-500/20 to-blue-500/20"></div>
              <div className="relative z-10 text-center">
                <Camera className="h-12 w-12 text-white/60 mx-auto mb-2" />
                <p className="text-white/80">Camera Feed Active</p>
                <p className="text-white/60 text-sm">AI Form Analysis Running</p>
              </div>
              <div className="absolute top-4 right-4 flex gap-2">
                <div className="w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
                <span className="text-xs text-white/80">LIVE</span>
              </div>
            </div>
            
            <div className="flex gap-2">
              <Button className="flex-1 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700">
                <Play className="h-4 w-4 mr-2" />
                Start Recording
              </Button>
              <Button variant="outline" className="border-white/20 text-white hover:bg-white/10">
                <Square className="h-4 w-4" />
              </Button>
              <Button variant="outline" className="border-white/20 text-white hover:bg-white/10">
                <RotateCcw className="h-4 w-4" />
              </Button>
            </div>
          </div>
          
          {/* Form Analysis Metrics */}
          <div className="space-y-4">
            <h3 className="text-lg">Form Analysis</h3>
            
            <div className="space-y-4">
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-white/80">Squat Depth</span>
                  <span className="text-green-400">92%</span>
                </div>
                <Progress value={92} className="h-2" />
              </div>
              
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-white/80">Knee Alignment</span>
                  <span className="text-yellow-400">78%</span>
                </div>
                <Progress value={78} className="h-2" />
              </div>
              
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-white/80">Back Posture</span>
                  <span className="text-green-400">88%</span>
                </div>
                <Progress value={88} className="h-2" />
              </div>
              
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-white/80">Rep Tempo</span>
                  <span className="text-blue-400">85%</span>
                </div>
                <Progress value={85} className="h-2" />
              </div>
            </div>
            
            <div className="mt-6 p-4 bg-gradient-to-r from-green-500/20 to-blue-500/20 rounded-lg border border-white/10">
              <h4 className="text-green-400 mb-2">Current Exercise: Squats</h4>
              <p className="text-white/80 text-sm">Great form! Focus on knee tracking for optimal performance.</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}