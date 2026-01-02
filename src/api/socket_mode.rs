//! Socket Mode API
//!
//! Socket Mode allows your app to receive events via WebSocket instead of HTTP.
//! This is useful for apps that can't expose a public HTTP endpoint.

use crate::client::SlackClient;
use crate::error::{Result, SlackError};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use tracing::{debug, error, info, warn};

/// Socket Mode API client
pub struct SocketModeApi {
    client: SlackClient,
}

// ============================================
// Request/Response Types
// ============================================

#[derive(Debug, Serialize)]
struct ConnectionOpenRequest {}

/// Response from apps.connections.open
#[derive(Debug, Deserialize)]
pub struct ConnectionOpenResponse {
    pub ok: bool,
    pub url: String,
}

// ============================================
// Socket Mode Event Types
// ============================================

/// A Socket Mode envelope wrapping an event
#[derive(Debug, Clone, Deserialize)]
pub struct SocketModeEnvelope {
    /// Unique ID for this envelope - must be acknowledged
    pub envelope_id: String,

    /// Type of payload: "events_api", "interactive", "slash_commands", "hello", "disconnect"
    #[serde(rename = "type")]
    pub envelope_type: String,

    /// Whether a response payload can be included in the acknowledgment
    #[serde(default)]
    pub accepts_response_payload: bool,

    /// The actual event payload (absent for hello/disconnect)
    #[serde(default)]
    pub payload: Option<Value>,

    /// Retry attempt number (if this is a retry)
    #[serde(default)]
    pub retry_attempt: Option<u32>,

    /// Reason for retry
    #[serde(default)]
    pub retry_reason: Option<String>,
}

/// Parsed Socket Mode event with typed payload
#[derive(Debug, Clone)]
pub struct SocketModeEvent {
    /// Unique ID for this envelope
    pub envelope_id: String,

    /// Type of the envelope
    pub envelope_type: SocketModeEventType,

    /// Whether a response can be sent with the acknowledgment
    pub accepts_response_payload: bool,

    /// The parsed payload
    pub payload: SocketModePayload,
}

/// Types of Socket Mode events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketModeEventType {
    /// Events API event (app_mention, message, etc.)
    EventsApi,
    /// Interactive component (button click, modal submission, etc.)
    Interactive,
    /// Slash command invocation
    SlashCommands,
    /// Initial connection message
    Hello,
    /// Server requesting disconnect
    Disconnect,
    /// Unknown event type
    Unknown(String),
}

impl From<&str> for SocketModeEventType {
    fn from(s: &str) -> Self {
        match s {
            "events_api" => Self::EventsApi,
            "interactive" => Self::Interactive,
            "slash_commands" => Self::SlashCommands,
            "hello" => Self::Hello,
            "disconnect" => Self::Disconnect,
            other => Self::Unknown(other.to_string()),
        }
    }
}

/// Payload types for Socket Mode events
#[derive(Debug, Clone)]
pub enum SocketModePayload {
    /// Events API payload
    EventsApi(EventsApiPayload),
    /// Interactive component payload
    Interactive(InteractivePayload),
    /// Slash command payload
    SlashCommand(SlashCommandPayload),
    /// Hello message (connection established)
    Hello,
    /// Disconnect request
    Disconnect { reason: String },
    /// Raw payload for unknown types
    Raw(Value),
}

/// Events API payload structure
#[derive(Debug, Clone, Deserialize)]
pub struct EventsApiPayload {
    /// The token (deprecated, use signing secret instead)
    #[serde(default)]
    pub token: Option<String>,

    /// Team ID
    pub team_id: Option<String>,

    /// API app ID
    pub api_app_id: Option<String>,

    /// The actual event
    pub event: Option<Value>,

    /// Event type (e.g., "app_mention", "message")
    #[serde(rename = "type")]
    pub payload_type: Option<String>,

    /// Event ID
    pub event_id: Option<String>,

    /// Event timestamp
    pub event_time: Option<i64>,

    /// Authorizations
    #[serde(default)]
    pub authorizations: Vec<Value>,
}

/// Interactive component payload
#[derive(Debug, Clone, Deserialize)]
pub struct InteractivePayload {
    /// Type of interaction
    #[serde(rename = "type")]
    pub interaction_type: String,

    /// User who triggered the interaction
    #[serde(default)]
    pub user: Option<InteractiveUser>,

    /// Channel where interaction occurred
    #[serde(default)]
    pub channel: Option<InteractiveChannel>,

    /// Trigger ID for opening modals
    #[serde(default)]
    pub trigger_id: Option<String>,

    /// Response URL for async responses
    #[serde(default)]
    pub response_url: Option<String>,

    /// Action details
    #[serde(default)]
    pub actions: Vec<Value>,

    /// View details (for view_submission, view_closed)
    #[serde(default)]
    pub view: Option<Value>,

    /// Message that was interacted with
    #[serde(default)]
    pub message: Option<Value>,

    /// Full raw payload
    #[serde(flatten)]
    pub extra: Value,
}

/// User in interactive payload
#[derive(Debug, Clone, Deserialize)]
pub struct InteractiveUser {
    pub id: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub team_id: Option<String>,
}

/// Channel in interactive payload
#[derive(Debug, Clone, Deserialize)]
pub struct InteractiveChannel {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

/// Slash command payload
#[derive(Debug, Clone, Deserialize)]
pub struct SlashCommandPayload {
    /// The command (e.g., "/weather")
    pub command: String,

    /// Text after the command
    #[serde(default)]
    pub text: Option<String>,

    /// Response URL for async responses
    pub response_url: String,

    /// Trigger ID for opening modals
    #[serde(default)]
    pub trigger_id: Option<String>,

    /// User ID
    pub user_id: String,

    /// User name
    #[serde(default)]
    pub user_name: Option<String>,

    /// Channel ID
    pub channel_id: String,

    /// Channel name
    #[serde(default)]
    pub channel_name: Option<String>,

    /// Team ID
    #[serde(default)]
    pub team_id: Option<String>,

    /// Team domain
    #[serde(default)]
    pub team_domain: Option<String>,

    /// Full raw payload
    #[serde(flatten)]
    pub extra: Value,
}

/// Acknowledgment message sent back to Slack
#[derive(Debug, Serialize)]
struct SocketModeAck {
    envelope_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Value>,
}

// ============================================
// Socket Mode Client Implementation
// ============================================

impl SocketModeApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Open a Socket Mode connection
    ///
    /// Returns a WebSocket URL that can be used to establish the connection.
    /// Note: Requires an app-level token (xapp-...), not a bot or user token.
    pub async fn open_connection(&self) -> Result<ConnectionOpenResponse> {
        self.client
            .post("apps.connections.open", &ConnectionOpenRequest {})
            .await
    }

    /// Start Socket Mode and listen for events
    ///
    /// This method connects to Socket Mode and calls the provided handler for each event.
    /// The handler can optionally return a response payload.
    ///
    /// # Arguments
    ///
    /// * `handler` - Function called for each event, returns optional response payload
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use slacko::{SlackClient, AuthConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SlackClient::new(AuthConfig::bot("xapp-token"))?;
    /// client.socket_mode().start(|event| {
    ///     println!("Received: {:?}", event.envelope_type);
    ///     None // No response payload
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start<F>(&self, handler: F) -> Result<()>
    where
        F: Fn(SocketModeEvent) -> Option<Value> + Send + Sync + 'static,
    {
        let conn = self.open_connection().await?;
        self.run_connection(&conn.url, Arc::new(handler)).await
    }

    /// Start Socket Mode with automatic reconnection
    ///
    /// Like `start()`, but automatically reconnects if the connection is lost.
    /// Uses exponential backoff between reconnection attempts.
    pub async fn start_with_reconnect<F>(&self, handler: F) -> Result<()>
    where
        F: Fn(SocketModeEvent) -> Option<Value> + Send + Sync + 'static,
    {
        let handler = Arc::new(handler);
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let conn = match self.open_connection().await {
                Ok(c) => {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection
                    c
                }
                Err(e) => {
                    error!("Failed to open Socket Mode connection: {}", e);
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(max_backoff);
                    continue;
                }
            };

            match self.run_connection(&conn.url, handler.clone()).await {
                Ok(()) => {
                    info!("Socket Mode connection closed normally");
                    break Ok(());
                }
                Err(e) => {
                    warn!("Socket Mode connection error: {}, reconnecting...", e);
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(max_backoff);
                }
            }
        }
    }

    /// Run a single Socket Mode connection
    async fn run_connection<F>(&self, url: &str, handler: Arc<F>) -> Result<()>
    where
        F: Fn(SocketModeEvent) -> Option<Value> + Send + Sync + 'static,
    {
        info!("Connecting to Socket Mode: {}", url);

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| SlackError::websocket_error(format!("Failed to connect: {}", e)))?;

        info!("Socket Mode connected");

        let (mut write, mut read) = ws_stream.split();

        // Channel for sending acknowledgments
        let (ack_tx, mut ack_rx) = mpsc::channel::<SocketModeAck>(100);

        // Spawn task to send acknowledgments
        let write_task = tokio::spawn(async move {
            while let Some(ack) = ack_rx.recv().await {
                let msg =
                    serde_json::to_string(&ack).expect("SocketModeAck is always serializable");
                debug!("Sending ack: {}", msg);
                if let Err(e) = write.send(WsMessage::Text(msg)).await {
                    error!("Failed to send ack: {}", e);
                    break;
                }
            }
        });

        // Process incoming messages
        while let Some(msg) = read.next().await {
            match msg {
                Ok(WsMessage::Text(text)) => {
                    debug!("Socket Mode received: {}", text);

                    match serde_json::from_str::<SocketModeEnvelope>(&text) {
                        Ok(envelope) => {
                            let event = Self::parse_envelope(envelope.clone());

                            // Always acknowledge (except for hello which doesn't need it)
                            if event.envelope_type != SocketModeEventType::Hello {
                                let response = handler(event);
                                let ack = SocketModeAck {
                                    envelope_id: envelope.envelope_id,
                                    payload: response,
                                };
                                if ack_tx.send(ack).await.is_err() {
                                    error!("Failed to queue ack");
                                    break;
                                }
                            } else {
                                // Just log hello, call handler but don't ack
                                info!("Socket Mode hello received");
                                let _ = handler(event);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse Socket Mode message: {}", e);
                        }
                    }
                }
                Ok(WsMessage::Close(frame)) => {
                    info!("Socket Mode close frame: {:?}", frame);
                    break;
                }
                Ok(WsMessage::Ping(data)) => {
                    debug!("Socket Mode ping received, data: {:?}", data);
                    // Pong is handled automatically by tungstenite
                }
                Ok(_) => {
                    debug!("Socket Mode received other message type");
                }
                Err(e) => {
                    error!("Socket Mode error: {}", e);
                    write_task.abort();
                    return Err(SlackError::websocket_error(format!(
                        "WebSocket error: {}",
                        e
                    )));
                }
            }
        }

        write_task.abort();
        Ok(())
    }

    /// Parse a raw envelope into a typed event
    fn parse_envelope(envelope: SocketModeEnvelope) -> SocketModeEvent {
        let envelope_type = SocketModeEventType::from(envelope.envelope_type.as_str());

        let payload = match &envelope_type {
            SocketModeEventType::EventsApi => {
                if let Some(p) = &envelope.payload {
                    match serde_json::from_value::<EventsApiPayload>(p.clone()) {
                        Ok(events) => SocketModePayload::EventsApi(events),
                        Err(_) => SocketModePayload::Raw(p.clone()),
                    }
                } else {
                    SocketModePayload::Raw(Value::Null)
                }
            }
            SocketModeEventType::Interactive => {
                if let Some(p) = &envelope.payload {
                    match serde_json::from_value::<InteractivePayload>(p.clone()) {
                        Ok(interactive) => SocketModePayload::Interactive(interactive),
                        Err(_) => SocketModePayload::Raw(p.clone()),
                    }
                } else {
                    SocketModePayload::Raw(Value::Null)
                }
            }
            SocketModeEventType::SlashCommands => {
                if let Some(p) = &envelope.payload {
                    match serde_json::from_value::<SlashCommandPayload>(p.clone()) {
                        Ok(cmd) => SocketModePayload::SlashCommand(cmd),
                        Err(_) => SocketModePayload::Raw(p.clone()),
                    }
                } else {
                    SocketModePayload::Raw(Value::Null)
                }
            }
            SocketModeEventType::Hello => SocketModePayload::Hello,
            SocketModeEventType::Disconnect => {
                let reason = envelope
                    .payload
                    .as_ref()
                    .and_then(|p| p.get("reason"))
                    .and_then(|r| r.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                SocketModePayload::Disconnect { reason }
            }
            SocketModeEventType::Unknown(_) => {
                SocketModePayload::Raw(envelope.payload.unwrap_or(Value::Null))
            }
        };

        SocketModeEvent {
            envelope_id: envelope.envelope_id,
            envelope_type,
            accepts_response_payload: envelope.accepts_response_payload,
            payload,
        }
    }
}
