//! Views API
//!
//! Methods for managing modals and App Home views.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Views API client
pub struct ViewsApi {
    client: SlackClient,
}

impl ViewsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Open a modal view
    ///
    /// # Arguments
    ///
    /// * `trigger_id` - Trigger ID from an interaction payload
    /// * `view` - View object (Block Kit)
    pub async fn open(
        &self,
        trigger_id: &str,
        view: serde_json::Value,
    ) -> Result<ViewOpenResponse> {
        let params = ViewOpenRequest {
            trigger_id: trigger_id.to_string(),
            view,
        };

        self.client.post("views.open", &params).await
    }

    /// Push a new view onto the modal stack
    ///
    /// # Arguments
    ///
    /// * `trigger_id` - Trigger ID from an interaction payload
    /// * `view` - View object (Block Kit)
    pub async fn push(
        &self,
        trigger_id: &str,
        view: serde_json::Value,
    ) -> Result<ViewPushResponse> {
        let params = ViewPushRequest {
            trigger_id: trigger_id.to_string(),
            view,
        };

        self.client.post("views.push", &params).await
    }

    /// Update an existing view
    ///
    /// # Arguments
    ///
    /// * `view_id` - View ID (from view_submission or view_closed payload)
    /// * `view` - Updated view object
    /// * `hash` - View hash for optimistic locking (optional)
    pub async fn update(
        &self,
        view_id: &str,
        view: serde_json::Value,
        hash: Option<&str>,
    ) -> Result<ViewUpdateResponse> {
        let params = ViewUpdateRequest {
            view_id: Some(view_id.to_string()),
            external_id: None,
            view,
            hash: hash.map(|s| s.to_string()),
        };

        self.client.post("views.update", &params).await
    }

    /// Publish a view to a user's App Home
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `view` - View object (Block Kit)
    pub async fn publish(
        &self,
        user_id: &str,
        view: serde_json::Value,
    ) -> Result<ViewPublishResponse> {
        let params = ViewPublishRequest {
            user_id: user_id.to_string(),
            view,
            hash: None,
        };

        self.client.post("views.publish", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct ViewOpenRequest {
    pub trigger_id: String,
    pub view: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ViewOpenResponse {
    pub view: View,
}

#[derive(Debug, Serialize)]
pub struct ViewPushRequest {
    pub trigger_id: String,
    pub view: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ViewPushResponse {
    pub view: View,
}

#[derive(Debug, Serialize)]
pub struct ViewUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub view: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ViewUpdateResponse {
    pub view: View,
}

#[derive(Debug, Serialize)]
pub struct ViewPublishRequest {
    pub user_id: String,
    pub view: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ViewPublishResponse {
    pub view: View,
}

#[derive(Debug, Deserialize)]
pub struct View {
    pub id: String,
    pub team_id: String,
    #[serde(rename = "type")]
    pub view_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<serde_json::Value>,
    pub blocks: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_on_close: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_close: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_installed_team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
}
