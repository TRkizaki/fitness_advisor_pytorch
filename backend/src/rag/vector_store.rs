use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    CreateCollection, Distance, VectorParams, PointStruct, SearchPoints,
    Filter, FieldCondition, Match, Value as QdrantValue
};
use uuid::Uuid;
use std::collections::HashMap;
use crate::rag::types::{Chunk, SearchQuery, SearchFilters, DocumentType};

pub struct VectorStore {
    client: QdrantClient,
    collection_name: String,
    dimension: u64,
}

impl VectorStore {
    pub async fn new(url: &str, collection_name: &str, dimension: u64) -> Result<Self> {
        let client = QdrantClient::from_url(url).build()?;
        
        let store = Self {
            client,
            collection_name: collection_name.to_string(),
            dimension,
        };

        // Create collection if it doesn't exist
        store.create_collection_if_not_exists().await?;

        Ok(store)
    }

    async fn create_collection_if_not_exists(&self) -> Result<()> {
        // Check if collection exists
        let collections = self.client.list_collections().await?;
        
        let collection_exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.collection_name);

        if !collection_exists {
            self.client
                .create_collection(&CreateCollection {
                    collection_name: self.collection_name.clone(),
                    vectors_config: Some(VectorParams {
                        size: self.dimension,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    }.into()),
                    ..Default::default()
                })
                .await?;
        }

        Ok(())
    }

    pub async fn upsert_chunks(&self, chunks: Vec<Chunk>) -> Result<()> {
        let points: Vec<PointStruct> = chunks
            .into_iter()
            .filter_map(|chunk| {
                // Only process chunks that have embeddings
                chunk.embedding.map(|embedding| {
                    let mut payload = HashMap::new();
                    payload.insert("content".to_string(), QdrantValue::from(chunk.content.clone()));
                    payload.insert("document_id".to_string(), QdrantValue::from(chunk.document_id.to_string()));
                    payload.insert("chunk_index".to_string(), QdrantValue::from(chunk.chunk_index as i64));
                    payload.insert("start_char".to_string(), QdrantValue::from(chunk.metadata.start_char as i64));
                    payload.insert("end_char".to_string(), QdrantValue::from(chunk.metadata.end_char as i64));
                    payload.insert("semantic_level".to_string(), QdrantValue::from(chunk.metadata.semantic_level.clone()));

                    PointStruct::new(
                        chunk.id.to_string(),
                        embedding,
                        payload,
                    )
                })
            })
            .collect();

        if !points.is_empty() {
            self.client
                .upsert_points_blocking(&self.collection_name, points, None)
                .await?;
        }

        Ok(())
    }

    pub async fn search(&self, query_embedding: Vec<f32>, query: &SearchQuery) -> Result<Vec<ScoredPoint>> {
        let limit = query.limit.unwrap_or(10) as u64;
        let score_threshold = query.threshold;

        // Build filter from search query
        let filter = self.build_filter(query.filters.as_ref());

        let search_points = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query_embedding,
            limit,
            score_threshold,
            filter,
            with_payload: Some(true.into()),
            ..Default::default()
        };

        let search_result = self.client.search_points(&search_points).await?;
        Ok(search_result.result)
    }

    fn build_filter(&self, filters: Option<&SearchFilters>) -> Option<Filter> {
        let Some(filters) = filters else {
            return None;
        };

        let mut conditions = Vec::new();

        // Filter by document types
        if let Some(doc_types) = &filters.document_types {
            let type_strings: Vec<String> = doc_types
                .iter()
                .map(|dt| self.document_type_to_string(dt))
                .collect();

            if !type_strings.is_empty() {
                conditions.push(FieldCondition {
                    key: "document_type".to_string(),
                    match_: Some(Match {
                        match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keywords(
                            qdrant_client::qdrant::MatchKeywords {
                                keywords: type_strings,
                            }
                        )),
                    }),
                    ..Default::default()
                }.into());
            }
        }

        // Filter by tags
        if let Some(tags) = &filters.tags {
            if !tags.is_empty() {
                conditions.push(FieldCondition {
                    key: "tags".to_string(),
                    match_: Some(Match {
                        match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keywords(
                            qdrant_client::qdrant::MatchKeywords {
                                keywords: tags.clone(),
                            }
                        )),
                    }),
                    ..Default::default()
                }.into());
            }
        }

        // Add date range filter if provided
        if let Some(date_range) = &filters.date_range {
            let start_timestamp = date_range.start.timestamp() as f64;
            let end_timestamp = date_range.end.timestamp() as f64;

            conditions.push(FieldCondition {
                key: "created_at".to_string(),
                range: Some(qdrant_client::qdrant::Range {
                    gte: Some(start_timestamp),
                    lte: Some(end_timestamp),
                    ..Default::default()
                }),
                ..Default::default()
            }.into());
        }

        if conditions.is_empty() {
            None
        } else {
            Some(Filter {
                must: conditions,
                ..Default::default()
            })
        }
    }

    fn document_type_to_string(&self, doc_type: &DocumentType) -> String {
        match doc_type {
            DocumentType::ResearchPaper => "research_paper".to_string(),
            DocumentType::WebPage => "web_page".to_string(),
            DocumentType::PDF => "pdf".to_string(),
            DocumentType::PlainText => "plain_text".to_string(),
            DocumentType::Exercise => "exercise".to_string(),
            DocumentType::Nutrition => "nutrition".to_string(),
        }
    }

    pub async fn delete_document(&self, document_id: Uuid) -> Result<()> {
        let filter = Filter {
            must: vec![FieldCondition {
                key: "document_id".to_string(),
                match_: Some(Match {
                    match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keyword(
                        document_id.to_string()
                    )),
                }),
                ..Default::default()
            }.into()],
            ..Default::default()
        };

        self.client
            .delete_points(&self.collection_name, &filter.into(), None)
            .await?;

        Ok(())
    }

    pub async fn get_collection_info(&self) -> Result<qdrant_client::qdrant::CollectionInfo> {
        let info = self.client.collection_info(&self.collection_name).await?;
        Ok(info.result.expect("Collection should exist"))
    }
}