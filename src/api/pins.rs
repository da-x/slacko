//! Pins API
//!
//! Methods for pinning and unpinning messages and files.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Pins API client
pub struct PinsApi {
    client: SlackClient,
}

impl PinsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Pin a message to a channel
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `timestamp` - Message timestamp
    pub async fn add(&self, channel: &str, timestamp: &str) -> Result<PinAddResponse> {
        let params = PinAddRequest {
            channel: channel.to_string(),
            timestamp: timestamp.to_string(),
        };

        self.client.post("pins.add", &params).await
    }

    /// Unpin a message from a channel
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `timestamp` - Message timestamp
    pub async fn remove(&self, channel: &str, timestamp: &str) -> Result<PinRemoveResponse> {
        let params = PinRemoveRequest {
            channel: channel.to_string(),
            timestamp: timestamp.to_string(),
        };

        self.client.post("pins.remove", &params).await
    }

    /// List pinned items in a channel
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn list(&self, channel: &str) -> Result<PinListResponse> {
        let params = [("channel", channel)];

        self.client.get("pins.list", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct PinAddRequest {
    pub channel: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct PinAddResponse {}

#[derive(Debug, Serialize)]
pub struct PinRemoveRequest {
    pub channel: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct PinRemoveResponse {}

#[derive(Debug, Deserialize)]
pub struct PinListResponse {
    pub items: Vec<PinnedItem>,
}

#[derive(Debug, Deserialize)]
pub struct PinnedItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub channel: String,
    pub created: i64,
    pub created_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
}
