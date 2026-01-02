//! # Slack SDK
//!
//! A comprehensive Rust SDK for the Slack API with support for both stealth mode
//! (xoxc/xoxd tokens) and regular OAuth tokens (xoxp/xoxb).
//!
//! ## Features
//!
//! - **Multiple Authentication Methods**: OAuth tokens (xoxp, xoxb) and stealth mode (xoxc/xoxd)
//! - **Complete API Coverage**: 24 API modules covering all Slack functionality
//! - **Real-time Messaging**: RTM API support via WebSocket
//! - **Block Kit Support**: Builders for rich message layouts
//! - **Type Safety**: Strongly typed API responses and requests
//! - **Async/Await**: Built on tokio for high-performance async operations
//! - **Rate Limit Handling**: Automatic detection with retry information
//!
//! ## Quick Start
//!
//! ```no_run
//! use slacko::{SlackClient, AuthConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client with bot token
//!     let client = SlackClient::new(AuthConfig::bot("xoxb-your-token"))?;
//!
//!     // Post a message
//!     client.chat().post_message("#general", "Hello from Rust!").await?;
//!
//!     // List channels
//!     let channels = client.conversations().list().await?;
//!     for channel in channels.channels {
//!         println!("#{}", channel.name.unwrap_or_default());
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! ### OAuth Tokens
//!
//! ```no_run
//! use slacko::{SlackClient, AuthConfig};
//!
//! // Bot token (xoxb-...)
//! let client = SlackClient::new(AuthConfig::bot("xoxb-token")).unwrap();
//!
//! // User token (xoxp-...)
//! let client = SlackClient::new(AuthConfig::oauth("xoxp-token")).unwrap();
//! ```
//!
//! ### Stealth Mode
//!
//! Stealth mode uses browser session tokens and does not require app installation:
//!
//! ```no_run
//! use slacko::{SlackClient, AuthConfig};
//!
//! let client = SlackClient::new(
//!     AuthConfig::stealth("xoxc-token", "xoxd-cookie")
//! ).unwrap();
//! ```
//!
//! ### Environment Variables
//!
//! The SDK can auto-detect credentials from environment variables:
//!
//! ```no_run
//! use slacko::{SlackClient, AuthConfig};
//!
//! // Checks SLACK_XOXC_TOKEN/SLACK_XOXD_COOKIE, then SLACK_XOXP_TOKEN, then SLACK_BOT_TOKEN
//! let client = SlackClient::new(AuthConfig::from_env().unwrap()).unwrap();
//! ```
//!
//! ## API Modules
//!
//! The SDK provides access to all major Slack APIs:
//!
//! - [`api::admin`] - Enterprise Grid administration
//! - [`api::apps`] - App management and permissions
//! - [`api::auth`] - Authentication verification
//! - [`api::bookmarks`] - Channel bookmarks
//! - [`api::calls`] - Slack Calls integration
//! - [`api::chat`] - Messages and threads
//! - [`api::conversations`] - Channels, DMs, and groups
//! - [`api::dialog`] - Legacy dialogs
//! - [`api::dnd`] - Do Not Disturb settings
//! - [`api::emoji`] - Custom emoji
//! - [`api::files`] - File uploads and management
//! - [`api::oauth`] - OAuth token exchange
//! - [`api::openid`] - OpenID Connect authentication
//! - [`api::pins`] - Pinned messages
//! - [`api::reactions`] - Emoji reactions
//! - [`api::reminders`] - Reminders
//! - [`api::rtm`] - Real-time messaging via WebSocket
//! - [`api::search`] - Message and file search
//! - [`api::stars`] - Starred items
//! - [`api::team`] - Team information
//! - [`api::usergroups`] - User groups
//! - [`api::users`] - User information and presence
//! - [`api::views`] - Modals and App Home
//! - [`api::workflows`] - Workflow Builder
//!
//! ## Block Kit
//!
//! Build rich messages using Block Kit builders:
//!
//! ```no_run
//! use slacko::{SlackClient, AuthConfig, MessageBuilder};
//!
//! let client = SlackClient::new(AuthConfig::bot("xoxb-token")).unwrap();
//!
//! let blocks = MessageBuilder::new()
//!     .section("*Hello* from Block Kit")
//!     .divider()
//!     .build();
//!
//! // client.chat().post_message_with_blocks("#general", "Hello", blocks).await?;
//! ```
//!
//! ## Error Handling
//!
//! The SDK provides detailed error types:
//!
//! ```no_run
//! use slacko::{SlackClient, AuthConfig, SlackError};
//!
//! async fn example() {
//!     let client = SlackClient::new(AuthConfig::bot("xoxb-token")).unwrap();
//!
//!     match client.chat().post_message("#general", "Hello").await {
//!         Ok(response) => println!("Message sent: {}", response.ts),
//!         Err(SlackError::RateLimitExceeded { retry_after }) => {
//!             println!("Rate limited, retry after {} seconds", retry_after);
//!         }
//!         Err(SlackError::ApiError { code, message }) => {
//!             println!("API error {}: {}", code, message);
//!         }
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```

pub mod auth;
pub mod blocks;
pub mod client;
pub mod error;
pub mod types;

pub mod api;

// Re-export commonly used types
pub use auth::{AuthConfig, AuthType};
pub use client::SlackClient;
pub use error::{Result, SlackError};

// Re-export Block Kit builders for convenience
pub use blocks::{
    ActionsBlock, ButtonElement, ConfirmationDialog, ContextBlock, DividerBlock, HeaderBlock,
    ImageBlock, MessageBuilder, OptionObject, SectionBlock, SelectElement, TextObject,
};
