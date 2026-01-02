//! Admin API
//!
//! Methods for Enterprise Grid administration. Requires admin privileges.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::ResponseMetadata;
use serde::{Deserialize, Serialize};

/// Admin API client
pub struct AdminApi {
    client: SlackClient,
}

impl AdminApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Get the Apps sub-API
    pub fn apps(&self) -> AdminAppsApi {
        AdminAppsApi::new(self.client.clone())
    }

    /// Get the Users sub-API
    pub fn users(&self) -> AdminUsersApi {
        AdminUsersApi::new(self.client.clone())
    }

    /// Get the Teams sub-API
    pub fn teams(&self) -> AdminTeamsApi {
        AdminTeamsApi::new(self.client.clone())
    }

    /// Get the Conversations sub-API
    pub fn conversations(&self) -> AdminConversationsApi {
        AdminConversationsApi::new(self.client.clone())
    }
}

/// Admin Apps API
pub struct AdminAppsApi {
    client: SlackClient,
}

impl AdminAppsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Approve an app installation request
    pub async fn approve(
        &self,
        app_id: &str,
        request_id: &str,
    ) -> Result<AdminAppsApproveResponse> {
        let params = AdminAppsApproveRequest {
            app_id: app_id.to_string(),
            request_id: request_id.to_string(),
            team_id: None,
        };

        self.client.post("admin.apps.approve", &params).await
    }

    /// Restrict an app
    pub async fn restrict(
        &self,
        app_id: &str,
        request_id: &str,
    ) -> Result<AdminAppsRestrictResponse> {
        let params = AdminAppsRestrictRequest {
            app_id: app_id.to_string(),
            request_id: request_id.to_string(),
            team_id: None,
        };

        self.client.post("admin.apps.restrict", &params).await
    }
}

/// Admin Users API
pub struct AdminUsersApi {
    client: SlackClient,
}

impl AdminUsersApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Invite a user to a workspace
    pub async fn invite(
        &self,
        channel_ids: &[&str],
        email: &str,
        team_id: &str,
    ) -> Result<AdminUsersInviteResponse> {
        let params = AdminUsersInviteRequest {
            channel_ids: channel_ids.join(","),
            email: email.to_string(),
            team_id: team_id.to_string(),
            custom_message: None,
            real_name: None,
        };

        self.client.post("admin.users.invite", &params).await
    }

    /// Remove a user from a workspace
    pub async fn remove(&self, team_id: &str, user_id: &str) -> Result<AdminUsersRemoveResponse> {
        let params = AdminUsersRemoveRequest {
            team_id: team_id.to_string(),
            user_id: user_id.to_string(),
        };

        self.client.post("admin.users.remove", &params).await
    }

    /// Set a user as a workspace admin
    pub async fn set_admin(
        &self,
        team_id: &str,
        user_id: &str,
    ) -> Result<AdminUsersSetAdminResponse> {
        let params = AdminUsersSetAdminRequest {
            team_id: team_id.to_string(),
            user_id: user_id.to_string(),
        };

        self.client.post("admin.users.setAdmin", &params).await
    }
}

/// Admin Teams API
pub struct AdminTeamsApi {
    client: SlackClient,
}

impl AdminTeamsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Create a new workspace
    pub async fn create(
        &self,
        team_domain: &str,
        team_name: &str,
    ) -> Result<AdminTeamsCreateResponse> {
        let params = AdminTeamsCreateRequest {
            team_domain: team_domain.to_string(),
            team_name: team_name.to_string(),
            team_description: None,
            team_discoverability: None,
        };

        self.client.post("admin.teams.create", &params).await
    }

    /// List all workspaces in an Enterprise Grid
    pub async fn list(&self) -> Result<AdminTeamsListResponse> {
        let params = AdminTeamsListRequest {
            limit: Some(100),
            cursor: None,
        };

        self.client.post("admin.teams.list", &params).await
    }
}

/// Admin Conversations API
pub struct AdminConversationsApi {
    client: SlackClient,
}

impl AdminConversationsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Archive a channel (admin override)
    pub async fn archive(&self, channel_id: &str) -> Result<AdminConversationsArchiveResponse> {
        let params = AdminConversationsArchiveRequest {
            channel_id: channel_id.to_string(),
        };

        self.client
            .post("admin.conversations.archive", &params)
            .await
    }

    /// Delete a channel (admin override)
    pub async fn delete(&self, channel_id: &str) -> Result<AdminConversationsDeleteResponse> {
        let params = AdminConversationsDeleteRequest {
            channel_id: channel_id.to_string(),
        };

        self.client
            .post("admin.conversations.delete", &params)
            .await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct AdminAppsApproveRequest {
    pub app_id: String,
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminAppsApproveResponse {}

#[derive(Debug, Serialize)]
pub struct AdminAppsRestrictRequest {
    pub app_id: String,
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminAppsRestrictResponse {}

#[derive(Debug, Serialize)]
pub struct AdminUsersInviteRequest {
    pub channel_ids: String,
    pub email: String,
    pub team_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminUsersInviteResponse {}

#[derive(Debug, Serialize)]
pub struct AdminUsersRemoveRequest {
    pub team_id: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminUsersRemoveResponse {}

#[derive(Debug, Serialize)]
pub struct AdminUsersSetAdminRequest {
    pub team_id: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminUsersSetAdminResponse {}

#[derive(Debug, Serialize)]
pub struct AdminTeamsCreateRequest {
    pub team_domain: String,
    pub team_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_discoverability: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminTeamsCreateResponse {
    pub team: String,
}

#[derive(Debug, Serialize)]
pub struct AdminTeamsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminTeamsListResponse {
    pub teams: Vec<AdminTeam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct AdminTeam {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct AdminConversationsArchiveRequest {
    pub channel_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminConversationsArchiveResponse {}

#[derive(Debug, Serialize)]
pub struct AdminConversationsDeleteRequest {
    pub channel_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminConversationsDeleteResponse {}
