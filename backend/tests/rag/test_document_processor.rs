#[cfg(test)]
mod document_processor_tests {
    use fitness_advisor_ai::rag::{DocumentProcessor, DocumentType};
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_process_text_document() {
        let processor = DocumentProcessor::new(500);
        let content = "This is a test document about fitness and nutrition. \
                      Exercise is important for maintaining good health. \
                      Proper nutrition supports athletic performance.";
        
        let doc = processor.process_text(content, "Test Document", "test://source", DocumentType::PlainText)
            .expect("Should process text document");
        
        assert_eq!(doc.title, "Test Document");
        assert_eq!(doc.content, content);
        assert_eq!(doc.source, "test://source");
        assert_eq!(doc.metadata.document_type, DocumentType::PlainText);
        assert!(doc.metadata.word_count > 0);
    }

    #[tokio::test]
    async fn test_chunk_document() {
        let processor = DocumentProcessor::new(100); // Small chunks for testing
        
        let content = "This is the first paragraph about exercise. It contains important information about workout routines and strength training. Regular exercise improves cardiovascular health and builds muscle mass. \

This is the second paragraph about nutrition. Proper nutrition is essential for athletic performance. Eating balanced meals provides the energy needed for workouts. Protein helps with muscle recovery and growth.";

        let doc = processor.process_text(content, "Test Doc", "test://source", DocumentType::PlainText)
            .expect("Should create document");
        
        let chunks = processor.chunk_document(&doc)
            .expect("Should chunk document");
        
        assert!(!chunks.is_empty());
        assert!(chunks.len() >= 2); // Should create multiple chunks
        
        // Verify chunk metadata
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.document_id, doc.id);
            assert_eq!(chunk.chunk_index, i);
            assert!(chunk.metadata.start_char < chunk.metadata.end_char);
            assert!(!chunk.content.is_empty());
        }
    }

    #[tokio::test]
    async fn test_process_html() {
        let processor = DocumentProcessor::new(500);
        let html_content = r#"
            <html>
                <head><title>Fitness Article</title></head>
                <body>
                    <h1>The Benefits of Regular Exercise</h1>
                    <p>Exercise is crucial for maintaining good health and fitness.</p>
                    <p>Regular physical activity can help prevent chronic diseases.</p>
                    <script>console.log('This should be removed');</script>
                    <style>body { color: red; }</style>
                </body>
            </html>
        "#;

        let doc = processor.process_html(html_content, "Fitness Article", "https://example.com/fitness")
            .expect("Should process HTML document");

        assert_eq!(doc.title, "Fitness Article");
        assert_eq!(doc.source, "https://example.com/fitness");
        assert_eq!(doc.metadata.document_type, DocumentType::WebPage);
        
        // Should extract text content and remove scripts/styles
        assert!(doc.content.contains("Benefits of Regular Exercise"));
        assert!(doc.content.contains("crucial for maintaining"));
        assert!(!doc.content.contains("console.log"));
        assert!(!doc.content.contains("color: red"));
    }

    #[tokio::test]
    async fn test_empty_document() {
        let processor = DocumentProcessor::new(500);
        
        let doc = processor.process_text("", "Empty Doc", "test://empty", DocumentType::PlainText)
            .expect("Should handle empty document");
        
        assert_eq!(doc.content, "");
        assert_eq!(doc.metadata.word_count, 0);
        
        let chunks = processor.chunk_document(&doc)
            .expect("Should handle empty document chunking");
        
        // Empty document should create no chunks or one empty chunk
        assert!(chunks.is_empty() || (chunks.len() == 1 && chunks[0].content.is_empty()));
    }

    #[tokio::test]
    async fn test_large_document_chunking() {
        let processor = DocumentProcessor::new(200); // Small chunk size
        
        // Create a large document
        let mut content = String::new();
        for i in 1..=10 {
            content.push_str(&format!(
                "This is paragraph {} about fitness and health. It contains detailed information about exercise routines, nutrition guidelines, and wellness practices that are important for maintaining an active lifestyle. ",
                i
            ));
        }
        
        let doc = processor.process_text(&content, "Large Doc", "test://large", DocumentType::PlainText)
            .expect("Should process large document");
        
        let chunks = processor.chunk_document(&doc)
            .expect("Should chunk large document");
        
        assert!(chunks.len() > 3); // Should create multiple chunks
        
        // Verify chunks don't exceed max size significantly
        for chunk in &chunks {
            assert!(chunk.content.len() <= 300); // Allow some flexibility
        }
        
        // Verify all content is preserved
        let reconstructed: String = chunks.iter().map(|c| &c.content).collect();
        assert!(reconstructed.contains("paragraph 1"));
        assert!(reconstructed.contains("paragraph 10"));
    }

    #[tokio::test]
    async fn test_pdf_processing_mock() {
        // Since we can't easily test actual PDF processing without a real PDF file,
        // we'll test the error handling for non-existent files
        let processor = DocumentProcessor::new(500);
        
        let result = processor.process_pdf("nonexistent.pdf", "Test PDF", "file://test.pdf");
        assert!(result.is_err()); // Should fail for non-existent file
    }

    #[tokio::test]
    async fn test_chunk_metadata_accuracy() {
        let processor = DocumentProcessor::new(50); // Very small chunks
        let content = "First sentence. Second sentence. Third sentence. Fourth sentence.";
        
        let doc = processor.process_text(content, "Test", "test://source", DocumentType::PlainText)
            .expect("Should create document");
        
        let chunks = processor.chunk_document(&doc)
            .expect("Should create chunks");
        
        let mut previous_end = 0;
        for (i, chunk) in chunks.iter().enumerate() {
            // Verify chunk indices are sequential
            assert_eq!(chunk.chunk_index, i);
            
            // Verify character offsets are correct
            assert_eq!(chunk.metadata.start_char, previous_end);
            assert_eq!(chunk.metadata.end_char, previous_end + chunk.content.len());
            
            // Verify content matches position in original
            let expected_content = &content[chunk.metadata.start_char..chunk.metadata.end_char];
            assert_eq!(chunk.content, expected_content);
            
            previous_end = chunk.metadata.end_char;
        }
        
        // Verify all content is covered
        assert_eq!(previous_end, content.len());
    }
}