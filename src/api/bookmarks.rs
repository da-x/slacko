//! Bookmarks API
//!
//! Methods for managing channel bookmarks.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Bookmarks API client
pub struct BookmarksApi {
    client: SlackClient,
}

impl BookmarksApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Add a bookmark to a channel
    ///
    /// # Arguments
    ///
    /// * `channel_id` - Channel ID
    /// * `title` - Bookmark title
    /// * `link` - Bookmark URL
    /// * `emoji` - Emoji icon (optional)
    pub async fn add(
        &self,
        channel_id: &str,
        title: &str,
        link: &str,
        emoji: Option<&str>,
    ) -> Result<BookmarkAddResponse> {
        let params = BookmarkAddRequest {
            channel_id: channel_id.to_string(),
            title: title.to_string(),
            bookmark_type: "link".to_string(),
            link: link.to_string(),
            emoji: emoji.map(|s| s.to_string()),
            entity_id: None,
        };

        self.client.post("bookmarks.add", &params).await
    }

    /// Edit a bookmark
    ///
    /// # Arguments
    ///
    /// * `bookmark_id` - Bookmark ID
    /// * `channel_id` - Channel ID
    /// * `title` - New title (optional)
    /// * `link` - New URL (optional)
    /// * `emoji` - New emoji (optional)
    pub async fn edit(
        &self,
        bookmark_id: &str,
        channel_id: &str,
        title: Option<&str>,
        link: Option<&str>,
        emoji: Option<&str>,
    ) -> Result<BookmarkEditResponse> {
        let params = BookmarkEditRequest {
            bookmark_id: bookmark_id.to_string(),
            channel_id: channel_id.to_string(),
            title: title.map(|s| s.to_string()),
            link: link.map(|s| s.to_string()),
            emoji: emoji.map(|s| s.to_string()),
        };

        self.client.post("bookmarks.edit", &params).await
    }

    /// List bookmarks in a channel
    ///
    /// # Arguments
    ///
    /// * `channel_id` - Channel ID
    pub async fn list(&self, channel_id: &str) -> Result<BookmarkListResponse> {
        let params = [("channel_id", channel_id)];

        self.client.get("bookmarks.list", &params).await
    }

    /// Remove a bookmark
    ///
    /// # Arguments
    ///
    /// * `bookmark_id` - Bookmark ID
    /// * `channel_id` - Channel ID
    pub async fn remove(
        &self,
        bookmark_id: &str,
        channel_id: &str,
    ) -> Result<BookmarkRemoveResponse> {
        let params = BookmarkRemoveRequest {
            bookmark_id: bookmark_id.to_string(),
            channel_id: channel_id.to_string(),
        };

        self.client.post("bookmarks.remove", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct BookmarkAddRequest {
    pub channel_id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub bookmark_type: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkAddResponse {
    pub bookmark: Bookmark,
}

#[derive(Debug, Serialize)]
pub struct BookmarkEditRequest {
    pub bookmark_id: String,
    pub channel_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkEditResponse {
    pub bookmark: Bookmark,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkListResponse {
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Serialize)]
pub struct BookmarkRemoveRequest {
    pub bookmark_id: String,
    pub channel_id: String,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkRemoveResponse {}

#[derive(Debug, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub channel_id: String,
    pub title: String,
    pub link: String,
    pub emoji: String,
    pub date_created: i64,
    pub date_updated: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}
