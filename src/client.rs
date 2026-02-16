//! Core Slack API client

use crate::api::{
    activity::ActivityApi, admin::AdminApi, api_test::ApiApi, apps::AppsApi, auth::AuthApi,
    bookmarks::BookmarksApi, bots::BotsApi, calls::CallsApi, chat::ChatApi,
    conversations::ConversationsApi, dialog::DialogApi, dnd::DndApi, emoji::EmojiApi,
    files::FilesApi, lists::ListsApi, messages::MessagesApi, oauth::OAuthApi, openid::OpenIDApi,
    pins::PinsApi, reactions::ReactionsApi, reminders::RemindersApi, rtm::RtmApi,
    search::SearchApi, socket_mode::SocketModeApi, stars::StarsApi, team::TeamApi,
    usergroups::UsergroupsApi, users::UsersApi, views::ViewsApi, workflows::WorkflowsApi,
};
use crate::auth::AuthConfig;
use crate::error::{Result, SlackError};
use crate::types::SlackResponse;
use reqwest::header::HeaderMap;
use std::sync::Arc;

const SLACK_API_BASE: &str = "https://slack.com/api";

/// Main Slack API client
///
/// This is the primary entry point for interacting with the Slack API.
/// It provides access to all API endpoints through specialized API clients.
///
/// # Example
///
/// ```no_run
/// use slacko::{SlackClient, AuthConfig};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SlackClient::new(AuthConfig::oauth("xoxp-token"))?;
///
///     // Post a message
///     client.chat()
///         .post_message("C12345", "Hello!")
///         .await?;
///
///     // List channels
///     let channels = client.conversations()
///         .list()
///         .await?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct SlackClient {
    pub(crate) http: reqwest::Client,
    pub(crate) auth: Arc<AuthConfig>,
    pub(crate) base_url: String,
}

impl SlackClient {
    /// Create a new Slack client with the given authentication configuration
    ///
    /// # Arguments
    ///
    /// * `auth` - Authentication configuration
    ///
    /// # Example
    ///
    /// ```
    /// use slacko::{SlackClient, AuthConfig};
    ///
    /// let client = SlackClient::new(
    ///     AuthConfig::oauth("xoxp-token")
    /// ).unwrap();
    /// ```
    pub fn new(auth: AuthConfig) -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent("slack-sdk-rust/0.1.0")
            .build()
            .map_err(|e| SlackError::config_error(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self {
            http,
            auth: Arc::new(auth),
            base_url: SLACK_API_BASE.to_string(),
        })
    }

    /// Get the Activity API client
    ///
    /// Provides methods for accessing activity feed and notifications.
    pub fn activity(&self) -> ActivityApi {
        ActivityApi::new(self.clone())
    }

    /// Get the API test client
    ///
    /// Provides methods for testing the Slack API connection.
    pub fn api(&self) -> ApiApi {
        ApiApi::new(self.clone())
    }

    /// Get the Bots API client
    ///
    /// Provides methods for getting information about bot users.
    pub fn bots(&self) -> BotsApi {
        BotsApi::new(self.clone())
    }

    /// Get the Chat API client
    ///
    /// Provides methods for posting, updating, and deleting messages.
    pub fn chat(&self) -> ChatApi {
        ChatApi::new(self.clone())
    }

    /// Get the Conversations API client
    ///
    /// Provides methods for managing channels, groups, and DMs.
    pub fn conversations(&self) -> ConversationsApi {
        ConversationsApi::new(self.clone())
    }

    /// Get the Users API client
    ///
    /// Provides methods for retrieving user information.
    pub fn users(&self) -> UsersApi {
        UsersApi::new(self.clone())
    }

    /// Get the Files API client
    ///
    /// Provides methods for uploading and managing files.
    pub fn files(&self) -> FilesApi {
        FilesApi::new(self.clone())
    }

    /// Get the Reactions API client
    ///
    /// Provides methods for adding and removing emoji reactions.
    pub fn reactions(&self) -> ReactionsApi {
        ReactionsApi::new(self.clone())
    }

    /// Get the Search API client
    ///
    /// Provides methods for searching messages and files.
    pub fn search(&self) -> SearchApi {
        SearchApi::new(self.clone())
    }

    /// Get the Team API client
    ///
    /// Provides methods for retrieving team/workspace information.
    pub fn team(&self) -> TeamApi {
        TeamApi::new(self.clone())
    }

    /// Get the RTM API client
    ///
    /// Provides methods for real-time messaging via WebSocket.
    pub fn rtm(&self) -> RtmApi {
        RtmApi::new(self.clone())
    }

    /// Get the Socket Mode API client
    ///
    /// Provides methods for receiving events via WebSocket using Socket Mode.
    /// This is the modern alternative to RTM for receiving events.
    /// Note: Requires an app-level token (xapp-...).
    pub fn socket_mode(&self) -> SocketModeApi {
        SocketModeApi::new(self.clone())
    }

    /// Get the Auth API client
    ///
    /// Provides methods for testing and managing authentication.
    pub fn auth(&self) -> AuthApi {
        AuthApi::new(self.clone())
    }

    /// Get the Pins API client
    ///
    /// Provides methods for pinning and unpinning messages.
    pub fn pins(&self) -> PinsApi {
        PinsApi::new(self.clone())
    }

    /// Get the Stars API client
    ///
    /// Provides methods for starring items.
    pub fn stars(&self) -> StarsApi {
        StarsApi::new(self.clone())
    }

    /// Get the Reminders API client
    ///
    /// Provides methods for creating and managing reminders.
    pub fn reminders(&self) -> RemindersApi {
        RemindersApi::new(self.clone())
    }

    /// Get the DND API client
    ///
    /// Provides methods for Do Not Disturb settings.
    pub fn dnd(&self) -> DndApi {
        DndApi::new(self.clone())
    }

    /// Get the Emoji API client
    ///
    /// Provides methods for listing custom emoji.
    pub fn emoji(&self) -> EmojiApi {
        EmojiApi::new(self.clone())
    }

    /// Get the OAuth v2 API client
    ///
    /// Provides methods for OAuth token exchange and management.
    /// Use this for building public Slack apps with OAuth flows.
    pub fn oauth(&self) -> OAuthApi {
        OAuthApi::new(self.clone())
    }

    /// Get the OpenID Connect API client
    ///
    /// Provides methods for OpenID Connect authentication flows.
    /// Use this for implementing Sign in with Slack and identity verification.
    pub fn openid(&self) -> OpenIDApi {
        OpenIDApi::new(self.clone())
    }

    /// Get the Usergroups API client
    ///
    /// Provides methods for managing user groups.
    pub fn usergroups(&self) -> UsergroupsApi {
        UsergroupsApi::new(self.clone())
    }

    /// Get the Views API client
    ///
    /// Provides methods for managing modals and App Home.
    pub fn views(&self) -> ViewsApi {
        ViewsApi::new(self.clone())
    }

    /// Get the Dialog API client (Legacy)
    ///
    /// Provides methods for opening legacy dialogs.
    pub fn dialog(&self) -> DialogApi {
        DialogApi::new(self.clone())
    }

    /// Get the Bookmarks API client
    ///
    /// Provides methods for managing channel bookmarks.
    pub fn bookmarks(&self) -> BookmarksApi {
        BookmarksApi::new(self.clone())
    }

    /// Get the Admin API client
    ///
    /// Provides methods for Enterprise Grid administration.
    pub fn admin(&self) -> AdminApi {
        AdminApi::new(self.clone())
    }

    /// Get the Apps API client
    ///
    /// Provides methods for managing app configurations.
    pub fn apps(&self) -> AppsApi {
        AppsApi::new(self.clone())
    }

    /// Get the Calls API client
    ///
    /// Provides methods for Slack Calls integration.
    pub fn calls(&self) -> CallsApi {
        CallsApi::new(self.clone())
    }

    /// Get the Workflows API client
    ///
    /// Provides methods for Workflow Builder integrations.
    pub fn workflows(&self) -> WorkflowsApi {
        WorkflowsApi::new(self.clone())
    }

    /// Get the Lists API client
    ///
    /// Provides methods for managing Slack Lists.
    pub fn lists(&self) -> ListsApi {
        ListsApi::new(self.clone())
    }

    /// Get the Messages API client
    ///
    /// Provides methods for managing messages.
    pub fn messages(&self) -> MessagesApi {
        MessagesApi::new(self.clone())
    }

    /// Make a POST request to the Slack API
    pub(crate) async fn post<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, method);
        let headers = self.auth.build_headers();

        let response = self
            .http
            .post(&url)
            .headers(headers)
            .json(params)
            .send()
            .await?;

        // Check for rate limiting
        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);

            return Err(SlackError::RateLimitExceeded { retry_after });
        }

        let slack_response: SlackResponse<T> = response.json().await?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(SlackError::api_error(method, error_msg));
        }

        slack_response
            .data
            .ok_or_else(|| SlackError::api_error(method, "No data in response"))
    }

    /// Make a GET request to the Slack API
    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, method);
        let headers = self.auth.build_headers();

        let response = self
            .http
            .get(&url)
            .headers(headers)
            .query(params)
            .send()
            .await?;

        // Check for rate limiting
        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);

            return Err(SlackError::RateLimitExceeded { retry_after });
        }

        let slack_response: SlackResponse<T> = response.json().await?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(SlackError::api_error(method, error_msg));
        }

        slack_response
            .data
            .ok_or_else(|| SlackError::api_error(method, "No data in response"))
    }

    /// Get headers for API requests
    #[allow(dead_code)]
    pub(crate) fn headers(&self) -> HeaderMap {
        self.auth.build_headers()
    }

    /// Upload a file via multipart form
    pub(crate) async fn upload_file<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        file_data: Vec<u8>,
        field_name: &str,
        file_name: &str,
    ) -> Result<T> {
        self.upload_file_with_params(method, file_data, field_name, file_name, &[])
            .await
    }

    /// Upload a file via multipart form with additional parameters
    pub(crate) async fn upload_file_with_params<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        file_data: Vec<u8>,
        field_name: &str,
        file_name: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        use reqwest::multipart::{Form, Part};

        let url = format!("{}/{}", self.base_url, method);
        let headers = self.auth.build_headers();

        let part = Part::bytes(file_data).file_name(file_name.to_string());

        let mut form = Form::new().part(field_name.to_string(), part);

        for (key, value) in params {
            form = form.text(key.to_string(), value.to_string());
        }

        let response = self
            .http
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);

            return Err(SlackError::RateLimitExceeded { retry_after });
        }

        let slack_response: SlackResponse<T> = response.json().await?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(SlackError::api_error(method, error_msg));
        }

        slack_response
            .data
            .ok_or_else(|| SlackError::api_error(method, "No data in response"))
    }

    /// Make a POST request using multipart/form-data
    /// This encodes each struct field as a separate form-data part
    pub(crate) async fn post_multipart<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, method);
        let headers = self.auth.build_headers();

        // Serialize struct to Value first, then convert to multipart form
        let value = serde_json::to_value(params)?;
        let form = self.build_multipart_form(&value)?;

        let response = self
            .http
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        // Check for rate limiting
        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);

            return Err(SlackError::RateLimitExceeded { retry_after });
        }

        let slack_response: SlackResponse<T> = response.json().await?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(SlackError::api_error(method, error_msg));
        }

        slack_response
            .data
            .ok_or_else(|| SlackError::api_error(method, "No data in response"))
    }

    /// Make a POST request using multipart/form-data
    /// This encodes each struct field as a separate form-data part
    pub(crate) async fn post_multipart_direct<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, method);
        let headers = self.auth.build_headers();

        // Serialize struct to Value first, then convert to multipart form
        let value = serde_json::to_value(params)?;
        let form = self.build_multipart_form(&value)?;

        let response = self
            .http
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        // Check for rate limiting
        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);

            return Err(SlackError::RateLimitExceeded { retry_after });
        }

        let rsp: serde_json::Value = response.json().await?;
        let rsp: T = serde_json::from_value(rsp)?;

        Ok(rsp)
    }

    /// Build multipart form from JSON value using reqwest's Form
    fn build_multipart_form(&self, value: &serde_json::Value) -> Result<reqwest::multipart::Form> {
        use reqwest::multipart::Form;

        let mut form = Form::new();

        // Flatten the JSON object to handle nested structures
        let flat_map = self.flatten_json_for_multipart(value, None)?;

        for (key, val) in flat_map {
            form = form.text(key, val);
        }

        Ok(form)
    }

    /// Flatten a JSON value into key-value pairs suitable for multipart form data
    /// Handles nested objects and arrays by converting them to JSON strings
    fn flatten_json_for_multipart(
        &self,
        value: &serde_json::Value,
        prefix: Option<&str>,
    ) -> Result<std::collections::HashMap<String, String>> {
        let mut result = std::collections::HashMap::new();

        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    // Skip null values (same as serde skip_serializing_if)
                    if val.is_null() {
                        continue;
                    }

                    let full_key = match prefix {
                        Some(p) => format!("{}[{}]", p, key),
                        None => key.clone(),
                    };

                    match val {
                        serde_json::Value::String(s) => {
                            result.insert(full_key, s.clone());
                        }
                        serde_json::Value::Bool(b) => {
                            result.insert(full_key, b.to_string());
                        }
                        serde_json::Value::Number(n) => {
                            result.insert(full_key, n.to_string());
                        }
                        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                            // For objects, encode as JSON string
                            let json_str = serde_json::to_string(val).map_err(|e| {
                                SlackError::config_error(format!(
                                    "Failed to serialize object: {}",
                                    e
                                ))
                            })?;
                            result.insert(full_key, json_str);
                        }
                        serde_json::Value::Null => {
                            // Skip null values
                        }
                    }
                }
            }
            _ => {
                // For non-object root values, use the prefix or default key
                let key = prefix.unwrap_or("value").to_string();
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Array(arr) => {
                        // For arrays, join with commas
                        let string_values: Vec<String> = arr
                            .iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect();
                        if string_values.len() == arr.len() {
                            string_values.join(",")
                        } else {
                            return Err(SlackError::config_error(
                                "Arrays must contain only strings for multipart encoding",
                            ));
                        }
                    }
                    serde_json::Value::Object(_) => serde_json::to_string(value).map_err(|e| {
                        SlackError::config_error(format!("Failed to serialize value: {}", e))
                    })?,
                    serde_json::Value::Null => return Ok(result), // Skip null values
                };
                result.insert(key, value_str);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = SlackClient::new(AuthConfig::oauth("xoxp-test-token"));
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_creation_stealth() {
        let client = SlackClient::new(AuthConfig::stealth("xoxc-token", "xoxd-cookie"));
        assert!(client.is_ok());
    }
}
