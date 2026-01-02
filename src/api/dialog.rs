//! Dialog API (Legacy)
//!
//! Methods for opening legacy dialogs. Note: Modals (Views API) are preferred for new apps.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Dialog API client
pub struct DialogApi {
    client: SlackClient,
}

impl DialogApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Open a dialog
    ///
    /// # Arguments
    ///
    /// * `trigger_id` - Trigger ID from an interaction payload
    /// * `dialog` - Dialog object
    pub async fn open(
        &self,
        trigger_id: &str,
        dialog: serde_json::Value,
    ) -> Result<DialogOpenResponse> {
        let params = DialogOpenRequest {
            trigger_id: trigger_id.to_string(),
            dialog,
        };

        self.client.post("dialog.open", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct DialogOpenRequest {
    pub trigger_id: String,
    pub dialog: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DialogOpenResponse {}
