import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Progress } from "./ui/progress";
import { Camera, Play, Square, RotateCcw } from "lucide-react";

export function WorkoutPanel() {
  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10 text-white">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Camera className="h-5 w-5" />
          Real-Time Workout Tracking
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
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
      </CardContent>
    </Card>
  );
}