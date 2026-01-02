//! Calls API
//!
//! Methods for Slack Calls integration.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Calls API client
pub struct CallsApi {
    client: SlackClient,
}

impl CallsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Register a new call
    ///
    /// # Arguments
    ///
    /// * `external_unique_id` - Unique ID for the call
    /// * `join_url` - URL to join the call
    pub async fn add(&self, external_unique_id: &str, join_url: &str) -> Result<CallsAddResponse> {
        let params = CallsAddRequest {
            external_unique_id: external_unique_id.to_string(),
            join_url: join_url.to_string(),
            created_by: None,
            date_start: None,
            desktop_app_join_url: None,
            external_display_id: None,
            title: None,
            users: None,
        };

        self.client.post("calls.add", &params).await
    }

    /// End a call
    ///
    /// # Arguments
    ///
    /// * `id` - Call ID
    pub async fn end(&self, id: &str) -> Result<CallsEndResponse> {
        let params = CallsEndRequest {
            id: id.to_string(),
            duration: None,
        };

        self.client.post("calls.end", &params).await
    }

    /// Get information about a call
    ///
    /// # Arguments
    ///
    /// * `id` - Call ID
    pub async fn info(&self, id: &str) -> Result<CallsInfoResponse> {
        let params = [("id", id)];

        self.client.get("calls.info", &params).await
    }

    /// Update a call
    ///
    /// # Arguments
    ///
    /// * `id` - Call ID
    /// * `title` - New title (optional)
    /// * `join_url` - New join URL (optional)
    pub async fn update(
        &self,
        id: &str,
        title: Option<&str>,
        join_url: Option<&str>,
    ) -> Result<CallsUpdateResponse> {
        let params = CallsUpdateRequest {
            id: id.to_string(),
            title: title.map(|s| s.to_string()),
            join_url: join_url.map(|s| s.to_string()),
            desktop_app_join_url: None,
        };

        self.client.post("calls.update", &params).await
    }

    /// Add users to a call
    ///
    /// # Arguments
    ///
    /// * `id` - Call ID
    /// * `users` - User IDs to add
    pub async fn participants_add(
        &self,
        id: &str,
        users: &[&str],
    ) -> Result<CallsParticipantsAddResponse> {
        let params = CallsParticipantsAddRequest {
            id: id.to_string(),
            users: users.iter().map(|s| s.to_string()).collect(),
        };

        self.client.post("calls.participants.add", &params).await
    }

    /// Remove users from a call
    ///
    /// # Arguments
    ///
    /// * `id` - Call ID
    /// * `users` - User IDs to remove
    pub async fn participants_remove(
        &self,
        id: &str,
        users: &[&str],
    ) -> Result<CallsParticipantsRemoveResponse> {
        let params = CallsParticipantsRemoveRequest {
            id: id.to_string(),
            users: users.iter().map(|s| s.to_string()).collect(),
        };

        self.client.post("calls.participants.remove", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct CallsAddRequest {
    pub external_unique_id: String,
    pub join_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_app_join_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_display_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<CallUser>>,
}

#[derive(Debug, Serialize)]
pub struct CallUser {
    pub slack_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CallsAddResponse {
    pub call: Call,
}

#[derive(Debug, Serialize)]
pub struct CallsEndRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CallsEndResponse {}

#[derive(Debug, Deserialize)]
pub struct CallsInfoResponse {
    pub call: Call,
}

#[derive(Debug, Serialize)]
pub struct CallsUpdateRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_app_join_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CallsUpdateResponse {
    pub call: Call,
}

#[derive(Debug, Serialize)]
pub struct CallsParticipantsAddRequest {
    pub id: String,
    pub users: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CallsParticipantsAddResponse {}

#[derive(Debug, Serialize)]
pub struct CallsParticipantsRemoveRequest {
    pub id: String,
    pub users: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CallsParticipantsRemoveResponse {}

#[derive(Debug, Deserialize)]
pub struct Call {
    pub id: String,
    pub date_start: i64,
    pub external_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_app_join_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_display_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<serde_json::Value>>,
}
