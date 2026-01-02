//! RTM (Real-Time Messaging) API
//!
//! Methods for real-time messaging via WebSocket.

use crate::client::SlackClient;
use crate::error::{Result, SlackError};
use crate::types::RtmConnectResponse;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use tracing::{debug, error, info, warn};

/// RTM API client
pub struct RtmApi {
    client: SlackClient,
}

/// RTM event types
#[derive(Debug, Deserialize, Serialize)]
pub struct RtmEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// RTM message event
#[derive(Debug, Deserialize)]
pub struct RtmMessageEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub channel: Option<String>,
    pub user: Option<String>,
    pub text: Option<String>,
    pub ts: Option<String>,
    pub thread_ts: Option<String>,
    pub bot_id: Option<String>,
}

/// Callback type for RTM message handlers
pub type MessageHandler = Box<dyn Fn(RtmMessageEvent) + Send + Sync>;

impl RtmApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Connect to the RTM API
    ///
    /// Returns the WebSocket URL and self information
    pub async fn connect(&self) -> Result<RtmConnectResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("rtm.connect", &params).await
    }

    /// Start an RTM connection and listen for events
    ///
    /// # Arguments
    ///
    /// * `on_message` - Callback function for message events
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use slacko::{SlackClient, AuthConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SlackClient::new(AuthConfig::oauth("token"))?;
    /// client.rtm().start(|msg| {
    ///     println!("Received: {:?}", msg.text);
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start<F>(&self, on_message: F) -> Result<()>
    where
        F: Fn(RtmMessageEvent) + Send + Sync + 'static,
    {
        let rtm_info = self.connect().await?;
        let ws_url = &rtm_info.url;
        let bot_id = rtm_info.self_info.id.clone();

        info!("Connecting to RTM WebSocket: {}", ws_url);

        let (ws_stream, _) = connect_async(ws_url)
            .await
            .map_err(|e| SlackError::websocket_error(format!("Failed to connect: {}", e)))?;

        info!("RTM WebSocket connected");

        let (mut write, mut read) = ws_stream.split();

        // Message processing loop
        while let Some(msg) = read.next().await {
            match msg {
                Ok(WsMessage::Text(text)) => {
                    debug!("RTM received: {}", text);

                    if let Ok(event) = serde_json::from_str::<RtmMessageEvent>(&text) {
                        // Only process message events
                        if event.event_type == "message" {
                            // Skip messages from ourselves
                            if event.bot_id.as_ref() == Some(&bot_id) {
                                continue;
                            }

                            // Call the message handler
                            on_message(event);
                        }
                    }
                }
                Ok(WsMessage::Close(_)) => {
                    warn!("RTM WebSocket closed");
                    break;
                }
                Ok(WsMessage::Ping(data)) => {
                    debug!("RTM received ping");
                    if let Err(e) = write.send(WsMessage::Pong(data)).await {
                        error!("Failed to send pong: {}", e);
                    }
                }
                Ok(_) => {
                    debug!("RTM received other message type");
                }
                Err(e) => {
                    error!("RTM WebSocket error: {}", e);
                    return Err(SlackError::websocket_error(format!(
                        "WebSocket error: {}",
                        e
                    )));
                }
            }
        }

        warn!("RTM connection closed");
        Ok(())
    }

    /// Start RTM and filter messages by channel
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID to filter messages
    /// * `on_message` - Callback function for message events
    pub async fn start_with_channel<F>(&self, channel: &str, on_message: F) -> Result<()>
    where
        F: Fn(RtmMessageEvent) + Send + Sync + 'static,
    {
        let channel_filter = channel.to_string();

        self.start(move |event| {
            if event.channel.as_deref() == Some(&channel_filter) {
                on_message(event);
            }
        })
        .await
    }
}
