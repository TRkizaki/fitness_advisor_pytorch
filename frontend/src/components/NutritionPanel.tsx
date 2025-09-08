import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";
import { Progress } from "./ui/progress";
import { Apple, Plus, Search, TrendingUp, Clock, AlertCircle } from "lucide-react";
import { FitnessApiClient, User, userToMCPProfile, NutritionAnalysisRequest, NutritionPlanRequest } from "../api/client";

interface FoodItem {
  name: string;
  quantity: number;
  unit: string;
  meal_timing?: string;
}

export function NutritionPanel() {
  const [users, setUsers] = useState<User[]>([]);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [nutritionPlan, setNutritionPlan] = useState<any>(null);
  const [nutritionAnalysis, setNutritionAnalysis] = useState<any>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [error, setError] = useState<string>('');
  const [foodItems, setFoodItems] = useState<FoodItem[]>([
    { name: "Chicken breast", quantity: 150, unit: "g", meal_timing: "lunch" },
    { name: "Brown rice", quantity: 80, unit: "g", meal_timing: "lunch" },
    { name: "Broccoli", quantity: 100, unit: "g", meal_timing: "lunch" }
  ]);

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

  const generateNutritionPlan = async (planType: 'balanced' | 'high_protein' | 'low_carb' | 'mediterranean' = 'balanced') => {
    if (!selectedUser) {
      setError('No user selected');
      return;
    }

    setIsGenerating(true);
    setError('');

    try {
      const mcpProfile = userToMCPProfile(selectedUser);
      
      const nutritionRequest: NutritionPlanRequest = {
        user_profile: mcpProfile,
        calorie_target: calculateCalorieTarget(selectedUser),
        meal_preferences: {
          meals_per_day: 3,
          prep_time_minutes: 30,
          cuisine_preferences: getPlanCuisinePreferences(planType),
          avoid_ingredients: [],
          macro_split: getPlanMacroSplit(planType)
        }
      };

      const result = await FitnessApiClient.createNutritionPlan(nutritionRequest);
      setNutritionPlan(result);
      
    } catch (err) {
      setError(`Failed to generate nutrition plan: ${err}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const analyzeFoods = async (analysisType: 'basic' | 'micronutrients' | 'interactions' | 'timing' = 'basic') => {
    if (foodItems.length === 0) {
      setError('No foods to analyze');
      return;
    }

    setIsAnalyzing(true);
    setError('');

    try {
      const analysisRequest: NutritionAnalysisRequest = {
        foods: foodItems,
        analysis_type: analysisType
      };

      const result = await FitnessApiClient.analyzeNutrition(analysisRequest);
      setNutritionAnalysis(result);
      
    } catch (err) {
      setError(`Failed to analyze nutrition: ${err}`);
    } finally {
      setIsAnalyzing(false);
    }
  };

  const calculateCalorieTarget = (user: User): number => {
    // Basic BMR calculation (Harris-Benedict)
    const bmr = 88.362 + (13.397 * user.weight) + (4.799 * user.height) - (5.677 * user.age);
    const activityMultiplier = user.fitness_level.toLowerCase() === 'beginner' ? 1.375 :
                              user.fitness_level.toLowerCase() === 'intermediate' ? 1.55 :
                              user.fitness_level.toLowerCase() === 'advanced' ? 1.725 : 1.9;
    return Math.round(bmr * activityMultiplier);
  };

  const getPlanCuisinePreferences = (planType: string): string[] => {
    switch (planType) {
      case 'mediterranean': return ['Mediterranean', 'Italian', 'Greek'];
      case 'high_protein': return ['American', 'Grilled'];
      case 'low_carb': return ['Keto', 'Paleo'];
      default: return ['American', 'Asian', 'Mediterranean'];
    }
  };

  const getPlanMacroSplit = (planType: string) => {
    switch (planType) {
      case 'high_protein': return { protein_percent: 35, carbohydrate_percent: 35, fat_percent: 30 };
      case 'low_carb': return { protein_percent: 25, carbohydrate_percent: 20, fat_percent: 55 };
      case 'mediterranean': return { protein_percent: 20, carbohydrate_percent: 45, fat_percent: 35 };
      default: return { protein_percent: 25, carbohydrate_percent: 45, fat_percent: 30 };
    }
  };

  const addFoodItem = () => {
    setFoodItems([...foodItems, { name: "", quantity: 0, unit: "g", meal_timing: "lunch" }]);
  };

  const updateFoodItem = (index: number, field: keyof FoodItem, value: string | number) => {
    const updated = [...foodItems];
    updated[index] = { ...updated[index], [field]: value };
    setFoodItems(updated);
  };

  const removeFoodItem = (index: number) => {
    setFoodItems(foodItems.filter((_, i) => i !== index));
  };

  const renderNutritionPlan = () => {
    if (!nutritionPlan) return null;

    const planText = nutritionPlan.content?.[0]?.text || JSON.stringify(nutritionPlan, null, 2);
    
    return (
      <div className="bg-gradient-to-r from-green-600/10 to-blue-600/10 border border-green-500/30 rounded-lg p-4">
        <div className="flex items-center justify-between mb-3">
          <h4 className="text-md font-medium">Generated Nutrition Plan</h4>
          <Badge variant="outline" className="text-green-300 border-green-400/50">
            MCP Generated
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

  const renderNutritionAnalysis = () => {
    if (!nutritionAnalysis) return null;

    const analysisText = nutritionAnalysis.content?.[0]?.text || JSON.stringify(nutritionAnalysis, null, 2);
    
    return (
      <div className="bg-gradient-to-r from-orange-600/10 to-red-600/10 border border-orange-500/30 rounded-lg p-4">
        <div className="flex items-center justify-between mb-3">
          <h4 className="text-md font-medium">Nutrition Analysis</h4>
          <Badge variant="outline" className="text-orange-300 border-orange-400/50">
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
            <Apple className="h-5 w-5" />
            Nutrition Center
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
        <Tabs defaultValue="planner" className="w-full">
          <TabsList className="grid w-full grid-cols-3 bg-white/10">
            <TabsTrigger value="planner" className="data-[state=active]:bg-white/20">
              <Plus className="h-4 w-4 mr-2" />
              Plan Generator
            </TabsTrigger>
            <TabsTrigger value="analyzer" className="data-[state=active]:bg-white/20">
              <Search className="h-4 w-4 mr-2" />
              Food Analysis
            </TabsTrigger>
            <TabsTrigger value="tracking" className="data-[state=active]:bg-white/20">
              <TrendingUp className="h-4 w-4 mr-2" />
              Daily Tracking
            </TabsTrigger>
          </TabsList>

          {/* Plan Generator Tab */}
          <TabsContent value="planner" className="space-y-4">
            {error && (
              <div className="bg-red-600/20 border border-red-500/30 rounded-lg p-3">
                <p className="text-red-300">{error}</p>
              </div>
            )}

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div className="space-y-4">
                <h3 className="text-lg">Nutrition Plan Generator</h3>
                <p className="text-white/70 text-sm">
                  Generate personalized nutrition plans using AI-powered MCP tools based on your profile and dietary preferences.
                </p>
                
                {selectedUser && (
                  <div className="bg-white/5 rounded-lg p-3">
                    <h4 className="text-sm font-medium mb-2">User Profile Summary</h4>
                    <div className="grid grid-cols-2 gap-2 text-xs text-white/70">
                      <span>Age: {selectedUser.age}yo</span>
                      <span>Level: {selectedUser.fitness_level}</span>
                      <span>Weight: {selectedUser.weight}kg</span>
                      <span>Height: {selectedUser.height}cm</span>
                      <span className="col-span-2">Target Calories: ~{calculateCalorieTarget(selectedUser)}</span>
                    </div>
                  </div>
                )}
                
                <div className="flex flex-wrap gap-2">
                  <Button 
                    onClick={() => generateNutritionPlan('balanced')}
                    disabled={isGenerating || !selectedUser}
                    className="bg-gradient-to-r from-green-600 to-blue-600 hover:from-green-700 hover:to-blue-700"
                  >
                    {isGenerating ? <Clock className="h-4 w-4 mr-2 animate-spin" /> : <Plus className="h-4 w-4 mr-2" />}
                    {isGenerating ? 'Generating...' : 'Balanced Plan'}
                  </Button>
                </div>
                
                <div className="grid grid-cols-2 gap-2">
                  <Button 
                    variant="outline"
                    onClick={() => generateNutritionPlan('high_protein')}
                    disabled={isGenerating || !selectedUser}
                    className="border-red-500/50 text-red-300 hover:bg-red-600/20"
                  >
                    High Protein
                  </Button>
                  <Button 
                    variant="outline"
                    onClick={() => generateNutritionPlan('low_carb')}
                    disabled={isGenerating || !selectedUser}
                    className="border-yellow-500/50 text-yellow-300 hover:bg-yellow-600/20"
                  >
                    Low Carb
                  </Button>
                  <Button 
                    variant="outline"
                    onClick={() => generateNutritionPlan('mediterranean')}
                    disabled={isGenerating || !selectedUser}
                    className="border-purple-500/50 text-purple-300 hover:bg-purple-600/20"
                  >
                    Mediterranean
                  </Button>
                  <Button 
                    variant="outline"
                    onClick={() => generateNutritionPlan('balanced')}
                    disabled={isGenerating || !selectedUser}
                    className="border-green-500/50 text-green-300 hover:bg-green-600/20"
                  >
                    Balanced
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
                {renderNutritionPlan()}
                
                {!nutritionPlan && !isGenerating && (
                  <div className="bg-gradient-to-br from-gray-800/50 to-gray-900/50 rounded-lg border border-white/10 p-6 text-center">
                    <Apple className="h-12 w-12 text-white/40 mx-auto mb-3" />
                    <p className="text-white/60">Generate a nutrition plan to get started</p>
                    <p className="text-white/40 text-sm mt-2">
                      AI-powered meal plans personalized to your dietary needs and goals
                    </p>
                  </div>
                )}
              </div>
            </div>
          </TabsContent>

          {/* Food Analysis Tab */}
          <TabsContent value="analyzer" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg">Food Analysis</h3>
                  <Button
                    onClick={addFoodItem}
                    size="sm"
                    variant="outline"
                    className="border-white/20 text-white hover:bg-white/10"
                  >
                    <Plus className="h-4 w-4 mr-1" />
                    Add Food
                  </Button>
                </div>

                <div className="space-y-3">
                  {foodItems.map((item, index) => (
                    <div key={index} className="bg-white/5 rounded-lg p-3 space-y-2">
                      <div className="flex gap-2">
                        <input
                          type="text"
                          placeholder="Food name"
                          value={item.name}
                          onChange={(e) => updateFoodItem(index, 'name', e.target.value)}
                          className="flex-1 bg-black/40 border border-white/20 rounded px-2 py-1 text-white text-sm"
                        />
                        <button
                          onClick={() => removeFoodItem(index)}
                          className="text-red-400 hover:text-red-300 px-2"
                        >
                          ×
                        </button>
                      </div>
                      <div className="flex gap-2">
                        <input
                          type="number"
                          placeholder="Qty"
                          value={item.quantity}
                          onChange={(e) => updateFoodItem(index, 'quantity', parseFloat(e.target.value) || 0)}
                          className="w-20 bg-black/40 border border-white/20 rounded px-2 py-1 text-white text-sm"
                        />
                        <select
                          value={item.unit}
                          onChange={(e) => updateFoodItem(index, 'unit', e.target.value)}
                          className="w-20 bg-black/40 border border-white/20 rounded px-2 py-1 text-white text-sm"
                        >
                          <option value="g">g</option>
                          <option value="oz">oz</option>
                          <option value="cup">cup</option>
                          <option value="tbsp">tbsp</option>
                        </select>
                        <select
                          value={item.meal_timing || 'lunch'}
                          onChange={(e) => updateFoodItem(index, 'meal_timing', e.target.value)}
                          className="flex-1 bg-black/40 border border-white/20 rounded px-2 py-1 text-white text-sm"
                        >
                          <option value="breakfast">Breakfast</option>
                          <option value="lunch">Lunch</option>
                          <option value="dinner">Dinner</option>
                          <option value="snack">Snack</option>
                        </select>
                      </div>
                    </div>
                  ))}
                </div>

                <div className="flex flex-wrap gap-2">
                  <Button
                    onClick={() => analyzeFoods('basic')}
                    disabled={isAnalyzing || foodItems.length === 0}
                    className="bg-orange-600 hover:bg-orange-700"
                  >
                    {isAnalyzing ? <Clock className="h-4 w-4 mr-2 animate-spin" /> : <Search className="h-4 w-4 mr-2" />}
                    {isAnalyzing ? 'Analyzing...' : 'Basic Analysis'}
                  </Button>
                  
                  <Button
                    variant="outline"
                    onClick={() => analyzeFoods('micronutrients')}
                    disabled={isAnalyzing || foodItems.length === 0}
                    className="border-purple-500/50 text-purple-300 hover:bg-purple-600/20"
                  >
                    Micronutrients
                  </Button>
                  
                  <Button
                    variant="outline"
                    onClick={() => analyzeFoods('interactions')}
                    disabled={isAnalyzing || foodItems.length === 0}
                    className="border-blue-500/50 text-blue-300 hover:bg-blue-600/20"
                  >
                    Interactions
                  </Button>
                </div>
              </div>

              <div className="space-y-4">
                {renderNutritionAnalysis()}
                
                {!nutritionAnalysis && !isAnalyzing && (
                  <div className="bg-gradient-to-br from-gray-800/50 to-gray-900/50 rounded-lg border border-white/10 p-6 text-center">
                    <Search className="h-12 w-12 text-white/40 mx-auto mb-3" />
                    <p className="text-white/60">Add foods and analyze their nutrition</p>
                    <p className="text-white/40 text-sm mt-2">
                      Get detailed nutritional breakdowns and recommendations
                    </p>
                  </div>
                )}
              </div>
            </div>
          </TabsContent>

          {/* Daily Tracking Tab */}
          <TabsContent value="tracking" className="space-y-4">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Daily Calories</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span>Consumed</span>
                      <span>1,847 / 2,200</span>
                    </div>
                    <Progress value={84} className="h-2" />
                    <p className="text-xs text-white/60">353 calories remaining</p>
                  </div>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Protein</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span>Target</span>
                      <span>128g / 150g</span>
                    </div>
                    <Progress value={85} className="h-2" />
                    <p className="text-xs text-green-400">On track</p>
                  </div>
                </CardContent>
              </Card>
              
              <Card className="bg-white/5 border-white/10">
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm text-white/80">Water Intake</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span>Today</span>
                      <span>6 / 8 glasses</span>
                    </div>
                    <Progress value={75} className="h-2" />
                    <p className="text-xs text-blue-400">2 more to go</p>
                  </div>
                </CardContent>
              </Card>
            </div>

            <div className="space-y-4">
              <h3 className="text-lg">Meal Timeline</h3>
              
              <div className="space-y-3">
                <div className="flex items-center gap-4 p-3 bg-green-600/10 border border-green-500/30 rounded-lg">
                  <div className="w-12 h-12 bg-green-600/20 rounded-full flex items-center justify-center">
                    <Clock className="h-6 w-6 text-green-400" />
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-green-300">Breakfast - 7:30 AM</p>
                        <p className="text-xs text-white/60">Oatmeal with berries, Greek yogurt</p>
                      </div>
                      <Badge variant="outline" className="text-green-300 border-green-400/50">
                        485 cal
                      </Badge>
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-4 p-3 bg-blue-600/10 border border-blue-500/30 rounded-lg">
                  <div className="w-12 h-12 bg-blue-600/20 rounded-full flex items-center justify-center">
                    <Clock className="h-6 w-6 text-blue-400" />
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-blue-300">Lunch - 12:45 PM</p>
                        <p className="text-xs text-white/60">Grilled chicken salad, quinoa</p>
                      </div>
                      <Badge variant="outline" className="text-blue-300 border-blue-400/50">
                        642 cal
                      </Badge>
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-4 p-3 bg-gray-600/10 border border-gray-500/30 rounded-lg">
                  <div className="w-12 h-12 bg-gray-600/20 rounded-full flex items-center justify-center">
                    <AlertCircle className="h-6 w-6 text-gray-400" />
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-gray-300">Dinner - Planned</p>
                        <p className="text-xs text-white/60">Salmon, roasted vegetables</p>
                      </div>
                      <Badge variant="outline" className="text-gray-300 border-gray-400/50">
                        ~520 cal
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