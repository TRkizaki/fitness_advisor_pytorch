use anyhow::Result;
use text_splitter::TextSplitter;
use pdf_extract::extract_text;
use scraper::{Html, Selector};
use uuid::Uuid;
use crate::rag::types::{Document, DocumentType, DocumentMetadata, Chunk, ChunkMetadata};

pub struct DocumentProcessor {
    text_splitter: TextSplitter<usize>,
    max_chunk_size: usize,
}

impl DocumentProcessor {
    pub fn new(max_chunk_size: usize) -> Self {
        let text_splitter = TextSplitter::new(max_chunk_size);
        
        Self {
            text_splitter,
            max_chunk_size,
        }
    }

    pub fn process_pdf(&self, pdf_path: &str, title: &str, source: &str) -> Result<Document> {
        let content = extract_text(pdf_path)?;
        let word_count = content.split_whitespace().count();

        let document = Document {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content,
            source: source.to_string(),
            metadata: DocumentMetadata {
                document_type: DocumentType::PDF,
                author: None,
                tags: vec![],
                language: "en".to_string(),
                word_count,
            },
            created_at: chrono::Utc::now(),
        };

        Ok(document)
    }

    pub fn process_html(&self, html_content: &str, title: &str, source: &str) -> Result<Document> {
        let document = Html::parse_document(html_content);
        
        // Extract text content from HTML
        let content = self.extract_text_from_html(&document)?;
        let word_count = content.split_whitespace().count();

        let document = Document {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content,
            source: source.to_string(),
            metadata: DocumentMetadata {
                document_type: DocumentType::WebPage,
                author: None,
                tags: vec![],
                language: "en".to_string(),
                word_count,
            },
            created_at: chrono::Utc::now(),
        };

        Ok(document)
    }

    pub fn process_text(&self, text_content: &str, title: &str, source: &str, doc_type: DocumentType) -> Result<Document> {
        let word_count = text_content.split_whitespace().count();

        let document = Document {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content: text_content.to_string(),
            source: source.to_string(),
            metadata: DocumentMetadata {
                document_type: doc_type,
                author: None,
                tags: vec![],
                language: "en".to_string(),
                word_count,
            },
            created_at: chrono::Utc::now(),
        };

        Ok(document)
    }

    pub fn chunk_document(&self, document: &Document) -> Result<Vec<Chunk>> {
        let chunks = self.text_splitter.chunks(&document.content);
        let mut result = Vec::new();
        
        let mut char_offset = 0;
        
        for (index, chunk_text) in chunks.enumerate() {
            let start_char = char_offset;
            let end_char = start_char + chunk_text.len();
            char_offset = end_char;

            let chunk = Chunk {
                id: Uuid::new_v4(),
                document_id: document.id,
                content: chunk_text.to_string(),
                embedding: None, // Will be populated later
                chunk_index: index,
                metadata: ChunkMetadata {
                    start_char,
                    end_char,
                    tokens: None, // Will be populated when creating embeddings
                    semantic_level: "paragraph".to_string(), // Default level
                },
            };

            result.push(chunk);
        }

        Ok(result)
    }

    pub async fn scrape_url(&self, url: &str) -> Result<String> {
        let response = reqwest::get(url).await?;
        let html_content = response.text().await?;
        Ok(html_content)
    }

    fn extract_text_from_html(&self, document: &Html) -> Result<String> {
        // Remove script and style elements
        let script_selector = Selector::parse("script, style").unwrap();
        let mut clean_html = document.html();
        
        for element in document.select(&script_selector) {
            let element_html = element.html();
            clean_html = clean_html.replace(&element_html, "");
        }

        // Parse the cleaned HTML
        let clean_document = Html::parse_document(&clean_html);
        
        // Extract text from common content elements
        let content_selectors = [
            "p", "h1", "h2", "h3", "h4", "h5", "h6", 
            "article", "section", "div", "span", "li", "td"
        ];
        
        let mut text_content = String::new();
        
        for selector_str in &content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in clean_document.select(&selector) {
                    let element_text = element.text().collect::<Vec<_>>().join(" ");
                    if !element_text.trim().is_empty() {
                        text_content.push_str(&element_text);
                        text_content.push('\n');
                    }
                }
            }
        }

        // Clean up the text
        let cleaned_text = text_content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(cleaned_text)
    }
}