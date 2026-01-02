//! Reminders API
//!
//! Methods for creating and managing reminders.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Reminders API client
pub struct RemindersApi {
    client: SlackClient,
}

impl RemindersApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Create a reminder
    ///
    /// # Arguments
    ///
    /// * `text` - Reminder text
    /// * `time` - Unix timestamp or natural language time
    /// * `user` - User ID to remind (optional, defaults to current user)
    pub async fn add(
        &self,
        text: &str,
        time: &str,
        user: Option<&str>,
    ) -> Result<ReminderAddResponse> {
        let params = ReminderAddRequest {
            text: text.to_string(),
            time: time.to_string(),
            user: user.map(|s| s.to_string()),
        };

        self.client.post("reminders.add", &params).await
    }

    /// Mark a reminder as complete
    ///
    /// # Arguments
    ///
    /// * `reminder` - Reminder ID
    pub async fn complete(&self, reminder: &str) -> Result<ReminderCompleteResponse> {
        let params = ReminderCompleteRequest {
            reminder: reminder.to_string(),
        };

        self.client.post("reminders.complete", &params).await
    }

    /// Delete a reminder
    ///
    /// # Arguments
    ///
    /// * `reminder` - Reminder ID
    pub async fn delete(&self, reminder: &str) -> Result<ReminderDeleteResponse> {
        let params = ReminderDeleteRequest {
            reminder: reminder.to_string(),
        };

        self.client.post("reminders.delete", &params).await
    }

    /// Get information about a reminder
    ///
    /// # Arguments
    ///
    /// * `reminder` - Reminder ID
    pub async fn info(&self, reminder: &str) -> Result<ReminderInfoResponse> {
        let params = [("reminder", reminder)];

        self.client.get("reminders.info", &params).await
    }

    /// List all reminders created by or for a user
    pub async fn list(&self) -> Result<ReminderListResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("reminders.list", &params).await
    }

    /// List all saved items including reminders (uses saved.list endpoint)
    /// This endpoint works with xoxc tokens unlike reminders.list
    pub async fn list_saved(&self) -> Result<SavedListResponse> {
        self.list_saved_with_filter(None).await
    }

    /// List saved items with optional filter (uses multipart form data like browser)
    /// filter can be: None (all), Some("completed"), Some("in_progress")
    /// Returns all items by automatically handling pagination
    pub async fn list_saved_with_filter(&self, filter: Option<&str>) -> Result<SavedListResponse> {
        let mut all_items = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let response = self.list_saved_page(filter, cursor.as_deref()).await?;
            all_items.extend(response.saved_items);

            // Check if there are more pages
            match response.next_cursor {
                Some(ref c) if !c.is_empty() => cursor = Some(c.clone()),
                _ => break,
            }
        }

        Ok(SavedListResponse {
            saved_items: all_items,
            next_cursor: None,
        })
    }

    /// List a single page of saved items using form-urlencoded POST
    async fn list_saved_page(
        &self,
        filter: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<SavedListResponse> {
        let url = format!("{}/saved.list", self.client.base_url);
        let headers = self.client.auth.build_headers();

        // Form-urlencoded without token (auth header provides it)
        // Note: limit must be <= 50
        let mut params = vec![
            ("limit", "50".to_string()),
            ("include_tombstones", "true".to_string()),
        ];

        if let Some(f) = filter {
            params.push(("filter", f.to_string()));
        }

        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }

        let response = self
            .client
            .http
            .post(&url)
            .headers(headers)
            .form(&params)
            .send()
            .await?;

        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);
            return Err(crate::error::SlackError::RateLimitExceeded { retry_after });
        }

        let text = response.text().await?;

        let slack_response: crate::types::SlackResponse<SavedListResponse> =
            serde_json::from_str(&text).map_err(|e| {
                crate::error::SlackError::api_error(
                    "saved.list",
                    format!("Parse error: {} - {}", e, &text[..text.len().min(200)]),
                )
            })?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(crate::error::SlackError::api_error("saved.list", error_msg));
        }

        slack_response
            .data
            .ok_or_else(|| crate::error::SlackError::api_error("saved.list", "No data in response"))
    }

    /// Delete a saved item by ID
    pub async fn delete_saved(&self, item_id: &str) -> Result<SavedDeleteResponse> {
        let url = format!("{}/saved.delete", self.client.base_url);
        let headers = self.client.auth.build_headers();

        let params = [("item_type", "reminder"), ("item_id", item_id)];

        let response = self
            .client
            .http
            .post(&url)
            .headers(headers)
            .form(&params)
            .send()
            .await?;

        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);
            return Err(crate::error::SlackError::RateLimitExceeded { retry_after });
        }

        let slack_response: crate::types::SlackResponse<SavedDeleteResponse> =
            response.json().await?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(crate::error::SlackError::api_error(
                "saved.delete",
                error_msg,
            ));
        }

        Ok(SavedDeleteResponse {})
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct ReminderAddRequest {
    pub text: String,
    pub time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReminderAddResponse {
    pub reminder: Reminder,
}

#[derive(Debug, Serialize)]
pub struct ReminderCompleteRequest {
    pub reminder: String,
}

#[derive(Debug, Deserialize)]
pub struct ReminderCompleteResponse {}

#[derive(Debug, Serialize)]
pub struct ReminderDeleteRequest {
    pub reminder: String,
}

#[derive(Debug, Deserialize)]
pub struct ReminderDeleteResponse {}

#[derive(Debug, Deserialize)]
pub struct ReminderInfoResponse {
    pub reminder: Reminder,
}

#[derive(Debug, Deserialize)]
pub struct ReminderListResponse {
    pub reminders: Vec<Reminder>,
}

#[derive(Debug, Deserialize)]
pub struct Reminder {
    pub id: String,
    #[serde(default)]
    pub creator: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub recurring: bool,
    #[serde(default)]
    pub time: i64,
    pub complete_ts: Option<i64>,
}

// Saved items API types (works with xoxc tokens)

#[derive(Debug, Deserialize)]
pub struct SavedListResponse {
    pub saved_items: Vec<SavedItem>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SavedItem {
    pub item_id: String,
    pub item_type: String,
    #[serde(default)]
    pub date_created: i64,
    #[serde(default)]
    pub date_due: i64,
    #[serde(default)]
    pub is_archived: bool,
}

#[derive(Debug, Serialize)]
pub struct SavedDeleteRequest {
    pub item_id: String,
    pub item_type: String,
}

#[derive(Debug, Deserialize)]
pub struct SavedDeleteResponse {}
