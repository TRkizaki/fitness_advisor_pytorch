import { Card, CardContent } from "./ui/card";
import { Activity, Target, TrendingUp } from "lucide-react";

interface StatCardProps {
  title: string;
  value: string;
  subtitle: string;
  icon: React.ReactNode;
  gradient: string;
}

function StatCard({ title, value, subtitle, icon, gradient }: StatCardProps) {
  return (
    <Card className={`bg-gradient-to-br ${gradient} border-0 text-white relative overflow-hidden`}>
      <div className="absolute inset-0 bg-black/10 backdrop-blur-sm"></div>
      <CardContent className="p-6 relative z-10">
        <div className="flex items-center justify-between mb-4">
          <div className="p-2 bg-white/20 rounded-lg backdrop-blur-sm">
            {icon}
          </div>
        </div>
        <div className="space-y-2">
          <p className="text-white/80 text-sm">{title}</p>
          <p className="text-2xl font-medium">{value}</p>
          <p className="text-white/70 text-sm">{subtitle}</p>
        </div>
      </CardContent>
    </Card>
  );
}

export function StatsCards() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <StatCard
        title="Body Mass Index"
        value="22.4"
        subtitle="Normal Range"
        icon={<Activity className="h-5 w-5 text-white" />}
        gradient="from-purple-500 to-purple-700"
      />
      <StatCard
        title="Total Daily Energy"
        value="2,340"
        subtitle="kcal/day"
        icon={<Target className="h-5 w-5 text-white" />}
        gradient="from-blue-500 to-blue-700"
      />
      <StatCard
        title="Fitness Level"
        value="Advanced"
        subtitle="8.5/10 Score"
        icon={<TrendingUp className="h-5 w-5 text-white" />}
        gradient="from-indigo-500 to-purple-600"
      />
    </div>
  );
}