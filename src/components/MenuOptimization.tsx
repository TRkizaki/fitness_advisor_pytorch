import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Slider } from "./ui/slider";
import { Badge } from "./ui/badge";
import { Brain, Utensils, Zap, Timer } from "lucide-react";
import { useState } from "react";
import { FitnessApiClient } from "../api/client";

export function MenuOptimization() {
  const [calorieTarget, setCalorieTarget] = useState(1960);
  const [proteinTarget, setProteinTarget] = useState(133);
  const [carbLimit, setCarbLimit] = useState(180);
  const [fatTarget, setFatTarget] = useState(70);
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [optimizationResult, setOptimizationResult] = useState(null);

  const handleOptimization = async () => {
    setIsOptimizing(true);
    try {
      const request = {
        user_id: "demo_user",
        time_horizon_days: 7,
        objectives: ["MaximizeNutrition", "BalanceMacros", "MinimizeCost"],
        constraints: {
          daily_calories: {
            min: calorieTarget * 0.9,
            max: calorieTarget * 1.1,
            target: calorieTarget,
          },
          macros: {
            protein_g: { min: proteinTarget * 0.8, max: proteinTarget * 1.2 },
            carbs_g: { min: carbLimit * 0.7, max: carbLimit * 1.3 },
            fat_g: { min: fatTarget * 0.8, max: fatTarget * 1.2 },
          },
        },
      };
      
      const result = await FitnessApiClient.optimizeMealPlan(request);
      setOptimizationResult(result);
    } catch (error) {
      console.error('Optimization failed:', error);
    } finally {
      setIsOptimizing(false);
    }
  };

  const mealPlan = [
    {
      meal: "Breakfast",
      items: ["Greek Yogurt with Berries", "Oatmeal with Almonds", "Green Tea"],
      calories: 420,
      protein: 28,
    },
    {
      meal: "Lunch",
      items: ["Grilled Chicken Salad", "Quinoa Bowl", "Avocado"],
      calories: 580,
      protein: 42,
    },
    {
      meal: "Dinner",
      items: ["Salmon Fillet", "Sweet Potato", "Steamed Broccoli"],
      calories: 650,
      protein: 38,
    },
    {
      meal: "Snacks",
      items: ["Protein Shake", "Mixed Nuts", "Apple"],
      calories: 310,
      protein: 25,
    },
  ];

  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Brain className="h-5 w-5" />
          AI Menu Optimization
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Genetic Algorithm Controls */}
          <div className="space-y-6">
            <h3 className="text-lg flex items-center gap-2">
              <Zap className="h-5 w-5" />
              Algorithm Controls
            </h3>
            
            <div className="space-y-4">
              <div className="space-y-2">
                <label className="text-white/80">Calorie Target: 1,960 kcal</label>
                <Slider defaultValue={[1960]} max={3000} min={1200} step={50} className="w-full" />
              </div>
              
              <div className="space-y-2">
                <label className="text-white/80">Protein Target: 133g</label>
                <Slider defaultValue={[133]} max={200} min={80} step={5} className="w-full" />
              </div>
              
              <div className="space-y-2">
                <label className="text-white/80">Carb Limit: 180g</label>
                <Slider defaultValue={[180]} max={300} min={100} step={10} className="w-full" />
              </div>
              
              <div className="space-y-2">
                <label className="text-white/80">Fat Target: 70g</label>
                <Slider defaultValue={[70]} max={120} min={40} step={5} className="w-full" />
              </div>
            </div>
            
            <div className="space-y-3">
              <Button 
                className="w-full bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700"
                onClick={handleOptimization}
                disabled={isOptimizing}
              >
                <Brain className="h-4 w-4 mr-2" />
                {isOptimizing ? 'Optimizing...' : 'Optimize Meal Plan'}
              </Button>
              <div className="flex gap-2">
                <Badge variant="secondary" className="bg-green-500/20 text-green-400 border-green-500/30">
                  Generation 23
                </Badge>
                <Badge variant="secondary" className="bg-blue-500/20 text-blue-400 border-blue-500/30">
                  Fitness: 94.2%
                </Badge>
              </div>
            </div>
          </div>
          
          {/* Generated Meal Plan */}
          <div className="space-y-4">
            <h3 className="text-lg flex items-center gap-2">
              <Utensils className="h-5 w-5" />
              Generated Meal Plan
            </h3>
            
            <div className="space-y-3">
              {mealPlan.map((meal, index) => (
                <Card key={index} className="bg-gradient-to-r from-white/5 to-white/10 border-white/20">
                  <CardContent className="p-4">
                    <div className="flex justify-between items-start mb-2">
                      <h4 className="text-white">{meal.meal}</h4>
                      <div className="flex gap-2">
                        <Badge variant="outline" className="border-purple-500/50 text-purple-300">
                          {meal.calories} kcal
                        </Badge>
                        <Badge variant="outline" className="border-blue-500/50 text-blue-300">
                          {meal.protein}g protein
                        </Badge>
                      </div>
                    </div>
                    <div className="space-y-1">
                      {meal.items.map((item, itemIndex) => (
                        <p key={itemIndex} className="text-white/70 text-sm">• {item}</p>
                      ))}
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
            
            <div className="p-4 bg-gradient-to-r from-purple-500/20 to-blue-500/20 rounded-lg border border-white/10">
              <div className="flex items-center gap-2 mb-2">
                <Timer className="h-4 w-4 text-green-400" />
                <span className="text-green-400">Optimization Complete</span>
              </div>
              <p className="text-white/80 text-sm">
                Total: 1,960 kcal, 133g protein, 45g fiber. Matches your goals perfectly!
              </p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}