//! Stars API
//!
//! Methods for starring and unstarring items.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Stars API client
pub struct StarsApi {
    client: SlackClient,
}

impl StarsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Star a message, file, or channel
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID (for messages)
    /// * `timestamp` - Message timestamp (optional)
    /// * `file` - File ID (optional)
    pub async fn add(
        &self,
        channel: Option<&str>,
        timestamp: Option<&str>,
        file: Option<&str>,
    ) -> Result<StarAddResponse> {
        let params = StarAddRequest {
            channel: channel.map(|s| s.to_string()),
            timestamp: timestamp.map(|s| s.to_string()),
            file: file.map(|s| s.to_string()),
        };

        self.client.post("stars.add", &params).await
    }

    /// Remove a star from a message, file, or channel
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID (for messages)
    /// * `timestamp` - Message timestamp (optional)
    /// * `file` - File ID (optional)
    pub async fn remove(
        &self,
        channel: Option<&str>,
        timestamp: Option<&str>,
        file: Option<&str>,
    ) -> Result<StarRemoveResponse> {
        let params = StarRemoveRequest {
            channel: channel.map(|s| s.to_string()),
            timestamp: timestamp.map(|s| s.to_string()),
            file: file.map(|s| s.to_string()),
        };

        self.client.post("stars.remove", &params).await
    }

    /// List starred items
    pub async fn list(&self) -> Result<StarListResponse> {
        let params = StarListRequest {
            count: Some(100),
            page: None,
        };

        self.client.post("stars.list", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct StarAddRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StarAddResponse {}

#[derive(Debug, Serialize)]
pub struct StarRemoveRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StarRemoveResponse {}

#[derive(Debug, Serialize)]
pub struct StarListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct StarListResponse {
    pub items: Vec<StarredItem>,
    pub paging: StarPaging,
}

#[derive(Debug, Deserialize)]
pub struct StarredItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub channel: Option<String>,
    pub date_create: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct StarPaging {
    pub count: u32,
    pub total: u32,
    pub page: u32,
    pub pages: u32,
}
