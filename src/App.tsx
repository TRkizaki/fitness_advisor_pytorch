import { Navigation } from "./components/Navigation";
import { StatsCards } from "./components/StatsCards";
import { WorkoutPanel } from "./components/WorkoutPanel";
import { ProgressCharts } from "./components/ProgressCharts";
import { QuickActions } from "./components/QuickActions";
import { ApiTest } from "./components/ApiTest";

export default function App() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
      {/* Background Effects */}
      <div className="fixed inset-0">
        <div className="absolute inset-0 bg-gradient-to-br from-purple-500/10 via-blue-500/10 to-indigo-500/10"></div>
        <div className="absolute top-0 left-1/4 w-96 h-96 bg-purple-500/20 rounded-full blur-3xl"></div>
        <div className="absolute bottom-0 right-1/4 w-96 h-96 bg-blue-500/20 rounded-full blur-3xl"></div>
      </div>
      
      {/* Main Content */}
      <div className="relative z-10">
        <Navigation />
        
        <main className="max-w-7xl mx-auto p-6 space-y-8">
          {/* Hero Section with Stats */}
          <div className="space-y-6">
            <div className="text-center space-y-2">
              <h2 className="text-3xl text-white">Welcome back, Alex!</h2>
              <p className="text-white/70">Here's your fitness overview for today</p>
            </div>
            <StatsCards />
          </div>
          
          {/* Workout Tracking Panel */}
          <WorkoutPanel />
          
          {/* Backend API Integration Test */}
          <ApiTest />
          
          {/* Progress Charts */}
          <div className="space-y-6">
            <h2 className="text-2xl text-white">Progress Analytics</h2>
            <ProgressCharts />
          </div>
          
          {/* Quick Actions */}
          <QuickActions />
        </main>
      </div>
    </div>
  );
}