//! Messages API
//!
//! Methods for managing messages.

use std::collections::HashMap;

use crate::error::Result;
use crate::types::Message;
use crate::{client::SlackClient, types::SlackResponse};
use serde::{Deserialize, Serialize};

/// Messages API client
pub struct MessagesApi {
    client: SlackClient,
}

impl MessagesApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// List messages by channel and timestamp
    ///
    /// This method lists messages using multipart/form-data encoding,
    /// encoding each struct field as a separate form part.
    ///
    /// # Arguments
    ///
    /// * `params` - Request parameters that will be encoded as multipart form data
    pub async fn list(&self, params: MessagesListRequest) -> Result<MessagesListResponse> {
        let x = self
            .client
            .post_multipart_direct("messages.list", &params)
            .await?;

        Ok(x)
    }
}

// Request/Response types

#[derive(Debug, Serialize, Default)]
pub struct MessagesListRequest {
    pub message_ids: Vec<ChanTimestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_wide_aware: Option<bool>,
    pub cached_latest_updates: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct ChanTimestamps {
    // Channel ID
    pub channel: String,
    pub timestamps: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessagesListResponse {
    #[serde(default)]
    pub messages_data: HashMap<String, MessagesData>,
    #[serde(flatten)]
    pub base: SlackResponse<()>,
}

#[derive(Debug, Deserialize, Default)]
pub struct MessagesData {
    pub messages: Vec<Message>,

    #[serde(default)]
    pub latest_updates: HashMap<String, String>,

    #[serde(default)]
    pub unchecked_messages: Vec<String>,
}
