import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { LineChart, Line, AreaChart, Area, XAxis, YAxis, CartesianGrid, ResponsiveContainer, Tooltip } from "recharts";
import { TrendingUp, Weight, Zap } from "lucide-react";

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
  return (
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
  );
}