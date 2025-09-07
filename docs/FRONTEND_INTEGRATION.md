# Frontend Integration Guide

## Current Setup (React Branch)

### Branch: `react-frontend-integration`
- **React Frontend**: `http://localhost:5173`  
- **Rust API**: `http://localhost:3000`
- **Proxy**: Vite proxies `/api/*` to Rust backend

### Components Converted from Figma Make:
- ✅ **Navigation** - Top header with FitAdvisor Pro branding
- ✅ **StatsCards** - BMI, TDEE, Fitness Level display
- ✅ **MenuOptimization** - Genetic algorithm controls + meal plan
- ✅ **WorkoutPanel** - Real-time camera feed + form analysis
- ✅ **ProgressCharts** - Weight/Calories/Strength analytics  
- ✅ **QuickActions** - 6 colorful action buttons

### API Integration:
- `FitnessApiClient` class in `src/api/client.ts`
- Connects to existing Rust endpoints:
  - `/api/users/*` - User management
  - `/api/menu/optimize` - Menu optimization (TODO: implement endpoint)
  - `/api/ml/*` - ML analysis
  - `/api/ai/realtime` - WebSocket real-time analysis

## Running the Frontend

```bash
# Start Rust API (port 3000)
cargo run

# Start React Frontend (port 5173) 
npm run dev
```

## Future Migration to Leptos (Full Rust Stack)

### Leptos Setup (Started):
- ✅ Dependencies added to `Cargo.toml`
- ✅ Components converted in `src/frontend/`
- ⏳ Compilation issues to resolve
- 🎯 Goal: Full WebAssembly Rust frontend

### Migration Steps:
1. Fix Leptos import/compilation issues
2. Install and configure Trunk bundler
3. Test WebAssembly build
4. Migrate API integration to gloo-net
5. Switch to Leptos branch when ready

## Design Source
Original Figma design and generated code preserved in:
- `Fitness_Advisor_AI.zip` - Complete React app from Figma Make
- Figma design includes all components with perfect styling