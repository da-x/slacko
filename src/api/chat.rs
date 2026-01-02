//! Chat API
//!
//! Methods for posting, updating, and deleting messages.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Chat API client
pub struct ChatApi {
    client: SlackClient,
}

impl ChatApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Post a message to a channel
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID or name
    /// * `text` - Message text
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use slacko::{SlackClient, AuthConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SlackClient::new(AuthConfig::oauth("token"))?;
    /// let response = client.chat()
    ///     .post_message("C12345", "Hello, world!")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_message(&self, channel: &str, text: &str) -> Result<PostMessageResponse> {
        let params = PostMessageRequest::new(channel).text(text);
        self.client.post("chat.postMessage", &params).await
    }

    /// Post a message with full options
    pub async fn post_message_with_options(
        &self,
        params: PostMessageRequest,
    ) -> Result<PostMessageResponse> {
        self.client.post("chat.postMessage", &params).await
    }

    /// Post a message with Block Kit blocks
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID or name
    /// * `blocks` - Block Kit blocks (use MessageBuilder)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use slacko::{SlackClient, AuthConfig};
    /// # use slacko::blocks::MessageBuilder;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SlackClient::new(AuthConfig::oauth("token"))?;
    /// let message = MessageBuilder::new()
    ///     .text("Fallback text")
    ///     .header("Welcome!")
    ///     .section("*This* is a Block Kit message")
    ///     .build();
    ///
    /// client.chat().post_message_blocks("C12345", message).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_message_blocks(
        &self,
        channel: &str,
        message: Value,
    ) -> Result<PostMessageResponse> {
        let mut params = serde_json::from_value::<PostMessageRequest>(message)?;
        params.channel = channel.to_string();

        self.client.post("chat.postMessage", &params).await
    }

    /// Update an existing message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `ts` - Message timestamp
    /// * `text` - New message text
    pub async fn update_message(
        &self,
        channel: &str,
        ts: &str,
        text: &str,
    ) -> Result<UpdateMessageResponse> {
        let params = UpdateMessageRequest {
            channel: channel.to_string(),
            ts: ts.to_string(),
            text: Some(text.to_string()),
            blocks: None,
            as_user: None,
        };

        self.client.post("chat.update", &params).await
    }

    /// Delete a message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `ts` - Message timestamp
    pub async fn delete_message(&self, channel: &str, ts: &str) -> Result<DeleteMessageResponse> {
        let params = DeleteMessageRequest {
            channel: channel.to_string(),
            ts: ts.to_string(),
            as_user: None,
        };

        self.client.post("chat.delete", &params).await
    }

    /// Post an ephemeral message (only visible to one user)
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `user` - User ID to show the message to
    /// * `text` - Message text
    pub async fn post_ephemeral(
        &self,
        channel: &str,
        user: &str,
        text: &str,
    ) -> Result<PostEphemeralResponse> {
        let params = PostEphemeralRequest {
            channel: channel.to_string(),
            user: user.to_string(),
            text: Some(text.to_string()),
            blocks: None,
            as_user: None,
        };

        self.client.post("chat.postEphemeral", &params).await
    }

    /// Get a permalink for a message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `message_ts` - Message timestamp
    pub async fn get_permalink(
        &self,
        channel: &str,
        message_ts: &str,
    ) -> Result<GetPermalinkResponse> {
        let params = [("channel", channel), ("message_ts", message_ts)];

        self.client.get("chat.getPermalink", &params).await
    }

    /// Schedule a message to be sent later
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `text` - Message text
    /// * `post_at` - Unix timestamp when to send
    pub async fn schedule_message(
        &self,
        channel: &str,
        text: &str,
        post_at: i64,
    ) -> Result<ScheduleMessageResponse> {
        let params = ScheduleMessageRequest {
            channel: channel.to_string(),
            text: Some(text.to_string()),
            post_at,
            blocks: None,
            as_user: None,
        };

        self.client.post("chat.scheduleMessage", &params).await
    }

    /// Delete a scheduled message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `scheduled_message_id` - Scheduled message ID to delete
    pub async fn delete_scheduled_message(
        &self,
        channel: &str,
        scheduled_message_id: &str,
    ) -> Result<DeleteScheduledMessageResponse> {
        let params = DeleteScheduledMessageRequest {
            channel: channel.to_string(),
            scheduled_message_id: scheduled_message_id.to_string(),
        };

        self.client
            .post("chat.deleteScheduledMessage", &params)
            .await
    }

    /// Provide custom unfurl behavior for URLs in messages
    ///
    /// This method allows apps to provide custom rich previews for URLs
    /// shared in messages. Your app must be subscribed to the `link_shared`
    /// event to receive URLs that need unfurling.
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID where the URL was shared
    /// * `ts` - Timestamp of the message containing the URL
    /// * `unfurls` - JSON object mapping URLs to unfurl attachments
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use slacko::{SlackClient, AuthConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SlackClient::new(AuthConfig::oauth("token"))?;
    /// use serde_json::json;
    ///
    /// let unfurls = json!({
    ///     "https://example.com/page": {
    ///         "text": "Custom unfurl for example.com",
    ///         "color": "#36a64f"
    ///     }
    /// });
    ///
    /// client.chat().unfurl("C12345", "1234567890.123456", unfurls).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unfurl(
        &self,
        channel: &str,
        ts: &str,
        unfurls: serde_json::Value,
    ) -> Result<UnfurlResponse> {
        let params = UnfurlRequest {
            channel: channel.to_string(),
            ts: ts.to_string(),
            unfurls,
            user_auth_message: None,
            user_auth_required: None,
            user_auth_url: None,
        };

        self.client.post("chat.unfurl", &params).await
    }

    /// Provide custom unfurl behavior with full options
    pub async fn unfurl_with_options(&self, params: UnfurlRequest) -> Result<UnfurlResponse> {
        self.client.post("chat.unfurl", &params).await
    }

    /// List scheduled messages
    ///
    /// Returns a list of scheduled messages for a channel or the entire workspace.
    ///
    /// # Arguments
    ///
    /// * `channel` - Optional channel ID to filter by
    pub async fn scheduled_messages_list(
        &self,
        channel: Option<&str>,
    ) -> Result<ScheduledMessagesListResponse> {
        let params = ScheduledMessagesListRequest {
            channel: channel.map(|c| c.to_string()),
            cursor: None,
            latest: None,
            oldest: None,
            limit: None,
            team_id: None,
        };

        self.client
            .post("chat.scheduledMessages.list", &params)
            .await
    }

    /// List scheduled messages with full options
    pub async fn scheduled_messages_list_with_options(
        &self,
        params: ScheduledMessagesListRequest,
    ) -> Result<ScheduledMessagesListResponse> {
        self.client
            .post("chat.scheduledMessages.list", &params)
            .await
    }

    /// Send a /me message
    ///
    /// Sends a message with the /me prefix, which displays as an action.
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `text` - Message text (will be prefixed with the user's name)
    pub async fn me_message(&self, channel: &str, text: &str) -> Result<MeMessageResponse> {
        let params = MeMessageRequest {
            channel: channel.to_string(),
            text: text.to_string(),
        };

        self.client.post("chat.meMessage", &params).await
    }

    // ========== Streaming Methods for AI/LLM Apps ==========

    /// Start a text stream for AI/LLM responses
    ///
    /// Initiates a streaming message that can be appended to in real-time.
    /// This is designed for AI-enabled Slack apps to provide streaming responses.
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `thread_ts` - Optional thread timestamp to reply in
    pub async fn start_stream(
        &self,
        channel: &str,
        thread_ts: Option<&str>,
    ) -> Result<StartStreamResponse> {
        let params = StartStreamRequest {
            channel: channel.to_string(),
            thread_ts: thread_ts.map(|s| s.to_string()),
        };

        self.client.post("chat.startStream", &params).await
    }

    /// Append text to an existing stream
    ///
    /// Adds content to a streaming message started with `start_stream`.
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `stream_id` - Stream ID from start_stream response
    /// * `text` - Text to append to the stream
    pub async fn append_stream(
        &self,
        channel: &str,
        stream_id: &str,
        text: &str,
    ) -> Result<AppendStreamResponse> {
        let params = AppendStreamRequest {
            channel: channel.to_string(),
            stream_id: stream_id.to_string(),
            text: text.to_string(),
        };

        self.client.post("chat.appendStream", &params).await
    }

    /// Stop/finalize a text stream
    ///
    /// Completes a streaming message and converts it to a regular message.
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `stream_id` - Stream ID from start_stream response
    pub async fn stop_stream(&self, channel: &str, stream_id: &str) -> Result<StopStreamResponse> {
        let params = StopStreamRequest {
            channel: channel.to_string(),
            stream_id: stream_id.to_string(),
        };

        self.client.post("chat.stopStream", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostMessageRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl PostMessageRequest {
    pub fn new(channel: &str) -> Self {
        Self {
            channel: channel.to_string(),
            ..Default::default()
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn thread_ts(mut self, ts: &str) -> Self {
        self.thread_ts = Some(ts.to_string());
        self
    }

    pub fn blocks(mut self, blocks: Vec<Value>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn attachments(mut self, attachments: Vec<Value>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn reply_broadcast(mut self, broadcast: bool) -> Self {
        self.reply_broadcast = Some(broadcast);
        self
    }

    pub fn unfurl_links(mut self, unfurl: bool) -> Self {
        self.unfurl_links = Some(unfurl);
        self
    }

    pub fn unfurl_media(mut self, unfurl: bool) -> Self {
        self.unfurl_media = Some(unfurl);
        self
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn icon_emoji(mut self, emoji: &str) -> Self {
        self.icon_emoji = Some(emoji.to_string());
        self
    }

    pub fn icon_url(mut self, url: &str) -> Self {
        self.icon_url = Some(url.to_string());
        self
    }

    pub fn mrkdwn(mut self, enabled: bool) -> Self {
        self.mrkdwn = Some(enabled);
        self
    }

    pub fn parse(mut self, mode: &str) -> Self {
        self.parse = Some(mode.to_string());
        self
    }

    pub fn link_names(mut self, enabled: bool) -> Self {
        self.link_names = Some(enabled);
        self
    }

    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct PostMessageResponse {
    pub channel: String,
    pub ts: String,
    pub message: Message,
}

#[derive(Debug, Serialize)]
pub struct UpdateMessageRequest {
    pub channel: String,
    pub ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessageResponse {
    pub channel: String,
    pub ts: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteMessageRequest {
    pub channel: String,
    pub ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteMessageResponse {
    pub channel: String,
    pub ts: String,
}

#[derive(Debug, Serialize)]
pub struct PostEphemeralRequest {
    pub channel: String,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PostEphemeralResponse {
    pub message_ts: String,
}

#[derive(Debug, Deserialize)]
pub struct GetPermalinkResponse {
    pub permalink: String,
}

#[derive(Debug, Serialize)]
pub struct ScheduleMessageRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub post_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleMessageResponse {
    pub channel: String,
    pub scheduled_message_id: String,
    pub post_at: i64,
}

#[derive(Debug, Serialize)]
pub struct DeleteScheduledMessageRequest {
    pub channel: String,
    pub scheduled_message_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteScheduledMessageResponse {}

/// Provide custom unfurl behavior for URLs in messages
///
/// # Arguments
///
/// * `channel` - Channel ID
/// * `ts` - Message timestamp
/// * `unfurls` - Map of URLs to unfurl definitions
#[derive(Debug, Serialize)]
pub struct UnfurlRequest {
    pub channel: String,
    pub ts: String,
    pub unfurls: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UnfurlResponse {}

#[derive(Debug, Serialize, Default)]
pub struct ScheduledMessagesListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduledMessagesListResponse {
    pub scheduled_messages: Vec<ScheduledMessage>,
    #[serde(default)]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduledMessage {
    pub id: String,
    pub channel_id: String,
    pub post_at: i64,
    pub date_created: i64,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetadata {
    #[serde(default)]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MeMessageRequest {
    pub channel: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct MeMessageResponse {
    pub channel: String,
    pub ts: String,
}

// ========== Streaming Types ==========

#[derive(Debug, Serialize)]
pub struct StartStreamRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartStreamResponse {
    pub channel: String,
    pub stream_id: String,
    pub ts: String,
}

#[derive(Debug, Serialize)]
pub struct AppendStreamRequest {
    pub channel: String,
    pub stream_id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct AppendStreamResponse {}

#[derive(Debug, Serialize)]
pub struct StopStreamRequest {
    pub channel: String,
    pub stream_id: String,
}

#[derive(Debug, Deserialize)]
pub struct StopStreamResponse {
    pub channel: String,
    pub ts: String,
}
