//! Usergroups API
//!
//! Methods for managing user groups (team channels).

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Usergroups API client
pub struct UsergroupsApi {
    client: SlackClient,
}

impl UsergroupsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Create a user group
    ///
    /// # Arguments
    ///
    /// * `name` - Group name
    /// * `handle` - Group handle (without @)
    /// * `description` - Group description (optional)
    pub async fn create(
        &self,
        name: &str,
        handle: &str,
        description: Option<&str>,
    ) -> Result<UsergroupCreateResponse> {
        let params = UsergroupCreateRequest {
            name: name.to_string(),
            handle: Some(handle.to_string()),
            description: description.map(|s| s.to_string()),
            channels: None,
        };

        self.client.post("usergroups.create", &params).await
    }

    /// Disable a user group
    ///
    /// # Arguments
    ///
    /// * `usergroup` - User group ID
    pub async fn disable(&self, usergroup: &str) -> Result<UsergroupDisableResponse> {
        let params = UsergroupDisableRequest {
            usergroup: usergroup.to_string(),
            include_count: None,
        };

        self.client.post("usergroups.disable", &params).await
    }

    /// Enable a user group
    ///
    /// # Arguments
    ///
    /// * `usergroup` - User group ID
    pub async fn enable(&self, usergroup: &str) -> Result<UsergroupEnableResponse> {
        let params = UsergroupEnableRequest {
            usergroup: usergroup.to_string(),
            include_count: None,
        };

        self.client.post("usergroups.enable", &params).await
    }

    /// List all user groups
    pub async fn list(&self) -> Result<UsergroupListResponse> {
        let params = UsergroupListRequest {
            include_count: Some(true),
            include_disabled: Some(false),
            include_users: Some(false),
        };

        self.client.post("usergroups.list", &params).await
    }

    /// Update a user group
    ///
    /// # Arguments
    ///
    /// * `usergroup` - User group ID
    /// * `name` - New name (optional)
    /// * `handle` - New handle (optional)
    /// * `description` - New description (optional)
    pub async fn update(
        &self,
        usergroup: &str,
        name: Option<&str>,
        handle: Option<&str>,
        description: Option<&str>,
    ) -> Result<UsergroupUpdateResponse> {
        let params = UsergroupUpdateRequest {
            usergroup: usergroup.to_string(),
            name: name.map(|s| s.to_string()),
            handle: handle.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            channels: None,
        };

        self.client.post("usergroups.update", &params).await
    }

    /// List users in a user group
    ///
    /// # Arguments
    ///
    /// * `usergroup` - User group ID
    pub async fn users_list(&self, usergroup: &str) -> Result<UsergroupUsersListResponse> {
        let params = UsergroupUsersListRequest {
            usergroup: usergroup.to_string(),
            include_disabled: None,
        };

        self.client.post("usergroups.users.list", &params).await
    }

    /// Update users in a user group
    ///
    /// # Arguments
    ///
    /// * `usergroup` - User group ID
    /// * `users` - User IDs to add
    pub async fn users_update(
        &self,
        usergroup: &str,
        users: &[&str],
    ) -> Result<UsergroupUsersUpdateResponse> {
        let params = UsergroupUsersUpdateRequest {
            usergroup: usergroup.to_string(),
            users: users.join(","),
            include_count: None,
        };

        self.client.post("usergroups.users.update", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct UsergroupCreateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupCreateResponse {
    pub usergroup: Usergroup,
}

#[derive(Debug, Serialize)]
pub struct UsergroupDisableRequest {
    pub usergroup: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupDisableResponse {
    pub usergroup: Usergroup,
}

#[derive(Debug, Serialize)]
pub struct UsergroupEnableRequest {
    pub usergroup: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupEnableResponse {
    pub usergroup: Usergroup,
}

#[derive(Debug, Serialize)]
pub struct UsergroupListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupListResponse {
    pub usergroups: Vec<Usergroup>,
}

#[derive(Debug, Serialize)]
pub struct UsergroupUpdateRequest {
    pub usergroup: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupUpdateResponse {
    pub usergroup: Usergroup,
}

#[derive(Debug, Serialize)]
pub struct UsergroupUsersListRequest {
    pub usergroup: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_disabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupUsersListResponse {
    pub users: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UsergroupUsersUpdateRequest {
    pub usergroup: String,
    pub users: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupUsersUpdateResponse {
    pub usergroup: Usergroup,
}

#[derive(Debug, Deserialize)]
pub struct Usergroup {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub description: String,
    pub handle: String,
    pub is_external: bool,
    pub date_create: i64,
    pub date_update: i64,
    pub date_delete: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_type: Option<String>,
    pub created_by: String,
    pub updated_by: String,
    pub deleted_by: String,
    pub prefs: UsergroupPrefs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UsergroupPrefs {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}
