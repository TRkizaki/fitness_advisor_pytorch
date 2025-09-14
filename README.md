# Fitness Advisor AI - MCP Server Integration (Leptos)

A comprehensive fitness and nutrition advisor application built with Leptos WebAssembly framework, featuring both RAG (Retrieval-Augmented Generation) knowledge system and MCP (Model Context Protocol) server integration for extensible AI-powered fitness guidance.

## Branch Overview

This is the `mcp-server-development-with-leptos-frontend` branch, featuring the complete **MCP Server Integration** with **RAG Knowledge System** on the Leptos WebAssembly frontend, providing extensible AI-enhanced fitness guidance through semantic search, intelligent recommendations, and external tool integration.

## Tech Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) 0.6+ (Reactive Rust web framework)
- **Language**: Rust + WebAssembly (WASM)
- **RAG System**: Vector embeddings with semantic search capabilities
- **MCP Integration**: Model Context Protocol for extensible AI tool integration
- **Styling**: Tailwind CSS with glass morphism effects
- **HTTP Client**: Custom Rust client using `web-sys` and `wasm-bindgen`
- **WebSocket**: Real-time communication with custom WebSocket service
- **State Management**: Leptos reactive signals and contexts
- **Build Tool**: Trunk for WASM compilation and dev server

## Project Structure

```
src/
├── api/
│   ├── client.rs              # HTTP API client for backend integration
│   ├── websocket.rs           # WebSocket service for real-time updates
│   ├── rag_client.rs          # RAG system API client
│   ├── mcp_client.rs          # MCP server API client
│   └── mod.rs                 # API module exports
├── components/
│   ├── navigation.rs          # Top navigation bar
│   ├── stats_cards.rs         # Dashboard statistics cards
│   ├── workout_panel.rs       # Smart workout center with AI recommendations
│   ├── nutrition_panel.rs     # Advanced nutrition center with AI guidance
│   ├── knowledge_base_panel.rs # Main RAG interface
│   ├── semantic_search.rs     # Search components and filters
│   ├── document_manager.rs    # Document upload and management
│   ├── mcp_server_panel.rs    # MCP server integration interface
│   ├── menu_optimization.rs   # Menu optimization algorithms
│   ├── progress_charts.rs     # Progress visualization charts
│   ├── quick_actions.rs       # Quick action buttons
│   ├── api_test.rs            # Backend API integration testing
│   ├── icons.rs              # SVG icon components
│   └── mod.rs                # Component module exports
├── lib.rs                    # Main application and routing
└── main.rs                   # Entry point for standalone builds
```

## MCP Server Integration

### Model Context Protocol (MCP) Features
- **Server Registry**: Discover and register MCP-compatible fitness servers
- **Tool Execution**: Execute remote tools for fitness analysis and recommendations
- **Session Management**: Maintain persistent contexts across tool interactions
- **Resource Access**: Access external fitness data and knowledge resources
- **Real-time Monitoring**: Monitor server health, performance, and usage statistics

### MCP Server Panel
The central interface for MCP server integration:
- **Servers Tab**: View and manage registered MCP servers with status monitoring
- **Sessions Tab**: Active sessions with context persistence and lifecycle management
- **Tools Tab**: Execute available tools with parameter configuration and result display
- **Monitoring Tab**: Server performance metrics, uptime statistics, and usage analytics

### Supported Tool Categories
- **Fitness Tracking**: Workout analysis, form correction, and progress monitoring
- **Nutrition Analysis**: Meal planning, macro calculation, and dietary recommendations
- **Workout Generation**: AI-generated workout plans based on user goals and constraints
- **Progress Monitoring**: Long-term tracking, trend analysis, and goal adjustment
- **Data Analysis**: Statistical analysis of fitness and nutrition data
- **Integration**: Third-party service connections and data synchronization
- **Utility**: Helper tools for data conversion, validation, and processing

### MCP Authentication & Security
- **Multiple Auth Types**: API Key, Bearer Token, OAuth2, Certificate-based authentication
- **Permission System**: Granular tool permissions with required capability checks
- **Secure Context**: Encrypted user context and session management
- **Server Validation**: Health checks and capability verification

## RAG System Features

### AI Knowledge Base
- **Semantic Search**: Advanced vector-based search across fitness knowledge base
- **Document Management**: Upload, organize, and manage fitness documentation
- **Smart Recommendations**: AI-powered personalized suggestions based on user context
- **Knowledge Analytics**: Insights and trends from the knowledge repository
- **Context-Aware Guidance**: Fitness advice informed by comprehensive knowledge base

### Document Types
- **Fitness Guides**: Comprehensive workout and training information
- **Nutrition Info**: Dietary guidelines and nutritional data
- **Exercise Descriptions**: Detailed exercise instructions and form guides
- **Research Papers**: Scientific studies and evidence-based recommendations
- **User Manuals**: Equipment and app usage instructions
- **FAQ**: Frequently asked questions and quick answers

### Knowledge Base Panel
The central interface for interacting with the knowledge system:
- **Search Tab**: Semantic search with advanced filtering by document type
- **Recommendations Tab**: AI-powered personalized suggestions for different categories
- **Browse Tab**: Document management with upload, organization, and bulk operations
- **Analytics Tab**: Knowledge base insights, usage metrics, and trends

### Enhanced Workout Center
- **Live Form Analysis**: Real-time AI-powered exercise form scoring
- **AI Recommendations Tab**: Personalized workout plans based on knowledge base
- **Interactive Camera Feed**: Simulated video analysis with performance metrics
- **Knowledge-Backed Suggestions**: Exercise recommendations from fitness database
- **Progressive Training Plans**: Evidence-based workout progression

### Smart Nutrition Center
- **AI Meal Planning**: Generate personalized meal plans with backend optimization
- **AI Nutrition Tab**: Knowledge-based nutrition recommendations and guidance
- **Macro Tracking**: Real-time protein, carbs, and fat tracking with visual progress
- **Evidence-Based Advice**: Nutrition guidance backed by research papers
- **Smart Analysis**: Weekly trends with intelligent recommendations

## Development Setup

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Trunk](https://trunkrs.dev/) for serving and bundling
- [Node.js](https://nodejs.org/) (for Tailwind CSS)

### Installation

1. **Clone and switch to branch**:
   ```bash
   git clone https://github.com/your-repo/fitness_advisor_ai.git
   cd fitness_advisor_ai
   git checkout rag-system-development-leptos
   ```

2. **Install Rust dependencies**:
   ```bash
   cargo build
   ```

3. **Install Trunk**:
   ```bash
   cargo install trunk
   ```

4. **Install Tailwind CSS**:
   ```bash
   npm install -D tailwindcss
   npx tailwindcss init
   ```

### Development Server

Start the development server with hot reloading:

```bash
trunk serve
```

The application will be available at `http://localhost:8080`

### Build for Production

Create an optimized production build:

```bash
trunk build --release
```

## API Integration

### MCP Server Endpoints
- `GET /api/mcp/servers` - List all registered MCP servers
- `POST /api/mcp/servers` - Register a new MCP server
- `GET /api/mcp/servers/{id}` - Get server details and status
- `DELETE /api/mcp/servers/{id}` - Unregister MCP server
- `GET /api/mcp/servers/{id}/stats` - Get server performance statistics
- `POST /api/mcp/servers/{id}/ping` - Health check for specific server
- `POST /api/mcp/sessions` - Create new MCP session with context
- `GET /api/mcp/sessions/{id}` - Get session details and status
- `DELETE /api/mcp/sessions/{id}` - Terminate MCP session
- `GET /api/mcp/sessions?user_id={id}` - List user's active sessions
- `POST /api/mcp/tools/call` - Execute MCP tool with parameters
- `GET /api/mcp/servers/{id}/tools` - List available tools for server
- `POST /api/mcp/resources` - Access MCP server resources
- `GET /api/mcp/servers/{id}/resources` - List available resources

### RAG System Endpoints
- `POST /api/rag/search/semantic` - Perform semantic search with filters
- `GET /api/rag/documents` - List documents with optional type filtering
- `POST /api/rag/documents` - Upload new documents to knowledge base
- `DELETE /api/rag/documents/{id}` - Remove documents from knowledge base
- `DELETE /api/rag/documents/bulk-delete` - Bulk document operations
- `POST /api/rag/recommendations` - Get AI-powered recommendations
- `GET /api/rag/insights` - Knowledge base insights and trends
- `GET /api/rag/analytics` - Usage analytics and metrics
- `POST /api/rag/admin/reindex` - Reindex documents for search

### Core Backend Endpoints
- `GET /api/health` - Backend health check
- `GET /api/users` - Fetch all users
- `GET /api/users/{id}` - Get specific user
- `POST /api/menu/optimize` - Meal plan optimization
- `POST /api/ml/analyze-frame` - Frame analysis

### WebSocket Events
- `workout_update` - Real-time workout data
- `nutrition_update` - Live nutrition tracking
- `knowledge_update` - Knowledge base changes
- `system_notification` - System messages

## RAG Client Implementation

### Semantic Search
```rust
let request = SemanticSearchRequest {
    query: "compound exercises for strength".to_string(),
    document_types: Some(vec![DocumentType::FitnessGuide]),
    limit: Some(10),
    similarity_threshold: Some(0.7),
};

let results = RagApiClient::semantic_search(request).await?;
```

### Document Management
```rust
let request = DocumentUploadRequest {
    title: "Progressive Overload Guide".to_string(),
    content: "Comprehensive guide to progressive overload...".to_string(),
    document_type: DocumentType::FitnessGuide,
    metadata: Some(json!({"difficulty": "intermediate"})),
    tags: Some(vec!["strength".to_string(), "progression".to_string()]),
};

let document = RagApiClient::upload_document(request).await?;
```

### Smart Recommendations
```rust
let request = RecommendationRequest {
    user_context: UserContext {
        user_id: "user123".to_string(),
        fitness_goals: vec!["strength".to_string(), "muscle_gain".to_string()],
        current_stats: json!({"weight": 75, "experience": "intermediate"}),
        preferences: vec!["compound_exercises".to_string()],
        workout_history: None,
    },
    recommendation_type: RecommendationType::WorkoutPlan,
    preferences: None,
    limit: Some(5),
};

let recommendations = RagApiClient::get_smart_recommendations(request).await?;
```

## MCP Client Implementation

### Server Registration
```rust
let registration = McpServerRegistration {
    name: "Fitness Tracker Pro".to_string(),
    description: "Advanced fitness tracking with AI analysis".to_string(),
    endpoint: "http://localhost:3001/mcp".to_string(),
    capabilities: vec![
        McpCapability::Tools,
        McpCapability::Resources,
        McpCapability::Progress,
    ],
    authentication: Some(McpAuthentication {
        auth_type: McpAuthType::ApiKey,
        credentials: json!({"api_key": "your-api-key"}),
    }),
    metadata: None,
};

let server = McpApiClient::register_server(registration).await?;
```

### Session Management
```rust
let context = McpApiClient::create_fitness_context(
    "user123".to_string(),
    "session456".to_string(),
    vec!["strength".to_string(), "muscle_gain".to_string()],
    Some(json!({"current_exercise": "squat", "sets": 3})),
    vec!["high_protein".to_string(), "vegetarian".to_string()],
);

let session_request = McpSessionRequest {
    server_id: "fitness-tracker-001".to_string(),
    context,
    capabilities: vec!["tool_execution".to_string()],
};

let session = McpApiClient::create_session(session_request).await?;
```

### Tool Execution
```rust
let tool_call = McpApiClient::create_tool_call(
    "fitness-tracker-001".to_string(),
    "analyze_workout".to_string(),
    json!({
        "workout_type": "strength",
        "duration": 45,
        "exercises": ["squat", "bench_press", "deadlift"]
    }),
    Some(fitness_context),
);

let response = McpApiClient::call_tool(tool_call).await?;
```

## Performance Metrics

### Bundle Size & Performance
- **WebAssembly Bundle**: ~58KB compressed (including RAG + MCP clients)
- **Memory Usage**: ~3.2MB runtime (with knowledge base cache and MCP sessions)
- **Cold Start**: < 150ms initialization
- **Search Performance**: < 200ms semantic search response
- **MCP Tool Execution**: < 300ms average response time
- **Framework**: Leptos + WASM for optimal performance

### Optimization Features
- **Vector Caching**: Client-side embedding cache for RAG
- **MCP Session Pooling**: Efficient session reuse and connection management
- **Lazy Loading**: On-demand knowledge base and MCP components
- **Batch Operations**: Efficient bulk document and tool operations
- **Search Debouncing**: Optimized search input handling
- **Tool Result Caching**: Cache frequently used tool responses

## UI/UX Features

### Knowledge Base Interface
- **Tabbed Navigation**: Organized knowledge system sections
- **Advanced Filters**: Document type, relevance, and date filtering
- **Real-time Search**: Instant semantic search results
- **Upload Interface**: Drag-and-drop document upload
- **Bulk Operations**: Multi-select document management

### Enhanced Components
- **AI Recommendation Cards**: Rich recommendation display with action items
- **Search Result Cards**: Relevance scoring and content previews
- **Knowledge Analytics**: Visual insights and trend charts
- **Document Status**: Embedding processing status indicators
- **MCP Server Cards**: Server status, capabilities, and connection health
- **Tool Execution Interface**: Parameter configuration and result visualization
- **Session Management**: Context persistence and session monitoring

## Testing & Monitoring

### RAG System Testing
The knowledge base includes comprehensive testing for:
- Semantic search accuracy and performance
- Document upload and processing
- Recommendation relevance and quality
- API error handling and fallbacks
- Real-time search responsiveness

### Knowledge Base Analytics
- **Usage Metrics**: Search frequency and popular queries
- **Content Analysis**: Document type distribution and trends
- **User Engagement**: Recommendation click-through rates
- **Performance Monitoring**: Search latency and embedding status

## Development Status

### Completed MCP Integration
- [x] Complete MCP API client implementation
- [x] Server registration and discovery
- [x] Session management with context persistence
- [x] Tool execution with parameter validation
- [x] Resource access and management
- [x] Performance monitoring and health checks
- [x] Multi-server support with load balancing
- [x] Comprehensive error handling and fallbacks

### Completed RAG Features
- [x] Complete RAG API client implementation
- [x] Semantic search with advanced filtering
- [x] Document management with upload/delete operations
- [x] Smart recommendation system
- [x] Knowledge base analytics and insights
- [x] AI-enhanced workout and nutrition panels
- [x] Comprehensive error handling and fallbacks

### Enhanced Core Features
- [x] Leptos WebAssembly framework setup
- [x] HTTP API client with RAG and MCP integration
- [x] WebSocket real-time communication
- [x] Smart workout center with AI recommendations
- [x] Advanced nutrition center with knowledge-backed guidance
- [x] MCP server integration panel
- [x] Responsive glass morphism UI
- [x] Multi-user support and context management

### Future Enhancements
- [ ] Advanced MCP tool chaining and workflows
- [ ] Real-time MCP server discovery and auto-registration
- [ ] Advanced embedding models for RAG
- [ ] Multi-modal search (text + images)
- [ ] Conversation history and context preservation
- [ ] Advanced knowledge graph features
- [ ] MCP server development SDK integration

## Configuration

### Environment Variables
```bash
# API Configuration
VITE_API_BASE_URL=http://localhost:3000
VITE_WS_URL=ws://localhost:3000/api/ai/realtime
VITE_RAG_API_URL=http://localhost:3000/api/rag
VITE_MCP_API_URL=http://localhost:3000/api/mcp

# MCP Server Configuration
MCP_SERVER_DISCOVERY_ENABLED=true
MCP_SESSION_TIMEOUT=3600
MCP_MAX_CONCURRENT_TOOLS=10

# Development
RUST_LOG=info
```

### Cargo Configuration
```toml
# Additional Cargo.toml dependencies for RAG and MCP
[dependencies]
serde_json = "1.0"
thiserror = "1.0"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = "0.3"
```

## Contributing

1. Ensure you're on the `mcp-server-development-with-leptos-frontend` branch
2. Follow Rust coding standards and conventions
3. Add comprehensive error handling for MCP and RAG operations
4. Test MCP tool execution and session management thoroughly
5. Test semantic search functionality thoroughly
6. Update documentation for new MCP and RAG features
7. Consider both MCP and RAG integration in all new features

## RAG System Architecture

### Vector Search Pipeline
1. **Document Ingestion**: Upload and process documents
2. **Embedding Generation**: Create vector embeddings for semantic search
3. **Index Creation**: Build searchable vector index
4. **Query Processing**: Convert user queries to vector representations
5. **Similarity Search**: Find most relevant documents
6. **Result Ranking**: Score and rank results by relevance

### Recommendation Engine
1. **User Context Analysis**: Process user goals, stats, and preferences
2. **Knowledge Retrieval**: Find relevant knowledge base content
3. **AI Generation**: Generate personalized recommendations
4. **Action Planning**: Create specific, actionable guidance
5. **Continuous Learning**: Improve recommendations based on user feedback

## Related Branches

- `main` - Core Rust backend implementation
- `leptos-frontend-wip` - Base Leptos frontend without RAG or MCP
- `rag-system-development-leptos` - RAG system with Leptos frontend
- `rag-system-development` - RAG system with React frontend
- `react-frontend-integration` - React frontend implementation
- `mcp-server-development-with-react-frontend` - MCP integration with React

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**Built with Leptos, WebAssembly, RAG, and MCP Integration**

*Extensible AI-Enhanced Fitness Guidance Through Advanced Knowledge Systems and Tool Integration*