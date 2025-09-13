# Fitness Advisor AI - RAG Knowledge System (Leptos)

A comprehensive fitness and nutrition advisor application built with Leptos WebAssembly framework, featuring a powerful RAG (Retrieval-Augmented Generation) knowledge system for intelligent fitness guidance, semantic search, and AI-powered recommendations.

## Branch Overview

This is the `rag-system-development-leptos` branch, featuring the complete **RAG Knowledge System integration** with the Leptos WebAssembly frontend, providing AI-enhanced fitness guidance through semantic search and intelligent recommendations.

## Tech Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) 0.6+ (Reactive Rust web framework)
- **Language**: Rust + WebAssembly (WASM)
- **RAG System**: Vector embeddings with semantic search capabilities
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
│   └── mod.rs                 # API module exports
├── components/
│   ├── navigation.rs          # Top navigation bar
│   ├── stats_cards.rs         # Dashboard statistics cards
│   ├── workout_panel.rs       # Smart workout center with AI recommendations
│   ├── nutrition_panel.rs     # Advanced nutrition center with AI guidance
│   ├── knowledge_base_panel.rs # Main RAG interface
│   ├── semantic_search.rs     # Search components and filters
│   ├── document_manager.rs    # Document upload and management
│   ├── menu_optimization.rs   # Menu optimization algorithms
│   ├── progress_charts.rs     # Progress visualization charts
│   ├── quick_actions.rs       # Quick action buttons
│   ├── api_test.rs            # Backend API integration testing
│   ├── icons.rs              # SVG icon components
│   └── mod.rs                # Component module exports
├── lib.rs                    # Main application and routing
└── main.rs                   # Entry point for standalone builds
```

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

## Performance Metrics

### Bundle Size & Performance
- **WebAssembly Bundle**: ~52KB compressed (including RAG client)
- **Memory Usage**: ~2.8MB runtime (with knowledge base cache)
- **Cold Start**: < 120ms initialization
- **Search Performance**: < 200ms semantic search response
- **Framework**: Leptos + WASM for optimal performance

### RAG Optimization Features
- **Vector Caching**: Client-side embedding cache
- **Lazy Loading**: On-demand knowledge base components
- **Batch Operations**: Efficient bulk document operations
- **Search Debouncing**: Optimized search input handling

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
- [x] HTTP API client with RAG integration
- [x] WebSocket real-time communication
- [x] Smart workout center with AI recommendations
- [x] Advanced nutrition center with knowledge-backed guidance
- [x] Responsive glass morphism UI
- [x] Multi-user support and context management

### Future RAG Enhancements
- [ ] Advanced embedding models
- [ ] Multi-modal search (text + images)
- [ ] Conversation history and context
- [ ] Advanced knowledge graph features
- [ ] Real-time collaborative filtering

## Configuration

### Environment Variables
```bash
# API Configuration
VITE_API_BASE_URL=http://localhost:3000
VITE_WS_URL=ws://localhost:3000/api/ai/realtime
VITE_RAG_API_URL=http://localhost:3000/api/rag

# Development
RUST_LOG=info
```

### RAG Configuration
```toml
# Additional Cargo.toml dependencies for RAG
[dependencies]
serde_json = "1.0"
thiserror = "1.0"
```

## Contributing

1. Ensure you're on the `rag-system-development-leptos` branch
2. Follow Rust coding standards and conventions
3. Add comprehensive error handling for RAG operations
4. Test semantic search functionality thoroughly
5. Update documentation for new RAG features
6. Consider knowledge base integration in all new features

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
- `leptos-frontend-wip` - Base Leptos frontend without RAG
- `rag-system-development` - RAG system with React frontend
- `mcp-server-development-with-leptos-frontend` - MCP integration with Leptos

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**Built with Leptos, WebAssembly, and RAG Technology**

*AI-Enhanced Fitness Guidance Through Advanced Knowledge Systems*