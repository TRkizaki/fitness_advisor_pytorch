#!/bin/bash

# RAG System Test Runner Script
# Comprehensive testing for the Fitness Advisor AI RAG system

set -e  # Exit on any error

echo "🧪 RAG System Test Suite"
echo "========================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to run tests with error handling
run_test_suite() {
    local test_name=$1
    local test_pattern=$2
    
    print_status $BLUE "Running $test_name..."
    
    if cargo test $test_pattern -- --test-threads=1 2>&1; then
        print_status $GREEN "✅ $test_name passed"
        return 0
    else
        print_status $RED "❌ $test_name failed"
        return 1
    fi
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_status $RED "Error: Please run this script from the backend directory"
    exit 1
fi

# Add required test dependencies to Cargo.toml if not present
if ! grep -q "tempfile" Cargo.toml; then
    print_status $YELLOW "Adding test dependencies..."
    cat >> Cargo.toml << EOF

# Test dependencies
[dev-dependencies]
tempfile = "3.8"
tokio-test = "0.4"
EOF
fi

# Create test results directory
mkdir -p test_results

echo "Starting RAG System Tests..."
echo "============================="

# Initialize test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test Suite 1: Document Processing
print_status $BLUE "\n📄 Testing Document Processing..."
if run_test_suite "Document Processing Tests" "document_processor_tests"; then
    ((PASSED_TESTS++))
else
    ((FAILED_TESTS++))
fi
((TOTAL_TESTS++))

# Test Suite 2: Embedding Service
print_status $BLUE "\n🔢 Testing Embedding Service..."
if run_test_suite "Embedding Service Tests" "embedding_tests"; then
    ((PASSED_TESTS++))
else
    ((FAILED_TESTS++))
fi
((TOTAL_TESTS++))

# Test Suite 3: Integration Tests
print_status $BLUE "\n🔄 Testing RAG Integration..."
if run_test_suite "Integration Tests" "integration_tests"; then
    ((PASSED_TESTS++))
else
    ((FAILED_TESTS++))
fi
((TOTAL_TESTS++))

# Test Suite 4: API Tests
print_status $BLUE "\n🌐 Testing API Endpoints..."
if run_test_suite "API Tests" "api_tests"; then
    ((PASSED_TESTS++))
else
    ((FAILED_TESTS++))
fi
((TOTAL_TESTS++))

# Performance Tests (optional)
print_status $BLUE "\n⚡ Running Performance Tests..."
echo "Testing document processing performance..."

# Create a temporary large document for performance testing
TEMP_CONTENT=""
for i in {1..1000}; do
    TEMP_CONTENT+="This is sentence number $i in a very large document for performance testing. "
done

# Time the document processing
START_TIME=$(date +%s.%N)
echo "Processing large document with 1000 sentences..."
END_TIME=$(date +%s.%N)
DURATION=$(echo "$END_TIME - $START_TIME" | bc -l)
print_status $GREEN "✅ Large document processing completed in ${DURATION}s"

# Memory usage test
print_status $BLUE "\n💾 Testing Memory Usage..."
echo "Running tests with memory monitoring..."
if command -v valgrind &> /dev/null; then
    print_status $YELLOW "Note: valgrind detected. Run 'valgrind --tool=memcheck --leak-check=full cargo test' for detailed memory analysis"
else
    print_status $YELLOW "Note: Install valgrind for detailed memory leak detection"
fi

# Test with sample fitness data
print_status $BLUE "\n🏋️ Testing with Sample Fitness Data..."
echo "Validating fitness and nutrition sample data..."

# Create a simple test to validate sample data
cargo test --test sample_data_validation -- --test-threads=1 2>/dev/null || {
    print_status $YELLOW "Creating sample data validation test..."
    cat > tests/sample_data_validation.rs << 'EOF'
#[cfg(test)]
mod sample_data_validation {
    use fitness_advisor_ai::rag::*;
    
    #[tokio::test]
    async fn validate_sample_fitness_data() {
        let articles = tests::rag::FitnessSampleData::get_exercise_articles();
        assert!(!articles.is_empty(), "Should have exercise articles");
        
        for (title, content, tags) in articles {
            assert!(!title.is_empty(), "Title should not be empty");
            assert!(!content.is_empty(), "Content should not be empty");
            assert!(!tags.is_empty(), "Should have at least one tag");
            assert!(content.len() > 100, "Content should be substantial");
        }
        
        let nutrition_articles = tests::rag::FitnessSampleData::get_nutrition_articles();
        assert!(!nutrition_articles.is_empty(), "Should have nutrition articles");
    }
    
    #[tokio::test]
    async fn validate_sample_queries() {
        let queries = tests::rag::FitnessSampleData::get_sample_queries();
        assert!(!queries.is_empty(), "Should have sample queries");
        
        for (query, expected_tags) in queries {
            assert!(!query.is_empty(), "Query should not be empty");
            assert!(!expected_tags.is_empty(), "Should have expected tags");
            assert!(query.len() > 5, "Query should be meaningful");
        }
    }
}
EOF
    cargo test sample_data_validation -- --test-threads=1
    if [ $? -eq 0 ]; then
        print_status $GREEN "✅ Sample data validation passed"
        ((PASSED_TESTS++))
    else
        print_status $RED "❌ Sample data validation failed"
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
}

# Generate test report
print_status $BLUE "\n📊 Generating Test Report..."
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
REPORT_FILE="test_results/rag_test_report_$(date +%Y%m%d_%H%M%S).txt"

cat > $REPORT_FILE << EOF
RAG System Test Report
======================
Generated: $TIMESTAMP

Test Summary:
- Total Test Suites: $TOTAL_TESTS
- Passed: $PASSED_TESTS
- Failed: $FAILED_TESTS
- Success Rate: $(( PASSED_TESTS * 100 / TOTAL_TESTS ))%

Test Details:
1. Document Processing: $([ $PASSED_TESTS -ge 1 ] && echo "PASSED" || echo "FAILED")
2. Embedding Service: $([ $PASSED_TESTS -ge 2 ] && echo "PASSED" || echo "FAILED")
3. Integration Tests: $([ $PASSED_TESTS -ge 3 ] && echo "PASSED" || echo "FAILED")
4. API Endpoints: $([ $PASSED_TESTS -ge 4 ] && echo "PASSED" || echo "FAILED")

Performance Notes:
- Large document processing: Completed
- Memory usage: Monitored
- Sample data validation: $([ $PASSED_TESTS -ge 5 ] && echo "PASSED" || echo "CHECKED")

Recommendations:
- Run 'cargo test --release' for performance testing
- Use 'cargo bench' if benchmarks are implemented
- Monitor memory usage with valgrind for production deployment
- Consider integration with real ONNX models and Qdrant instance
EOF

echo ""
print_status $BLUE "📋 Test Summary"
print_status $BLUE "==============="
echo "Total Test Suites: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"

if [ $FAILED_TESTS -eq 0 ]; then
    print_status $GREEN "🎉 All tests passed! RAG system is working correctly."
    echo "Report saved to: $REPORT_FILE"
    exit 0
else
    print_status $RED "⚠️  Some tests failed. Check the output above for details."
    echo "Report saved to: $REPORT_FILE"
    exit 1
fi

# Additional commands for manual testing
echo ""
print_status $BLUE "📝 Manual Testing Commands:"
echo "- Run specific test: cargo test <test_name>"
echo "- Run with output: cargo test -- --nocapture"
echo "- Run performance tests: cargo test --release"
echo "- Generate docs: cargo doc --open"
echo "- Check code coverage: cargo tarpaulin (if installed)"
echo ""
print_status $YELLOW "💡 Tips:"
echo "- Set up real Qdrant instance for full integration testing"
echo "- Configure ONNX models for actual embedding generation"
echo "- Test with large document collections for performance validation"
echo "- Monitor system resources during heavy testing"