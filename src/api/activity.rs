//! Activity API
//!
//! Methods for managing activity feed and notifications.

use crate::error::Result;
use crate::{api::chat::ResponseMetadata, client::SlackClient};
use serde::{Deserialize, Serialize};

/// Activity API client
pub struct ActivityApi {
    client: SlackClient,
}

impl ActivityApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Fetch the activity feed
    ///
    /// This method fetches the activity feed using multipart/form-data encoding,
    /// encoding each struct field as a separate form part.
    ///
    /// # Arguments
    ///
    /// * `params` - Request parameters that will be encoded as multipart form data
    pub async fn feed(&self, params: ActivityFeedRequest) -> Result<ActivityFeedResponse> {
        self.client.post_multipart("activity.feed", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize, Default)]
pub struct ActivityFeedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_types"
    )]
    /// For example: at_user, list_user_mentioned
    pub types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snooze_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_salesforce_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_activity_inbox: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

fn serialize_types<S>(
    types: &Option<Vec<String>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match types {
        Some(types_vec) => serializer.serialize_str(&types_vec.join(",")),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Deserialize)]
pub struct ActivityFeedResponse {
    #[serde(default)]
    pub items: Vec<ActivityItem>,
    #[serde(default)]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityItem {
    #[serde(default)]
    pub is_unread: Option<bool>,
    #[serde(default)]
    pub feed_ts: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub item: Option<ActivityItemData>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityItemData {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub bundle_info: Option<ActivityBundleInfo>,
    #[serde(default)]
    pub message: Option<ActivityMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityBundleInfo {
    #[serde(default)]
    pub payload: Option<ActivityPayload>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityPayload {
    #[serde(default)]
    pub thread_entry: Option<ThreadEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadEntry {
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub latest_ts: Option<String>,
    #[serde(default)]
    pub thread_ts: Option<String>,
    #[serde(default)]
    pub unread_msg_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityMessage {
    #[serde(default)]
    pub ts: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub is_broadcast: Option<bool>,
    #[serde(default)]
    pub author_user_id: Option<String>,
}
