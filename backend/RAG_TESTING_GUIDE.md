# RAG System Testing Guide

## Overview
This document provides comprehensive testing scenarios for the Fitness Advisor AI RAG (Retrieval-Augmented Generation) system. The testing suite validates all components from document processing to API endpoints.

## Test Structure

### 1. Unit Tests (`backend/tests/rag/`)

#### Document Processing Tests (`test_document_processor.rs`)
Tests the core document processing functionality:
- ✅ Text document processing and metadata extraction
- ✅ HTML content extraction and cleaning
- ✅ Document chunking with proper metadata
- ✅ Chunk boundary accuracy and content preservation
- ✅ Large document handling and performance
- ✅ Empty document edge cases
- ✅ Special character handling

#### Embedding Tests (`test_embeddings.rs`)
Tests embedding generation and similarity calculations:
- ✅ Single text embedding generation
- ✅ Batch embedding processing
- ✅ Cosine similarity calculations
- ✅ Embedding normalization and consistency
- ✅ Fitness domain-specific content processing
- ✅ Edge cases (empty text, special characters)

### 2. Integration Tests (`test_integration.rs`)
Tests the complete RAG pipeline:
- ✅ Full document-to-search pipeline
- ✅ Multi-document knowledge base creation
- ✅ Semantic search with relevance ranking
- ✅ Search threshold filtering
- ✅ Query-document matching accuracy

### 3. API Tests (`test_api.rs`)
Tests RESTful API endpoints:
- ✅ Document creation (text and URL)
- ✅ Document retrieval and listing
- ✅ Document deletion
- ✅ Semantic search endpoint
- ✅ RAG query endpoint
- ✅ Statistics and health checks
- ✅ Error handling and validation

## Sample Data

### Fitness Content (`sample_data.rs`)
Comprehensive fitness and nutrition articles covering:

**Exercise Topics:**
- Cardiovascular exercise science and guidelines
- Strength training fundamentals and progressive overload
- High-intensity interval training (HIIT)
- Exercise recovery and sleep importance
- Flexibility and mobility training

**Nutrition Topics:**
- Macronutrients for athletic performance
- Hydration and electrolyte balance
- Pre and post-workout nutrition timing
- Evidence-based supplements
- Weight management and body composition

**Sample Queries:**
- "How much cardio should I do per week?"
- "What are the benefits of strength training?"
- "How do I build muscle effectively?"
- "What should I eat before a workout?"
- "Should I take creatine supplements?"
- And more...

## Running Tests

### Quick Test Run
```bash
cd backend
cargo test --package fitness_advisor_ai --test rag
```

### Comprehensive Test Suite
```bash
cd backend
./run_rag_tests.sh
```

### Individual Test Suites
```bash
# Document processing only
cargo test document_processor_tests

# Embedding tests only
cargo test embedding_tests

# Integration tests only
cargo test integration_tests

# API tests only
cargo test api_tests
```

### Performance Testing
```bash
# Run with release optimizations
cargo test --release

# Run with timing information
cargo test -- --nocapture

# Memory leak detection (requires valgrind)
valgrind --tool=memcheck --leak-check=full cargo test
```

## Test Scenarios Covered

### 🟢 Document Processing Scenarios
1. **Basic Text Processing**: Standard fitness articles
2. **HTML Content Extraction**: Web pages with mixed content
3. **Large Document Handling**: 1000+ sentence documents
4. **Empty Content**: Edge case handling
5. **Special Characters**: Unicode and formatting characters
6. **Chunking Accuracy**: Proper boundary detection and metadata

### 🟢 Embedding Scenarios
1. **Single Text Embedding**: Individual sentence processing
2. **Batch Processing**: Multiple documents simultaneously
3. **Similarity Calculations**: Related vs unrelated content
4. **Fitness Domain Content**: Specialized terminology
5. **Consistency Testing**: Deterministic embedding generation
6. **Performance Testing**: Large text processing

### 🟢 Integration Scenarios
1. **End-to-End Pipeline**: Document → Chunks → Embeddings → Search
2. **Multi-Document Search**: Knowledge base with multiple sources
3. **Query-Document Matching**: Semantic relevance validation
4. **Threshold Filtering**: Precision vs recall optimization
5. **Ranking Accuracy**: Result ordering by relevance

### 🟢 API Scenarios
1. **CRUD Operations**: Create, read, update, delete documents
2. **Search Functionality**: Semantic search with parameters
3. **RAG Queries**: Question answering with source attribution
4. **Error Handling**: Invalid inputs and edge cases
5. **Data Validation**: Request format and content validation

## Expected Performance Benchmarks

### Processing Speed
- **Document Chunking**: < 100ms for 1000-word articles
- **Embedding Generation**: < 500ms per chunk (mock implementation)
- **Search Operations**: < 200ms for 10-document knowledge base
- **API Response**: < 1s for complete RAG pipeline

### Accuracy Metrics
- **Chunking Accuracy**: 100% content preservation
- **Search Relevance**: Top result should contain query keywords
- **Embedding Consistency**: Identical inputs produce identical outputs
- **API Reliability**: All endpoints return expected formats

## Mock vs Real Components

### Current Test Implementation (Mocks)
- **Embedding Service**: Deterministic hash-based embeddings
- **Vector Store**: In-memory similarity calculations
- **LLM Service**: Rule-based response generation
- **Knowledge Base**: Mock document storage

### Production Setup Requirements
- **ONNX Runtime**: Real embedding models
- **Qdrant Instance**: Actual vector database
- **External LLM**: API integration (OpenAI, Anthropic, etc.)
- **Persistent Storage**: Document and metadata persistence

## Troubleshooting

### Common Test Failures
1. **Dependency Issues**: Ensure all test dependencies are installed
2. **Async Runtime**: Tests require tokio runtime
3. **File System**: Temporary file creation permissions
4. **Memory Limits**: Large document processing tests

### Performance Issues
1. **Slow Tests**: Use `--release` flag for optimized builds
2. **Memory Usage**: Monitor with system tools or valgrind
3. **Timeout Issues**: Increase test timeouts for slower systems

### Debug Commands
```bash
# Verbose test output
cargo test -- --nocapture

# Run specific test with debugging
RUST_LOG=debug cargo test <test_name>

# Show test execution time
cargo test -- --test-threads=1 --nocapture
```

## Extending Tests

### Adding New Test Cases
1. Create new test functions in appropriate modules
2. Use descriptive test names: `test_<functionality>_<scenario>`
3. Include both positive and negative test cases
4. Test edge cases and error conditions

### Custom Sample Data
1. Add new content to `sample_data.rs`
2. Include diverse topics and query types
3. Maintain realistic fitness/nutrition focus
4. Provide expected tags/keywords for validation

### Performance Benchmarks
1. Use `criterion` crate for detailed benchmarks
2. Test with various document sizes
3. Monitor memory usage patterns
4. Compare mock vs real component performance

## Integration with CI/CD

### GitHub Actions Example
```yaml
name: RAG Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run RAG Tests
      run: |
        cd backend
        chmod +x run_rag_tests.sh
        ./run_rag_tests.sh
```

This comprehensive testing suite ensures the RAG system works correctly across all components and scenarios, providing confidence for production deployment.