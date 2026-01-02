//! Reactions API
//!
//! Methods for adding and removing emoji reactions.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::Message;
use serde::{Deserialize, Serialize};

/// Reactions API client
pub struct ReactionsApi {
    client: SlackClient,
}

impl ReactionsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Add a reaction to a message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `timestamp` - Message timestamp
    /// * `name` - Emoji name (without colons)
    pub async fn add(
        &self,
        channel: &str,
        timestamp: &str,
        name: &str,
    ) -> Result<ReactionAddResponse> {
        let params = ReactionAddRequest {
            channel: channel.to_string(),
            timestamp: timestamp.to_string(),
            name: name.to_string(),
        };

        self.client.post("reactions.add", &params).await
    }

    /// Remove a reaction from a message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `timestamp` - Message timestamp
    /// * `name` - Emoji name (without colons)
    pub async fn remove(
        &self,
        channel: &str,
        timestamp: &str,
        name: &str,
    ) -> Result<ReactionRemoveResponse> {
        let params = ReactionRemoveRequest {
            channel: channel.to_string(),
            timestamp: timestamp.to_string(),
            name: name.to_string(),
        };

        self.client.post("reactions.remove", &params).await
    }

    /// Get reactions for a message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `timestamp` - Message timestamp
    pub async fn get(&self, channel: &str, timestamp: &str) -> Result<ReactionGetResponse> {
        let params = [("channel", channel), ("timestamp", timestamp)];

        self.client.get("reactions.get", &params).await
    }

    /// List reactions made by the authenticated user
    pub async fn list(&self) -> Result<ReactionListResponse> {
        let params = ReactionListRequest {
            user: None,
            count: Some(100),
            page: None,
        };

        self.client.post("reactions.list", &params).await
    }

    /// List reactions with custom parameters
    pub async fn list_with_options(
        &self,
        params: ReactionListRequest,
    ) -> Result<ReactionListResponse> {
        self.client.post("reactions.list", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct ReactionAddRequest {
    pub channel: String,
    pub timestamp: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ReactionAddResponse {}

#[derive(Debug, Serialize)]
pub struct ReactionRemoveRequest {
    pub channel: String,
    pub timestamp: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ReactionRemoveResponse {}

#[derive(Debug, Deserialize)]
pub struct ReactionGetResponse {
    #[serde(rename = "type")]
    pub item_type: String,
    pub message: Message,
}

#[derive(Debug, Serialize)]
pub struct ReactionListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ReactionListResponse {
    pub items: Vec<ReactionItem>,
}

#[derive(Debug, Deserialize)]
pub struct ReactionItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub channel: String,
    pub message: Message,
}
