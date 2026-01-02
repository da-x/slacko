//! Team API
//!
//! Methods for retrieving team/workspace information.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::Team;
use serde::{Deserialize, Serialize};

/// Team API client
pub struct TeamApi {
    client: SlackClient,
}

impl TeamApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Get information about the team/workspace
    pub async fn info(&self) -> Result<TeamInfoResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("team.info", &params).await
    }

    /// Get the team's billable information
    pub async fn billable_info(&self) -> Result<TeamBillableInfoResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("team.billableInfo", &params).await
    }

    /// Get the team's access logs
    pub async fn access_logs(&self) -> Result<TeamAccessLogsResponse> {
        let params = TeamAccessLogsRequest {
            count: Some(100),
            page: None,
        };

        self.client.post("team.accessLogs", &params).await
    }

    /// Get integration logs for the team
    pub async fn integration_logs(&self) -> Result<TeamIntegrationLogsResponse> {
        let params = TeamIntegrationLogsRequest {
            count: Some(100),
            page: None,
            service_id: None,
            user: None,
        };

        self.client.post("team.integrationLogs", &params).await
    }

    /// Get the team's profile field definitions
    ///
    /// Returns the list of profile field definitions for the team.
    pub async fn profile_get(&self) -> Result<TeamProfileGetResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("team.profile.get", &params).await
    }

    /// Get the team's preferences
    ///
    /// Returns the team's preferences/settings.
    pub async fn preferences_list(&self) -> Result<TeamPreferencesListResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("team.preferences.list", &params).await
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct TeamInfoResponse {
    pub team: Team,
}

#[derive(Debug, Deserialize)]
pub struct TeamBillableInfoResponse {
    pub billable_info: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct TeamAccessLogsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TeamAccessLogsResponse {
    pub logins: Vec<AccessLog>,
    pub paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct AccessLog {
    pub user_id: String,
    pub username: String,
    pub date_first: i64,
    pub date_last: i64,
    pub count: u32,
    pub ip: String,
    pub user_agent: String,
}

#[derive(Debug, Deserialize)]
pub struct Paging {
    pub count: u32,
    pub total: u32,
    pub page: u32,
    pub pages: u32,
}

#[derive(Debug, Serialize)]
pub struct TeamIntegrationLogsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TeamIntegrationLogsResponse {
    pub logs: Vec<IntegrationLog>,
    pub paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct IntegrationLog {
    pub service_id: String,
    pub service_type: String,
    pub user_id: String,
    pub user_name: String,
    pub channel: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct TeamProfileGetResponse {
    pub profile: TeamProfile,
}

#[derive(Debug, Deserialize)]
pub struct TeamProfile {
    pub fields: Vec<ProfileField>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileField {
    pub id: String,
    pub ordering: i32,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TeamPreferencesListResponse {
    #[serde(flatten)]
    pub preferences: serde_json::Value,
}
