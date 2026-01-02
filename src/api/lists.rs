//! Lists API
//!
//! Methods for managing Slack Lists - a structured way to organize
//! information in channels and DMs.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::ResponseMetadata;
use serde::{Deserialize, Serialize};

/// Lists API client
pub struct ListsApi {
    client: SlackClient,
}

impl ListsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    // ========== List Management ==========

    /// Create a new list
    ///
    /// # Arguments
    ///
    /// * `title` - Title of the list
    /// * `description` - Optional description
    /// * `external_id` - Optional external ID for the list
    pub async fn create(
        &self,
        title: &str,
        description: Option<&str>,
        external_id: Option<&str>,
    ) -> Result<ListResponse> {
        let params = CreateListRequest {
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            external_id: external_id.map(|s| s.to_string()),
        };

        self.client.post("lists.create", &params).await
    }

    /// Update an existing list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list to update
    /// * `title` - New title (optional)
    /// * `description` - New description (optional)
    pub async fn update(
        &self,
        list_id: &str,
        title: Option<&str>,
        description: Option<&str>,
    ) -> Result<ListResponse> {
        let params = UpdateListRequest {
            list_id: list_id.to_string(),
            title: title.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
        };

        self.client.post("lists.update", &params).await
    }

    /// Delete a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list to delete
    pub async fn delete(&self, list_id: &str) -> Result<DeleteListResponse> {
        let params = DeleteListRequest {
            list_id: list_id.to_string(),
        };

        self.client.post("lists.delete", &params).await
    }

    // ========== Access Control ==========

    /// Set access permissions for a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `access_level` - Access level (e.g., "org", "team", "private")
    /// * `user_ids` - Optional list of user IDs to grant access
    /// * `team_ids` - Optional list of team IDs to grant access
    pub async fn access_set(
        &self,
        list_id: &str,
        access_level: &str,
        user_ids: Option<&[&str]>,
        team_ids: Option<&[&str]>,
    ) -> Result<AccessSetResponse> {
        let params = AccessSetRequest {
            list_id: list_id.to_string(),
            access_level: access_level.to_string(),
            user_ids: user_ids.map(|ids| ids.iter().map(|s| s.to_string()).collect()),
            team_ids: team_ids.map(|ids| ids.iter().map(|s| s.to_string()).collect()),
        };

        self.client.post("lists.access.set", &params).await
    }

    /// Remove access permissions from a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `user_ids` - Optional list of user IDs to remove access from
    /// * `team_ids` - Optional list of team IDs to remove access from
    pub async fn access_delete(
        &self,
        list_id: &str,
        user_ids: Option<&[&str]>,
        team_ids: Option<&[&str]>,
    ) -> Result<AccessDeleteResponse> {
        let params = AccessDeleteRequest {
            list_id: list_id.to_string(),
            user_ids: user_ids.map(|ids| ids.iter().map(|s| s.to_string()).collect()),
            team_ids: team_ids.map(|ids| ids.iter().map(|s| s.to_string()).collect()),
        };

        self.client.post("lists.access.delete", &params).await
    }

    // ========== List Items ==========

    /// Create a new item in a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `item` - Item data as JSON
    pub async fn items_create(
        &self,
        list_id: &str,
        item: serde_json::Value,
    ) -> Result<ItemResponse> {
        let params = ItemCreateRequest {
            list_id: list_id.to_string(),
            item,
        };

        self.client.post("lists.items.create", &params).await
    }

    /// Update an item in a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `item_id` - ID of the item to update
    /// * `item` - Updated item data as JSON
    pub async fn items_update(
        &self,
        list_id: &str,
        item_id: &str,
        item: serde_json::Value,
    ) -> Result<ItemResponse> {
        let params = ItemUpdateRequest {
            list_id: list_id.to_string(),
            item_id: item_id.to_string(),
            item,
        };

        self.client.post("lists.items.update", &params).await
    }

    /// Delete an item from a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `item_id` - ID of the item to delete
    pub async fn items_delete(&self, list_id: &str, item_id: &str) -> Result<ItemDeleteResponse> {
        let params = ItemDeleteRequest {
            list_id: list_id.to_string(),
            item_id: item_id.to_string(),
        };

        self.client.post("lists.items.delete", &params).await
    }

    /// Delete multiple items from a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `item_ids` - IDs of the items to delete
    pub async fn items_delete_multiple(
        &self,
        list_id: &str,
        item_ids: &[&str],
    ) -> Result<ItemsDeleteMultipleResponse> {
        let params = ItemsDeleteMultipleRequest {
            list_id: list_id.to_string(),
            item_ids: item_ids.iter().map(|s| s.to_string()).collect(),
        };

        self.client
            .post("lists.items.deleteMultiple", &params)
            .await
    }

    /// Get information about an item
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `item_id` - ID of the item
    pub async fn items_info(&self, list_id: &str, item_id: &str) -> Result<ItemResponse> {
        let params = ItemInfoRequest {
            list_id: list_id.to_string(),
            item_id: item_id.to_string(),
        };

        self.client.post("lists.items.info", &params).await
    }

    /// List items in a list
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `cursor` - Pagination cursor
    /// * `limit` - Maximum number of items to return
    pub async fn items_list(
        &self,
        list_id: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ItemsListResponse> {
        let params = ItemsListRequest {
            list_id: list_id.to_string(),
            cursor: cursor.map(|s| s.to_string()),
            limit,
        };

        self.client.post("lists.items.list", &params).await
    }

    // ========== Download/Export ==========

    /// Start a list export
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list to export
    /// * `format` - Export format (e.g., "csv", "json")
    pub async fn download_start(
        &self,
        list_id: &str,
        format: Option<&str>,
    ) -> Result<DownloadStartResponse> {
        let params = DownloadStartRequest {
            list_id: list_id.to_string(),
            format: format.map(|s| s.to_string()),
        };

        self.client.post("lists.download.start", &params).await
    }

    /// Get the download URL for a list export
    ///
    /// # Arguments
    ///
    /// * `list_id` - ID of the list
    /// * `download_id` - ID from download_start response
    pub async fn download_get(
        &self,
        list_id: &str,
        download_id: &str,
    ) -> Result<DownloadGetResponse> {
        let params = DownloadGetRequest {
            list_id: list_id.to_string(),
            download_id: download_id.to_string(),
        };

        self.client.post("lists.download.get", &params).await
    }
}

// ========== Request/Response Types ==========

// List management types

#[derive(Debug, Serialize)]
pub struct CreateListRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateListRequest {
    pub list_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteListRequest {
    pub list_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ListResponse {
    pub list: SlackList,
}

#[derive(Debug, Deserialize)]
pub struct DeleteListResponse {}

#[derive(Debug, Deserialize)]
pub struct SlackList {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub created: Option<i64>,
    #[serde(default)]
    pub updated: Option<i64>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub team_id: Option<String>,
}

// Access control types

#[derive(Debug, Serialize)]
pub struct AccessSetRequest {
    pub list_id: String,
    pub access_level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct AccessSetResponse {}

#[derive(Debug, Serialize)]
pub struct AccessDeleteRequest {
    pub list_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct AccessDeleteResponse {}

// Item types

#[derive(Debug, Serialize)]
pub struct ItemCreateRequest {
    pub list_id: String,
    pub item: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ItemUpdateRequest {
    pub list_id: String,
    pub item_id: String,
    pub item: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ItemDeleteRequest {
    pub list_id: String,
    pub item_id: String,
}

#[derive(Debug, Serialize)]
pub struct ItemsDeleteMultipleRequest {
    pub list_id: String,
    pub item_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ItemInfoRequest {
    pub list_id: String,
    pub item_id: String,
}

#[derive(Debug, Serialize)]
pub struct ItemsListRequest {
    pub list_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ItemResponse {
    pub item: ListItem,
}

#[derive(Debug, Deserialize)]
pub struct ItemDeleteResponse {}

#[derive(Debug, Deserialize)]
pub struct ItemsDeleteMultipleResponse {
    #[serde(default)]
    pub deleted_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ItemsListResponse {
    pub items: Vec<ListItem>,
    #[serde(default)]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct ListItem {
    pub id: String,
    #[serde(default)]
    pub values: Option<serde_json::Value>,
    #[serde(default)]
    pub created: Option<i64>,
    #[serde(default)]
    pub updated: Option<i64>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
}

// Download types

#[derive(Debug, Serialize)]
pub struct DownloadStartRequest {
    pub list_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadStartResponse {
    pub download_id: String,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DownloadGetRequest {
    pub list_id: String,
    pub download_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadGetResponse {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub expires_at: Option<i64>,
}
