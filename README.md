# Fitness Advisor AI

A comprehensive AI-powered fitness and nutrition advisory system built with Rust backend and React frontend integration.

## Architecture Overview

### Current Branch: `react-frontend-integration`

This branch implements **Phase 3: Basic React Frontend Integration** with a modern React 18 interface that communicates with the Rust backend through RESTful APIs. This branch provides the foundational frontend infrastructure for the fitness advisor system.

## Features

### **React Frontend Integration (Phase 3) - CURRENT**
- **Modern React UI**: React 18 with TypeScript and modern hooks
- **Backend API Integration**: RESTful API client for Rust backend communication
- **Comprehensive Component Library**: 30+ pre-built UI components with Radix UI
- **Real-time Workout Tracking**: Interactive workout panel with form analysis visualization
- **Progress Visualization**: Advanced charts for weight tracking, calories, and strength progress
- **Responsive Design**: Glass morphism design with mobile-first approach
- **API Testing Interface**: Built-in backend connection testing and user management

### **Backend Infrastructure** 
- **Rust Core**: High-performance, memory-safe backend
- **RESTful APIs**: Complete API endpoints for frontend integration
- **Menu Optimization**: Genetic algorithms for meal planning
- **ML Analytics**: Motion analysis and pattern recognition
- **Database Layer**: SQLite with SQLx integration
- **Configuration Management**: TOML-based settings

## System Architecture

```
fitness_advisor_ai/
├── backend/                 # Rust backend services
│   ├── src/
│   │   ├── advisors/       # Fitness advisory algorithms
│   │   ├── models/         # Data models and schemas
│   │   ├── core/           # Core utilities and errors
│   │   └── main.rs         # Application entry point
│   ├── tests/              # Backend test suite
│   └── Cargo.toml          # Rust dependencies
├── frontend/               # React frontend application
│   ├── src/
│   │   ├── components/     # React UI components
│   │   │   ├── WorkoutPanel.tsx      # Real-time workout tracking
│   │   │   ├── ProgressCharts.tsx    # Progress visualization charts
│   │   │   ├── ApiTest.tsx           # Backend API testing interface
│   │   │   ├── Navigation.tsx        # Main navigation component
│   │   │   ├── StatsCards.tsx        # Dashboard statistics display
│   │   │   ├── QuickActions.tsx      # Quick action buttons
│   │   │   └── ui/                   # 30+ Radix UI components
│   │   ├── api/
│   │   │   └── client.ts             # Backend API client (240+ lines)
│   │   ├── styles/         # CSS and styling files
│   │   └── App.tsx         # Main application component
│   ├── package.json        # Node.js dependencies
│   └── vite.config.ts      # Vite build configuration
├── ml-services/           # Python ML processing services
├── docs/                  # Documentation and guides
└── scripts/               # Build and utility scripts
```

## React Frontend Capabilities

### Component Architecture
- **WorkoutPanel**: Real-time workout tracking with camera feed simulation and form analysis metrics
- **ProgressCharts**: Interactive charts for weight progress, calories burned, and strength tracking
- **ApiTest**: Backend connectivity testing with user data display and health checks
- **Navigation**: Responsive navigation with glass morphism design
- **StatsCards**: Dashboard overview with key fitness metrics
- **UI Library**: Complete set of 30+ components based on Radix UI primitives

### Backend Integration
- **RESTful API Client**: Type-safe HTTP client with error handling
- **User Management**: User data fetching and display
- **Health Monitoring**: Backend connectivity and status checking
- **Real-time Updates**: WebSocket support for live data updates
- **Error Handling**: Comprehensive error management with user feedback

### Design System
- **Glass Morphism**: Modern transparent layered design aesthetic
- **Responsive Layout**: Mobile-first approach with breakpoint optimization
- **Dark Theme**: Professional dark theme with gradient backgrounds
- **Interactive Elements**: Smooth animations and hover effects
- **Typography**: Carefully chosen font hierarchy and spacing

### Performance Features
- **Vite Build System**: Fast development server and optimized production builds
- **Code Splitting**: Automatic code splitting for optimal loading
- **Tree Shaking**: Dead code elimination for smaller bundles
- **TypeScript**: Full type safety with strict mode enabled
- **Modern React**: Hooks-based architecture with concurrent features

## API Endpoints

### Backend REST APIs
```bash
# User Management
GET    /api/users              # Get all users
GET    /api/users/:id          # Get specific user
GET    /api/users/:id/recommendations  # Get user recommendations
GET    /api/users/:id/progress # Get user progress data

# Health & Status
GET    /api/health             # Backend health check
GET    /api/ml/status          # ML service status

# Menu Optimization
POST   /api/menu/optimize      # Optimize meal plans

# ML Analysis
POST   /api/ml/analyze-frame   # Analyze workout form
```

### Example API Usage
```typescript
// Fetch all users
const users = await FitnessApiClient.getUsers();

// Get user recommendations
const recommendations = await FitnessApiClient.getUserRecommendations(userId);

// Health check
const isHealthy = await FitnessApiClient.checkHealth();

// Analyze workout frame
const analysis = await FitnessApiClient.analyzeFrame(frameBase64, 'realtime');
```

## Tech Stack

### Frontend
- **Framework**: React 18 with TypeScript and modern hooks
- **Build Tool**: Vite with optimized bundling
- **Styling**: Tailwind CSS with glass morphism design system
- **UI Components**: Radix UI primitives with custom theming
- **Charts**: Recharts for interactive data visualization
- **Icons**: Lucide React for consistent iconography
- **Development**: ESLint, TypeScript strict mode

### Backend
- **Language**: Rust 2021 Edition
- **Web Framework**: Axum with Tower middleware
- **Database**: SQLite with SQLx
- **Configuration**: TOML-based settings
- **Error Handling**: Custom error types with detailed messaging

### Dependencies
```json
{
  "dependencies": {
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "@radix-ui/react-*": "Latest versions",
    "recharts": "^2.15.2",
    "lucide-react": "^0.487.0",
    "tailwind-merge": "*",
    "class-variance-authority": "^0.7.1"
  },
  "devDependencies": {
    "vite": "6.3.5",
    "@vitejs/plugin-react-swc": "^3.10.2",
    "typescript": "latest"
  }
}
```

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+
- npm or yarn

### Backend Setup
```bash
cd backend
cargo build --release
cargo test  # Run test suite
cargo run   # Start backend server on localhost:3000
```

### Frontend Setup
```bash
cd frontend
npm install
npm run dev  # Start development server on localhost:5173
```

### Full Stack Development
```bash
# Terminal 1: Start backend
cd backend && cargo run

# Terminal 2: Start frontend
cd frontend && npm run dev

# Access application at http://localhost:5173
```

## Key Features in Detail

### Real-time Workout Tracking
- **Camera Feed Simulation**: Visual representation of live video analysis
- **Form Analysis Metrics**: Real-time feedback on squat depth, knee alignment, posture
- **Rep Counting**: Automatic repetition tracking with accuracy indicators
- **Exercise Recognition**: AI-powered exercise identification

### Progress Analytics
- **Weight Tracking**: Historical weight data with trend analysis
- **Strength Progress**: Bench press, squat, and deadlift progression tracking
- **Calorie Monitoring**: Weekly calorie burn visualization
- **Goal Setting**: Personal fitness goal tracking with progress indicators

### Backend Integration
- **Live Health Checks**: Real-time backend connectivity monitoring
- **User Data Management**: Complete user profile and fitness data handling
- **Error Handling**: Comprehensive error states with user-friendly messages
- **API Testing**: Built-in tools for testing backend connectivity

## Development Roadmap

### Phase 1: Backend Foundation (COMPLETED)
- Rust backend architecture
- Database integration
- Basic API endpoints
- Menu optimization algorithms

### Phase 2: Core ML Services (COMPLETED)
- Motion analysis capabilities
- Form recognition algorithms
- Real-time processing pipeline
- ML model integration

### Phase 3: React Frontend Integration (CURRENT BRANCH)
- **Modern React UI**: Complete interface with TypeScript
- **Backend Integration**: RESTful API client with error handling
- **Component Library**: Comprehensive UI component system
- **Real-time Features**: Live workout tracking and progress visualization
- **Responsive Design**: Mobile-first glass morphism interface

### Phase 4: Advanced Features (PLANNED)
- Real camera integration
- Advanced ML form analysis
- Social features and sharing
- Mobile app development

### Phase 5: Production Deployment (PLANNED)
- Docker containerization
- Cloud deployment
- Performance optimization
- User authentication system

## API Client Features

The `FitnessApiClient` provides comprehensive backend integration:

```typescript
export class FitnessApiClient {
  // User management
  static async getUsers(): Promise<User[]>
  static async getUser(userId: string): Promise<User | null>
  static async getUserRecommendations(userId: string)
  static async getUserProgress(userId: string)

  // Menu optimization
  static async optimizeMealPlan(request: OptimizationRequest)

  // ML Analysis
  static async analyzeFrame(frameBase64: string, analysisType: string)

  // Health checks
  static async checkHealth(): Promise<boolean>
  static async checkMLServiceStatus()

  // Real-time communication
  static createWebSocket(onMessage: (data: any) => void): WebSocket
}
```

## Component Features

### WorkoutPanel
- Real-time form analysis simulation
- Camera feed visualization
- Progress metrics display
- Interactive controls for workout sessions

### ProgressCharts
- Weight progression area charts
- Weekly calorie burn line charts
- Strength training progress multi-line charts
- Responsive chart layouts with tooltips

### ApiTest
- Backend connectivity testing
- User data fetching and display
- Health status monitoring
- Error state handling with retry capabilities

## Contributing

### Development Guidelines
- Follow React best practices and hooks patterns
- Maintain TypeScript strict mode compliance
- Use semantic commit messages
- Ensure responsive design compatibility
- Test backend integration thoroughly

### Code Style
- ESLint configuration for consistent formatting
- TypeScript strict mode for type safety
- Tailwind CSS for styling consistency
- Component-based architecture
- Proper error boundary implementation

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Current Status

**Branch**: `react-frontend-integration`  
**Phase**: 3 (React Frontend Integration) - **CURRENT**  
**Next**: Advanced ML Integration  
**Features**: Modern React UI with complete backend integration  
**Documentation**: Complete setup and development guide  

### Implementation Statistics
- **Frontend Code**: 1,000+ lines of production React/TypeScript code
- **UI Components**: 30+ Radix UI components with custom theming
- **Backend Integration**: Complete RESTful API client
- **Charts & Visualization**: Interactive progress tracking charts
- **Design System**: Glass morphism with responsive layout
- **Type Safety**: Full TypeScript integration with strict mode

This branch provides the foundational React frontend that integrates seamlessly with the Rust backend, offering a modern, responsive, and feature-rich user interface for the fitness advisor system.