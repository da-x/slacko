//! Search API
//!
//! Methods for searching messages and files.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::{File, Message};
use serde::{Deserialize, Serialize};

/// Search API client
pub struct SearchApi {
    client: SlackClient,
}

impl SearchApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Search for messages
    ///
    /// # Arguments
    ///
    /// * `query` - Search query
    pub async fn messages(&self, query: &str) -> Result<SearchMessagesResponse> {
        let params = [("query", query), ("count", "20")];
        self.client.get("search.messages", &params).await
    }

    /// Search for messages with custom parameters
    pub async fn messages_with_options(
        &self,
        params: SearchRequest,
    ) -> Result<SearchMessagesResponse> {
        let mut query_params = vec![("query", params.query.as_str())];

        let count_str = params.count.map(|c| c.to_string());
        if let Some(ref c) = count_str {
            query_params.push(("count", c));
        }

        let page_str = params.page.map(|p| p.to_string());
        if let Some(ref p) = page_str {
            query_params.push(("page", p));
        }

        if let Some(ref s) = params.sort {
            query_params.push(("sort", s));
        }

        if let Some(ref sd) = params.sort_dir {
            query_params.push(("sort_dir", sd));
        }

        self.client.get("search.messages", &query_params).await
    }

    /// Search for files
    ///
    /// # Arguments
    ///
    /// * `query` - Search query
    pub async fn files(&self, query: &str) -> Result<SearchFilesResponse> {
        let params = [("query", query), ("count", "20")];
        self.client.get("search.files", &params).await
    }

    /// Search for files with custom parameters
    pub async fn files_with_options(&self, params: SearchRequest) -> Result<SearchFilesResponse> {
        let mut query_params = vec![("query", params.query.as_str())];

        let count_str = params.count.map(|c| c.to_string());
        if let Some(ref c) = count_str {
            query_params.push(("count", c));
        }

        let page_str = params.page.map(|p| p.to_string());
        if let Some(ref p) = page_str {
            query_params.push(("page", p));
        }

        if let Some(ref s) = params.sort {
            query_params.push(("sort", s));
        }

        if let Some(ref sd) = params.sort_dir {
            query_params.push(("sort_dir", sd));
        }

        self.client.get("search.files", &query_params).await
    }

    /// Search both messages and files
    ///
    /// # Arguments
    ///
    /// * `query` - Search query
    pub async fn all(&self, query: &str) -> Result<SearchAllResponse> {
        let params = [("query", query), ("count", "20")];
        self.client.get("search.all", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchMessagesResponse {
    pub messages: SearchMessagesResult,
}

#[derive(Debug, Deserialize)]
pub struct SearchMessagesResult {
    pub total: u32,
    pub matches: Vec<Message>,
    pub pagination: SearchPagination,
}

#[derive(Debug, Deserialize)]
pub struct SearchFilesResponse {
    pub files: SearchFilesResult,
}

#[derive(Debug, Deserialize)]
pub struct SearchFilesResult {
    pub total: u32,
    pub matches: Vec<File>,
    pub pagination: SearchPagination,
}

#[derive(Debug, Deserialize)]
pub struct SearchAllResponse {
    pub messages: SearchMessagesResult,
    pub files: SearchFilesResult,
}

#[derive(Debug, Deserialize)]
pub struct SearchPagination {
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub page_count: u32,
}
