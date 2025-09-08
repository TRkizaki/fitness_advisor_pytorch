# Fitness Advisor AI

A comprehensive AI-powered fitness and nutrition advisory system built with Rust backend, React frontend, and advanced RAG (Retrieval-Augmented Generation) capabilities.

## 🏗️ Architecture Overview

### Current Branch: `rag-system-development`

This branch implements the **Phase 1: RAG Knowledge System** with a complete retrieval-augmented generation pipeline for fitness and nutrition advice.

## 🚀 Features

### ✅ **RAG Knowledge System (Phase 1)**
- **Vector Embeddings**: ONNX Runtime + HuggingFace tokenizers
- **Semantic Search**: Qdrant vector database integration
- **Document Processing**: PDF extraction, web scraping, intelligent text chunking
- **Knowledge Base**: Multi-format document management and storage
- **LLM Integration**: Expandable response generation system
- **RESTful API**: Complete API endpoints for frontend integration

### ✅ **Frontend Integration**
- **React UI**: Modern responsive interface with TypeScript
- **Component Library**: Comprehensive UI components with Tailwind CSS
- **API Client**: Type-safe API integration layer
- **Real-time Features**: WebSocket support for live updates

### ✅ **Backend Infrastructure** 
- **Rust Core**: High-performance, memory-safe backend
- **Menu Optimization**: Genetic algorithms for meal planning
- **ML Analytics**: Motion analysis and pattern recognition
- **Database Layer**: SQLite with SQLx integration
- **Configuration Management**: TOML-based settings

## 🏛️ System Architecture

```
fitness_advisor_ai/
├── backend/                 # Rust backend services
│   ├── src/
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
│   └── tests/              # Comprehensive test suite
├── frontend/               # React frontend application
├── ml-services/           # Python ML processing services
└── docs/                  # Documentation and guides
```

## 🧪 Testing Infrastructure

### Comprehensive Test Coverage
- **Unit Tests**: Individual component validation
- **Integration Tests**: Full RAG pipeline testing
- **API Tests**: RESTful endpoint validation
- **Sample Data**: Rich fitness/nutrition content for testing

### Test Execution
```bash
# Quick test run
cd backend && cargo test

# Full test suite with reporting
cd backend && ./run_rag_tests.sh

# Specific test categories
cargo test document_processor_tests
cargo test integration_tests
cargo test api_tests
```

## 📚 RAG System Capabilities

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

## 🔧 API Endpoints

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

### Example Usage
```bash
# Add fitness content
curl -X POST http://localhost:8080/documents \
  -H "Content-Type: application/json" \
  -d '{
    "title": "HIIT Training Guide",
    "content": "High-intensity interval training improves cardiovascular fitness...",
    "source": "fitness-guide.pdf",
    "tags": ["hiit", "cardio", "training"]
  }'

# Search for information
curl -X POST http://localhost:8080/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "How much cardio should I do weekly?",
    "limit": 5,
    "threshold": 0.7
  }'

# Ask RAG question
curl -X POST http://localhost:8080/query \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What are the benefits of strength training?",
    "max_sources": 3
  }'
```

## 🛠️ Tech Stack

### Backend
- **Language**: Rust 2021 Edition
- **Web Framework**: Axum with Tower middleware
- **Database**: SQLite with SQLx
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

### Dependencies (Hybrid Approach)
```toml
# Core embedding infrastructure
ort = "1.16"                   # ONNX Runtime
hf-hub = "0.3"                 # Hugging Face Hub
tokenizers = "0.19"            # Text tokenization
qdrant-client = "1.14.0"       # Vector database
serde_derive = "1.0"           # Serialization

# Document processing
text-splitter = "0.13"         # Smart text chunking
pdf-extract = "0.7"            # PDF text extraction
scraper = "0.20"               # Web content scraping
```

## 🚦 Getting Started

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

## 📈 Development Roadmap

### ✅ Phase 1: RAG Knowledge System (COMPLETED)
- Vector embeddings and semantic search
- Document processing pipeline  
- Knowledge base management
- REST API implementation
- Comprehensive testing suite

### 🚧 Phase 2: MCP Server Implementation (IN PROGRESS)
- Model Context Protocol server setup
- External service integrations
- Standardized tool interfaces
- Enhanced middleware layer

### 📋 Phase 3: Advanced Features (PLANNED)
- Micronutrient interaction analysis
- Seasonal optimization algorithms
- Strava/MyFitnessPal integrations  
- Production security and monitoring

### 📋 Phase 4: Production Deployment (PLANNED)
- Containerization and orchestration
- Performance optimization
- Scalability enhancements
- Real-world user testing

## 🤝 Contributing

### Development Guidelines
- Follow Rust best practices and idioms
- Maintain comprehensive test coverage
- Use semantic commit messages
- Update documentation for new features

### Testing Requirements
- Unit tests for all new components
- Integration tests for RAG pipeline
- API tests for all endpoints
- Performance benchmarks for optimizations

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🔗 Related Documentation

- [RAG Testing Guide](backend/RAG_TESTING_GUIDE.md) - Comprehensive testing documentation
- [Frontend Integration](docs/FRONTEND_INTEGRATION.md) - React setup and API integration  
- [Hybrid Setup](docs/HYBRID_SETUP.md) - Multi-technology stack configuration

## 🎯 Current Status

**Branch**: `rag-system-development`  
**Phase**: 1 (RAG Knowledge System) - ✅ **COMPLETE**  
**Next**: Phase 2 (MCP Server Implementation)  
**Test Coverage**: Comprehensive unit, integration, and API tests  
**Documentation**: Complete with examples and guides  

---

*Built with ❤️ using Rust, React, and modern AI technologies*