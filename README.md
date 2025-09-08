# Fitness Advisor AI

A comprehensive AI-powered fitness and nutrition advisory system built with Rust backend, React frontend, and advanced MCP (Model Context Protocol) server capabilities.

## Architecture Overview

### Current Branch: `mcp-server-development-with-react-frontend`

This branch implements the **Phase 2: MCP Server Implementation** with complete Model Context Protocol 2025-06-18 specification compliance, building upon the established RAG knowledge system.

## Features

### **MCP Server Implementation (Phase 2)**
- **MCP 2025-06-18 Compliance**: Complete JSON-RPC 2.0 protocol implementation
- **Multi-Transport Support**: STDIO, WebSocket, and HTTP transport layers
- **Authentication System**: JWT + API key authentication with session management
- **Fitness-Specific Tools**: 5 specialized tools for workout planning and nutrition analysis
- **Comprehensive Testing**: 1,400+ lines of test coverage with integration tests
- **Production Architecture**: Async/await design with robust error handling

### **RAG Knowledge System (Phase 1)**
- **Vector Embeddings**: ONNX Runtime + HuggingFace tokenizers
- **Semantic Search**: Qdrant vector database integration
- **Document Processing**: PDF extraction, web scraping, intelligent text chunking
- **Knowledge Base**: Multi-format document management and storage
- **LLM Integration**: Expandable response generation system
- **RESTful API**: Complete API endpoints for frontend integration

### **Frontend Integration**
- **React UI**: Modern responsive interface with TypeScript
- **Component Library**: Comprehensive UI components with Tailwind CSS
- **API Client**: Type-safe API integration layer
- **Real-time Features**: WebSocket support for live updates

### **Backend Infrastructure** 
- **Rust Core**: High-performance, memory-safe backend
- **Menu Optimization**: Genetic algorithms for meal planning
- **ML Analytics**: Motion analysis and pattern recognition
- **Database Layer**: SQLite with SQLx integration
- **Configuration Management**: TOML-based settings

## System Architecture

```
fitness_advisor_ai/
├── backend/                 # Rust backend services
│   ├── src/
│   │   ├── mcp/            # MCP server implementation
│   │   │   ├── types.rs           # MCP type system (400+ lines)
│   │   │   ├── protocol.rs        # JSON-RPC 2.0 handler (600+ lines)
│   │   │   ├── server.rs          # MCP server lifecycle
│   │   │   ├── transport.rs       # Multi-transport layer
│   │   │   ├── auth.rs            # JWT + API key authentication
│   │   │   ├── fitness_tools.rs   # Workout planning tools
│   │   │   └── nutrition_tools.rs # Nutrition analysis tools
│   │   ├── rag/            # RAG system core
│   │   │   ├── embeddings.rs        # ONNX embedding service
│   │   │   ├── vector_store.rs      # Qdrant vector operations
│   │   │   ├── knowledge_base.rs    # Document management
│   │   │   ├── document_processor.rs # PDF/HTML processing
│   │   │   ├── search.rs           # Semantic search engine
│   │   │   ├── llm_service.rs      # Response generation
│   │   │   └── api.rs              # REST API endpoints
│   │   ├── advisors/       # Fitness advisory algorithms
│   │   ├── models/         # Data models and schemas
│   │   └── core/           # Core utilities and errors
│   ├── tests/              # Comprehensive test suite
│   │   └── mcp/           # MCP server test suite
│   │       ├── test_protocol.rs      # Protocol tests (324 lines)
│   │       ├── test_fitness_tools.rs # Fitness tool tests (330 lines)
│   │       ├── test_nutrition_tools.rs # Nutrition tests (420+ lines)
│   │       ├── test_auth_manager.rs  # Authentication tests
│   │       ├── test_transport.rs     # Transport layer tests
│   │       └── integration_tests.rs  # End-to-end tests
│   └── examples/           # Example programs
│       └── mcp_test.rs           # MCP server demo
├── frontend/               # React frontend application
├── ml-services/           # Python ML processing services
└── docs/                  # Documentation and guides
```

## MCP Server Capabilities

### Protocol Implementation
- **JSON-RPC 2.0**: Complete bidirectional message protocol
- **Session Management**: Multi-client support with activity tracking
- **Error Handling**: Comprehensive error responses with detailed messaging
- **Resource Management**: Exercise databases and nutrition guidelines
- **Prompt Templates**: AI coaching templates for personalized guidance

### Fitness-Specific Tools
1. **Workout Plan Creation**: Personalized exercise routines with equipment/difficulty adaptation
2. **Nutrition Analysis**: Advanced meal planning with micronutrient analysis and interaction detection
3. **Progress Tracking**: Metrics analysis with trend identification and recommendations
4. **Seasonal Optimization**: Location-based workout adaptations for weather/climate
5. **RAG Fitness Query**: Knowledge-base powered fitness question answering

### Transport Layers
- **STDIO**: Command-line interface integration
- **WebSocket**: Real-time bidirectional communication
- **HTTP**: RESTful API compatibility

### Authentication & Security
- **JWT Tokens**: Configurable expiration and refresh
- **API Keys**: Named keys with granular permissions
- **Session Tracking**: IP address and user agent logging
- **Activity Monitoring**: Last login and usage analytics

## Testing Infrastructure

### Comprehensive Test Coverage (1,400+ lines)
- **Protocol Tests**: JSON-RPC message handling and session management
- **Tool Tests**: Fitness and nutrition tool functionality
- **Authentication Tests**: JWT validation and session lifecycle
- **Transport Tests**: Connection handling and protocol compliance
- **Integration Tests**: Complete workflow validation
- **Unit Tests**: Individual component validation

### Test Execution
```bash
# Quick test run
cd backend && cargo test

# MCP-specific tests
cd backend && cargo test mcp

# Integration tests only  
cd backend && cargo test integration_tests

# Full test suite with reporting
cd backend && ./run_rag_tests.sh
```

## RAG System Capabilities

### Document Processing
- **PDF Extraction**: Scientific papers and fitness guides
- **Web Scraping**: Health and fitness websites
- **Text Chunking**: Semantic boundary-aware segmentation
- **Metadata Management**: Source tracking and tagging

### Semantic Search
- **Vector Embeddings**: 384-dimensional embeddings via ONNX
- **Similarity Search**: Cosine similarity with configurable thresholds  
- **Result Ranking**: Relevance-based result ordering
- **Context Expansion**: Related content discovery

### Knowledge Base Topics
- **Exercise Science**: Cardio, strength training, HIIT, recovery
- **Nutrition**: Macronutrients, hydration, meal timing, supplements
- **Performance**: Athletic performance optimization
- **Health**: General wellness and injury prevention

## API Endpoints

### MCP Server APIs
```bash
# JSON-RPC 2.0 over STDIO/WebSocket/HTTP
{
  "jsonrpc": "2.0",
  "method": "initialize",
  "params": {
    "protocolVersion": "2025-06-18",
    "capabilities": {},
    "clientInfo": {"name": "Fitness AI Client", "version": "1.0.0"}
  },
  "id": 1
}

# Available MCP methods
initialize          # Initialize MCP session
tools/list         # List available tools
tools/call         # Call specific tool
resources/list     # List available resources
resources/read     # Read specific resource
prompts/list       # List available prompts
prompts/get        # Get specific prompt
logging/setLevel   # Configure logging
```

### RAG System APIs
```bash
POST   /documents          # Add text documents
POST   /documents/url      # Add web documents  
GET    /documents          # List all documents
GET    /documents/:id      # Get specific document
DELETE /documents/:id      # Remove document
POST   /search            # Semantic search
POST   /query             # RAG question answering
GET    /stats             # Knowledge base statistics
```

### Example MCP Tool Usage
```bash
# Create workout plan via MCP
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "create_workout_plan",
    "arguments": {
      "user_profile": {
        "age": 25,
        "weight_kg": 70.0,
        "fitness_goals": ["muscle_gain"],
        "activity_level": "moderately_active"
      },
      "workout_preferences": {
        "duration_minutes": 45,
        "difficulty_level": "intermediate",
        "equipment_available": ["dumbbells", "barbell"]
      }
    }
  },
  "id": 2
}

# Analyze nutrition via MCP
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "analyze_nutrition",
    "arguments": {
      "foods": [
        {"name": "chicken breast", "quantity": 4.0, "unit": "oz"},
        {"name": "brown rice", "quantity": 1.0, "unit": "cup"}
      ],
      "analysis_type": "micronutrients"
    }
  },
  "id": 3
}
```

## Tech Stack

### Backend
- **Language**: Rust 2021 Edition
- **MCP Framework**: Custom JSON-RPC 2.0 implementation
- **Web Framework**: Axum with Tower middleware
- **Database**: SQLite with SQLx
- **Authentication**: JWT + API keys with session management
- **ML Framework**: ONNX Runtime for embeddings
- **Vector Database**: Qdrant for semantic search
- **Text Processing**: HuggingFace tokenizers, text-splitter
- **Document Processing**: PDF extraction, HTML scraping

### Frontend  
- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **UI Components**: Custom component library
- **API Client**: Type-safe HTTP client

### Dependencies
```toml
# MCP (Model Context Protocol) server
jsonwebtoken = "9.2"           # JWT authentication for MCP

# Core embedding infrastructure (modular approach)
ort = "1.16"                   # ONNX Runtime for embeddings
hf-hub = "0.3"                 # Hugging Face Hub integration  
tokenizers = "0.19"            # Text tokenization
qdrant-client = "1.14.0"       # Vector database client
serde_derive = "1.0"           # For serializing embeddings

# Document processing
text-splitter = "0.13"         # Document chunking
pdf-extract = "0.7"            # PDF text extraction
scraper = "0.20"               # Web scraping for knowledge ingestion
```

## Getting Started

### Prerequisites
- Rust 1.70+ 
- Node.js 18+
- Qdrant vector database
- ONNX Runtime

### Backend Setup
```bash
cd backend
cargo build --release
cargo test  # Run test suite
cargo run   # Start backend server
```

### MCP Server Testing
```bash
cd backend
cargo run --example mcp_test  # Test MCP server functionality
```

### Frontend Setup  
```bash
cd frontend
npm install
npm run dev  # Start development server
```

### RAG System Setup
1. **Start Qdrant**: `docker run -p 6333:6333 qdrant/qdrant`
2. **Configure Models**: Download ONNX embedding models
3. **Load Sample Data**: Use provided fitness/nutrition content
4. **Test Pipeline**: Run `./run_rag_tests.sh`

## Development Roadmap

### Phase 1: RAG Knowledge System (COMPLETED)
- Vector embeddings and semantic search
- Document processing pipeline  
- Knowledge base management
- REST API implementation
- Comprehensive testing suite

### Phase 2: MCP Server Implementation (COMPLETED)
- **MCP 2025-06-18 Compliance**: Complete protocol implementation
- **Multi-Transport Layer**: STDIO, WebSocket, HTTP support
- **Authentication System**: JWT + API key management
- **Fitness Tools**: 5 specialized fitness/nutrition analysis tools
- **Test Suite**: 1,400+ lines of comprehensive test coverage
- **Production Architecture**: Async/await with robust error handling

### Phase 3: React Frontend Integration (IN PROGRESS)
- MCP client integration with React components
- Real-time workout plan generation UI
- Interactive nutrition analysis dashboard
- Progress tracking visualizations
- User authentication and session management

### Phase 4: Advanced Features (PLANNED)
- External integrations (Strava, MyFitnessPal)
- Advanced micronutrient analysis
- Seasonal optimization algorithms
- Production middleware (monitoring, rate limiting)

### Phase 5: Production Deployment (PLANNED)
- Docker containerization
- Load balancing and scaling
- Performance optimization
- Real-world user testing

## Contributing

### Development Guidelines
- Follow Rust best practices and idioms
- Maintain comprehensive test coverage
- Use semantic commit messages
- Update documentation for new features

### Testing Requirements
- Unit tests for all new components
- Integration tests for MCP workflows
- API tests for all endpoints
- Performance benchmarks for optimizations

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Related Documentation

- [MCP Testing Guide](backend/tests/mcp/) - Comprehensive MCP test suite
- [RAG Testing Guide](backend/RAG_TESTING_GUIDE.md) - RAG system testing documentation
- [Frontend Integration](docs/FRONTEND_INTEGRATION.md) - React setup and API integration  
- [Hybrid Setup](docs/HYBRID_SETUP.md) - Multi-technology stack configuration

## Current Status

**Branch**: `mcp-server-development-with-react-frontend`  
**Phase**: 2 (MCP Server Implementation) - **COMPLETE**  
**Next**: Phase 3 (React Frontend Integration)  
**Test Coverage**: 1,400+ lines of comprehensive MCP and RAG tests  
**Documentation**: Complete with examples, guides, and API reference  

### Implementation Statistics
- **Total Code**: 2,500+ lines of production Rust code
- **Test Suite**: 1,400+ lines of comprehensive test coverage
- **MCP Tools**: 5 fitness/nutrition analysis tools implemented
- **Transport Layers**: 3 (STDIO, WebSocket, HTTP)
- **Authentication**: JWT + API key system with session management
- **Protocol Compliance**: MCP 2025-06-18 specification