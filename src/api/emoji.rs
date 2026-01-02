//! Emoji API
//!
//! Methods for listing and managing custom emoji.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Emoji API client
pub struct EmojiApi {
    client: SlackClient,
}

impl EmojiApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// List all custom emoji for a team
    pub async fn list(&self) -> Result<EmojiListResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("emoji.list", &params).await
    }

    /// Add a custom emoji
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the emoji (without colons)
    /// * `url` - URL of the emoji image
    ///
    /// # Note
    ///
    /// This is an admin API that requires admin.teams:write scope
    /// or workspace admin privileges.
    pub async fn add(&self, name: &str, url: &str) -> Result<EmojiAddResponse> {
        let params = EmojiAddRequest {
            name: name.to_string(),
            url: url.to_string(),
        };

        self.client.post("admin.emoji.add", &params).await
    }

    /// Add an emoji alias
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the new alias (without colons)
    /// * `alias_for` - Name of the existing emoji to alias
    pub async fn add_alias(&self, name: &str, alias_for: &str) -> Result<EmojiAddResponse> {
        let params = EmojiAddAliasRequest {
            name: name.to_string(),
            alias_for: alias_for.to_string(),
        };

        self.client.post("admin.emoji.addAlias", &params).await
    }

    /// Remove a custom emoji
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the emoji to remove (without colons)
    ///
    /// # Note
    ///
    /// This is an admin API that requires admin.teams:write scope
    /// or workspace admin privileges.
    pub async fn remove(&self, name: &str) -> Result<EmojiRemoveResponse> {
        let params = EmojiRemoveRequest {
            name: name.to_string(),
        };

        self.client.post("admin.emoji.remove", &params).await
    }

    /// Rename a custom emoji
    ///
    /// # Arguments
    ///
    /// * `name` - Current name of the emoji
    /// * `new_name` - New name for the emoji
    pub async fn rename(&self, name: &str, new_name: &str) -> Result<EmojiRenameResponse> {
        let params = EmojiRenameRequest {
            name: name.to_string(),
            new_name: new_name.to_string(),
        };

        self.client.post("admin.emoji.rename", &params).await
    }

    /// List custom emoji with pagination (admin API)
    pub async fn admin_list(&self) -> Result<EmojiAdminListResponse> {
        let params = EmojiAdminListRequest {
            cursor: None,
            limit: Some(100),
        };

        self.client.post("admin.emoji.list", &params).await
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct EmojiListResponse {
    pub emoji: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ts: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmojiAddRequest {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct EmojiAddResponse {}

#[derive(Debug, Serialize)]
pub struct EmojiAddAliasRequest {
    pub name: String,
    pub alias_for: String,
}

#[derive(Debug, Serialize)]
pub struct EmojiRemoveRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EmojiRemoveResponse {}

#[derive(Debug, Serialize)]
pub struct EmojiRenameRequest {
    pub name: String,
    pub new_name: String,
}

#[derive(Debug, Deserialize)]
pub struct EmojiRenameResponse {}

#[derive(Debug, Serialize)]
pub struct EmojiAdminListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct EmojiAdminListResponse {
    pub emoji: Vec<AdminEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::types::ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct AdminEmoji {
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_by: Option<String>,
}
