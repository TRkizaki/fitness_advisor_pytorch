import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";
import { Sun, Cloud, Snowflake, Leaf, MapPin, Settings, Thermometer, Calendar } from "lucide-react";
import { FitnessApiClient, User, userToMCPProfile, SeasonalOptimizationRequest } from "../api/client";

interface WeatherData {
  temperature: number;
  condition: string;
  season: 'spring' | 'summer' | 'fall' | 'winter';
  location: string;
}

export function SeasonalOptimization() {
  const [users, setUsers] = useState<User[]>([]);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [seasonalPlan, setSeasonalPlan] = useState<any>(null);
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [error, setError] = useState<string>('');
  const [location, setLocation] = useState('New York, NY');
  const [currentSeason, setCurrentSeason] = useState<'spring' | 'summer' | 'fall' | 'winter'>('winter');
  const [indoorPreference, setIndoorPreference] = useState(false);
  const [weatherData, setWeatherData] = useState<WeatherData>({
    temperature: 35,
    condition: 'Clear',
    season: 'winter',
    location: 'New York, NY'
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

    // Simulate getting current season based on date
    const month = new Date().getMonth();
    if (month >= 2 && month <= 4) setCurrentSeason('spring');
    else if (month >= 5 && month <= 7) setCurrentSeason('summer');
    else if (month >= 8 && month <= 10) setCurrentSeason('fall');
    else setCurrentSeason('winter');
  }, []);

  const optimizeForSeason = async (season: 'spring' | 'summer' | 'fall' | 'winter') => {
    if (!selectedUser) {
      setError('No user selected');
      return;
    }

    setIsOptimizing(true);
    setError('');

    try {
      const mcpProfile = userToMCPProfile(selectedUser);
      
      const optimizationRequest: SeasonalOptimizationRequest = {
        location: location,
        season: season,
        indoor_preference: indoorPreference,
        user_profile: mcpProfile
      };

      const result = await FitnessApiClient.optimizeForSeason(optimizationRequest);
      setSeasonalPlan(result);
      
    } catch (err) {
      setError(`Failed to optimize for season: ${err}`);
    } finally {
      setIsOptimizing(false);
    }
  };

  const getSeasonIcon = (season: string) => {
    switch (season) {
      case 'spring': return <Leaf className="h-5 w-5 text-green-400" />;
      case 'summer': return <Sun className="h-5 w-5 text-yellow-400" />;
      case 'fall': return <Leaf className="h-5 w-5 text-orange-400" />;
      case 'winter': return <Snowflake className="h-5 w-5 text-blue-400" />;
      default: return <Cloud className="h-5 w-5 text-gray-400" />;
    }
  };

  const getSeasonColor = (season: string) => {
    switch (season) {
      case 'spring': return 'from-green-600/10 to-emerald-600/10 border-green-500/30';
      case 'summer': return 'from-yellow-600/10 to-orange-600/10 border-yellow-500/30';
      case 'fall': return 'from-orange-600/10 to-red-600/10 border-orange-500/30';
      case 'winter': return 'from-blue-600/10 to-cyan-600/10 border-blue-500/30';
      default: return 'from-gray-600/10 to-gray-700/10 border-gray-500/30';
    }
  };

  const renderSeasonalPlan = () => {
    if (!seasonalPlan) return null;

    const planText = seasonalPlan.content?.[0]?.text || JSON.stringify(seasonalPlan, null, 2);
    
    return (
      <div className={`bg-gradient-to-r ${getSeasonColor(currentSeason)} border rounded-lg p-4`}>
        <div className="flex items-center justify-between mb-3">
          <h4 className="text-md font-medium">Seasonal Optimization Plan</h4>
          <Badge variant="outline" className="text-blue-300 border-blue-400/50">
            MCP Optimized
          </Badge>
        </div>
        <div className="bg-black/20 rounded p-3 max-h-64 overflow-auto">
          <pre className="text-xs whitespace-pre-wrap text-white/90">
            {planText}
          </pre>
        </div>
      </div>
    );
  };

  const getCurrentSeasonRecommendations = () => {
    switch (currentSeason) {
      case 'spring':
        return [
          'Outdoor running and cycling',
          'Spring hiking activities',
          'Allergy-friendly indoor alternatives',
          'Gradual activity increase'
        ];
      case 'summer':
        return [
          'Early morning workouts',
          'Swimming and water sports',
          'Heat management strategies',
          'Increased hydration focus'
        ];
      case 'fall':
        return [
          'Resistance training preparation',
          'Harvest season nutrition',
          'Indoor activity planning',
          'Immune system support'
        ];
      case 'winter':
        return [
          'Indoor strength training',
          'Vitamin D supplementation',
          'Winter sport activities',
          'Mood and energy management'
        ];
      default:
        return ['General fitness recommendations'];
    }
  };

  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            {getSeasonIcon(currentSeason)}
            Seasonal Optimization
          </div>
          {selectedUser && (
            <div className="flex items-center gap-2">
              <span className="text-sm text-white/70">Active User:</span>
              <Badge variant="outline" className="text-green-300 border-green-400/50">
                {selectedUser.name}
              </Badge>
            </div>
          )}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <Tabs defaultValue="current" className="w-full">
          <TabsList className="grid w-full grid-cols-3 bg-white/10">
            <TabsTrigger value="current" className="data-[state=active]:bg-white/20">
              <Calendar className="h-4 w-4 mr-2" />
              Current Season
            </TabsTrigger>
            <TabsTrigger value="optimizer" className="data-[state=active]:bg-white/20">
              <Settings className="h-4 w-4 mr-2" />
              Optimizer
            </TabsTrigger>
            <TabsTrigger value="weather" className="data-[state=active]:bg-white/20">
              <Thermometer className="h-4 w-4 mr-2" />
              Weather Impact
            </TabsTrigger>
          </TabsList>

          {/* Current Season Tab */}
          <TabsContent value="current" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div className="space-y-4">
                <div className={`bg-gradient-to-r ${getSeasonColor(currentSeason)} border rounded-lg p-4`}>
                  <div className="flex items-center gap-3 mb-3">
                    {getSeasonIcon(currentSeason)}
                    <div>
                      <h3 className="text-lg font-medium capitalize">{currentSeason} Season</h3>
                      <p className="text-white/60 text-sm">Current fitness recommendations</p>
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    {getCurrentSeasonRecommendations().map((rec, index) => (
                      <div key={index} className="flex items-center gap-2 text-sm">
                        <div className="w-2 h-2 bg-white/40 rounded-full"></div>
                        <span className="text-white/80">{rec}</span>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="bg-white/5 rounded-lg p-4">
                  <div className="flex items-center gap-2 mb-3">
                    <MapPin className="h-4 w-4 text-blue-400" />
                    <h4 className="text-sm font-medium">Location & Weather</h4>
                  </div>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-white/70">Location:</span>
                      <span>{weatherData.location}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-white/70">Temperature:</span>
                      <span>{weatherData.temperature}°F</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-white/70">Condition:</span>
                      <span>{weatherData.condition}</span>
                    </div>
                  </div>
                </div>
              </div>

              <div className="space-y-4">
                <h3 className="text-lg">Seasonal Activity Suggestions</h3>
                
                <div className="space-y-3">
                  <div className="bg-green-600/10 border border-green-500/30 rounded-lg p-3">
                    <div className="flex items-center gap-2 mb-2">
                      <Sun className="h-4 w-4 text-green-400" />
                      <span className="text-green-300 text-sm font-medium">Outdoor Activities</span>
                    </div>
                    <p className="text-white/70 text-xs">
                      {currentSeason === 'winter' 
                        ? 'Limited outdoor options due to cold. Focus on winter sports.'
                        : 'Great weather for outdoor activities. Take advantage of natural conditions.'}
                    </p>
                  </div>

                  <div className="bg-blue-600/10 border border-blue-500/30 rounded-lg p-3">
                    <div className="flex items-center gap-2 mb-2">
                      <Settings className="h-4 w-4 text-blue-400" />
                      <span className="text-blue-300 text-sm font-medium">Equipment Recommendations</span>
                    </div>
                    <p className="text-white/70 text-xs">
                      Seasonal equipment adjustments for optimal performance and comfort.
                    </p>
                  </div>

                  <div className="bg-purple-600/10 border border-purple-500/30 rounded-lg p-3">
                    <div className="flex items-center gap-2 mb-2">
                      <Thermometer className="h-4 w-4 text-purple-400" />
                      <span className="text-purple-300 text-sm font-medium">Nutrition Focus</span>
                    </div>
                    <p className="text-white/70 text-xs">
                      Seasonal nutrition to support immune system and energy levels.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </TabsContent>

          {/* Optimizer Tab */}
          <TabsContent value="optimizer" className="space-y-4">
            {error && (
              <div className="bg-red-600/20 border border-red-500/30 rounded-lg p-3">
                <p className="text-red-300">{error}</p>
              </div>
            )}

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div className="space-y-4">
                <h3 className="text-lg">Seasonal Optimization Settings</h3>
                
                <div className="space-y-3">
                  <div>
                    <label className="text-sm text-white/70">Location</label>
                    <input
                      type="text"
                      value={location}
                      onChange={(e) => setLocation(e.target.value)}
                      className="w-full bg-black/40 border border-white/20 rounded px-3 py-2 text-white text-sm mt-1"
                      placeholder="City, State or Country"
                    />
                  </div>

                  <div>
                    <label className="text-sm text-white/70">Season to Optimize For</label>
                    <div className="grid grid-cols-2 gap-2 mt-2">
                      {(['spring', 'summer', 'fall', 'winter'] as const).map((season) => (
                        <Button
                          key={season}
                          variant={currentSeason === season ? "default" : "outline"}
                          onClick={() => setCurrentSeason(season)}
                          className="flex items-center gap-2"
                        >
                          {getSeasonIcon(season)}
                          <span className="capitalize">{season}</span>
                        </Button>
                      ))}
                    </div>
                  </div>

                  <div className="flex items-center gap-3">
                    <input
                      type="checkbox"
                      id="indoorPreference"
                      checked={indoorPreference}
                      onChange={(e) => setIndoorPreference(e.target.checked)}
                      className="w-4 h-4 bg-black/40 border border-white/20 rounded"
                    />
                    <label htmlFor="indoorPreference" className="text-sm text-white/70">
                      Prefer indoor activities
                    </label>
                  </div>

                  <Button
                    onClick={() => optimizeForSeason(currentSeason)}
                    disabled={isOptimizing || !selectedUser}
                    className="w-full bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700"
                  >
                    {isOptimizing ? 'Optimizing...' : 'Generate Seasonal Plan'}
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
                {renderSeasonalPlan()}
                
                {!seasonalPlan && !isOptimizing && (
                  <div className="bg-gradient-to-br from-gray-800/50 to-gray-900/50 rounded-lg border border-white/10 p-6 text-center">
                    {getSeasonIcon(currentSeason)}
                    <p className="text-white/60 mt-3">Generate a seasonal optimization plan</p>
                    <p className="text-white/40 text-sm mt-2">
                      AI-powered recommendations based on weather and seasonal factors
                    </p>
                  </div>
                )}
              </div>
            </div>
          </TabsContent>

          {/* Weather Impact Tab */}
          <TabsContent value="weather" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Temperature Impact</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-white">{weatherData.temperature}°F</div>
                  <p className="text-xs text-white/60">
                    {weatherData.temperature < 40 ? 'Cold - Indoor focus recommended' :
                     weatherData.temperature > 80 ? 'Hot - Early morning workouts' : 
                     'Ideal - Great for outdoor activities'}
                  </p>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Activity Adjustment</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-yellow-400">85%</div>
                  <p className="text-xs text-white/60">Outdoor activity suitability</p>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Hydration Need</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold text-blue-400">Normal</div>
                  <p className="text-xs text-white/60">Based on temperature and humidity</p>
                </CardContent>
              </Card>
            </div>

            <div className="space-y-4">
              <h3 className="text-lg">Weather-Based Recommendations</h3>
              
              <div className="space-y-3">
                <div className="bg-blue-600/10 border border-blue-500/30 rounded-lg p-3">
                  <div className="flex items-center gap-2 mb-2">
                    <Thermometer className="h-4 w-4 text-blue-400" />
                    <span className="text-blue-300 text-sm font-medium">Cold Weather Adaptations</span>
                  </div>
                  <ul className="text-xs text-white/70 space-y-1">
                    <li>• Extended warm-up periods</li>
                    <li>• Layer clothing for temperature regulation</li>
                    <li>• Focus on indoor strength training</li>
                    <li>• Maintain consistent routine</li>
                  </ul>
                </div>

                <div className="bg-orange-600/10 border border-orange-500/30 rounded-lg p-3">
                  <div className="flex items-center gap-2 mb-2">
                    <Sun className="h-4 w-4 text-orange-400" />
                    <span className="text-orange-300 text-sm font-medium">Seasonal Energy Management</span>
                  </div>
                  <ul className="text-xs text-white/70 space-y-1">
                    <li>• Vitamin D supplementation in winter</li>
                    <li>• Light therapy for mood regulation</li>
                    <li>• Adjusted sleep schedule</li>
                    <li>• Seasonal affective disorder prevention</li>
                  </ul>
                </div>

                <div className="bg-green-600/10 border border-green-500/30 rounded-lg p-3">
                  <div className="flex items-center gap-2 mb-2">
                    <Leaf className="h-4 w-4 text-green-400" />
                    <span className="text-green-300 text-sm font-medium">Nutrition Adjustments</span>
                  </div>
                  <ul className="text-xs text-white/70 space-y-1">
                    <li>• Increased caloric intake in cold weather</li>
                    <li>• Seasonal produce incorporation</li>
                    <li>• Immune system support foods</li>
                    <li>• Hydration adjustment for temperature</li>
                  </ul>
                </div>
              </div>
            </div>
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
}