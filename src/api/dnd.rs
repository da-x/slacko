//! DND (Do Not Disturb) API
//!
//! Methods for managing Do Not Disturb settings.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// DND API client
pub struct DndApi {
    client: SlackClient,
}

impl DndApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Get DND status for the current user
    pub async fn info(&self) -> Result<DndInfoResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("dnd.info", &params).await
    }

    /// Get DND status for a specific user
    ///
    /// # Arguments
    ///
    /// * `user` - User ID
    pub async fn team_info(&self, user: &str) -> Result<DndInfoResponse> {
        let params = [("user", user)];

        self.client.get("dnd.teamInfo", &params).await
    }

    /// Set a DND snooze for the current user
    ///
    /// # Arguments
    ///
    /// * `num_minutes` - Number of minutes to snooze
    pub async fn set_snooze(&self, num_minutes: u32) -> Result<DndSetSnoozeResponse> {
        let params = DndSetSnoozeRequest { num_minutes };

        self.client.post("dnd.setSnooze", &params).await
    }

    /// End the current user's DND snooze
    pub async fn end_snooze(&self) -> Result<DndEndSnoozeResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.post("dnd.endSnooze", &params).await
    }

    /// End DND for the current user
    pub async fn end_dnd(&self) -> Result<DndEndDndResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.post("dnd.endDnd", &params).await
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct DndInfoResponse {
    pub dnd_enabled: bool,
    pub next_dnd_start_ts: i64,
    pub next_dnd_end_ts: i64,
    pub snooze_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snooze_endtime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snooze_remaining: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DndSetSnoozeRequest {
    pub num_minutes: u32,
}

#[derive(Debug, Deserialize)]
pub struct DndSetSnoozeResponse {
    pub snooze_enabled: bool,
    pub snooze_endtime: i64,
    pub snooze_remaining: i64,
}

#[derive(Debug, Deserialize)]
pub struct DndEndSnoozeResponse {
    pub dnd_enabled: bool,
    pub next_dnd_start_ts: i64,
    pub next_dnd_end_ts: i64,
    pub snooze_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct DndEndDndResponse {}
