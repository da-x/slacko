//! Auth API
//!
//! Methods for testing and managing authentication.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Auth API client
pub struct AuthApi {
    client: SlackClient,
}

impl AuthApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Test authentication and get information about the authenticated user/bot
    pub async fn test(&self) -> Result<AuthTestResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("auth.test", &params).await
    }

    /// Revoke an authentication token
    ///
    /// # Arguments
    ///
    /// * `test` - Test mode (don't actually revoke)
    pub async fn revoke(&self, test: bool) -> Result<AuthRevokeResponse> {
        let params = AuthRevokeRequest { test: Some(test) };

        self.client.post("auth.revoke", &params).await
    }

    /// List teams the authenticated user has access to
    pub async fn teams_list(&self) -> Result<AuthTeamsListResponse> {
        let params = AuthTeamsListRequest {
            cursor: None,
            limit: Some(100),
        };

        self.client.post("auth.teams.list", &params).await
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct AuthTestResponse {
    pub url: String,
    pub team: String,
    pub user: String,
    pub team_id: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enterprise_install: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AuthRevokeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AuthRevokeResponse {
    pub revoked: bool,
}

#[derive(Debug, Serialize)]
pub struct AuthTeamsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AuthTeamsListResponse {
    pub teams: Vec<AuthTeam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::types::ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct AuthTeam {
    pub id: String,
    pub name: String,
}
