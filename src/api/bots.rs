//! Bots API
//!
//! Methods for getting information about bot users.

use crate::client::SlackClient;
use crate::error::Result;
use serde::Deserialize;

/// Bots API client
pub struct BotsApi {
    client: SlackClient,
}

impl BotsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Get information about a bot user
    ///
    /// # Arguments
    ///
    /// * `bot` - Bot user ID
    pub async fn info(&self, bot: &str) -> Result<BotInfoResponse> {
        let params = [("bot", bot)];
        self.client.get("bots.info", &params).await
    }
}

#[derive(Debug, Deserialize)]
pub struct BotInfoResponse {
    pub bot: Bot,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    pub id: String,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub updated: Option<i64>,
    #[serde(default)]
    pub app_id: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub icons: Option<BotIcons>,
}

#[derive(Debug, Deserialize)]
pub struct BotIcons {
    #[serde(default)]
    pub image_36: Option<String>,
    #[serde(default)]
    pub image_48: Option<String>,
    #[serde(default)]
    pub image_72: Option<String>,
}
