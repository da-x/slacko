//! API Test endpoint
//!
//! Methods for testing the Slack API connection.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API Test client
pub struct ApiApi {
    client: SlackClient,
}

impl ApiApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Test the Slack API connection
    ///
    /// This method checks if your API call is working.
    pub async fn test(&self) -> Result<ApiTestResponse> {
        let params = ApiTestRequest {
            args: HashMap::new(),
        };

        self.client.post("api.test", &params).await
    }

    /// Test the Slack API connection with custom arguments
    ///
    /// The API echoes back any arguments you pass to it.
    ///
    /// # Arguments
    ///
    /// * `args` - Key-value pairs to echo back
    pub async fn test_with_args(&self, args: HashMap<String, String>) -> Result<ApiTestResponse> {
        let params = ApiTestRequest { args };

        self.client.post("api.test", &params).await
    }
}

impl ApiTestResponse {
    pub fn ok(&self) -> bool {
        true // If we got a response, the test passed
    }
}

#[derive(Debug, Serialize, Default)]
pub struct ApiTestRequest {
    #[serde(flatten)]
    pub args: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiTestResponse {
    #[serde(flatten)]
    pub args: HashMap<String, serde_json::Value>,
}
