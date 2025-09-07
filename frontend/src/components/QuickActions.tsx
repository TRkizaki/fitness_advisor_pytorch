import { Card, CardContent } from "./ui/card";
import { Button } from "./ui/button";
import { Play, Utensils, BarChart3, Calendar, Camera, Target } from "lucide-react";

interface ActionButtonProps {
  icon: React.ReactNode;
  title: string;
  subtitle: string;
  gradient: string;
  onClick?: () => void;
}

function ActionButton({ icon, title, subtitle, gradient, onClick }: ActionButtonProps) {
  return (
    <Button
      onClick={onClick}
      className={`h-auto p-6 bg-gradient-to-br ${gradient} hover:scale-105 transition-all duration-200 border-0`}
    >
      <div className="flex flex-col items-center space-y-3 text-white">
        <div className="p-3 bg-white/20 rounded-xl backdrop-blur-sm">
          {icon}
        </div>
        <div className="text-center">
          <h4 className="text-sm font-medium">{title}</h4>
          <p className="text-xs text-white/80">{subtitle}</p>
        </div>
      </div>
    </Button>
  );
}

export function QuickActions() {
  return (
    <Card className="bg-black/40 backdrop-blur-lg border-white/10">
      <CardContent className="p-6">
        <h3 className="text-lg text-white mb-6">Quick Actions</h3>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
          <ActionButton
            icon={<Play className="h-6 w-6" />}
            title="Start Workout"
            subtitle="Begin session"
            gradient="from-purple-600 to-purple-800"
          />
          <ActionButton
            icon={<Camera className="h-6 w-6" />}
            title="Form Check"
            subtitle="AI analysis"
            gradient="from-blue-600 to-blue-800"
          />
          <ActionButton
            icon={<Utensils className="h-6 w-6" />}
            title="Optimize Meals"
            subtitle="Generate plan"
            gradient="from-green-600 to-green-800"
          />
          <ActionButton
            icon={<BarChart3 className="h-6 w-6" />}
            title="View Progress"
            subtitle="Analytics"
            gradient="from-orange-600 to-orange-800"
          />
          <ActionButton
            icon={<Calendar className="h-6 w-6" />}
            title="Schedule"
            subtitle="Plan workouts"
            gradient="from-pink-600 to-pink-800"
          />
          <ActionButton
            icon={<Target className="h-6 w-6" />}
            title="Set Goals"
            subtitle="Track targets"
            gradient="from-indigo-600 to-indigo-800"
          />
        </div>
      </CardContent>
    </Card>
  );
}