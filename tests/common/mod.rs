//! Common test utilities and setup

#![allow(dead_code)]

use slacko::{AuthConfig, SlackClient};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment (logging, etc.)
pub fn init() {
    INIT.call_once(|| {
        // Initialize tracing for tests if RUST_LOG is set
        if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::fmt().with_test_writer().init();
        }
    });
}

/// Create a test client from environment variables
///
/// Requires either:
/// - SLACK_XOXC_TOKEN + SLACK_XOXD_COOKIE (stealth mode)
/// - SLACK_XOXP_TOKEN (OAuth)
/// - SLACK_BOT_TOKEN or SLACK_TOKEN (bot)
pub fn test_client() -> Option<SlackClient> {
    init();

    match AuthConfig::from_env() {
        Ok(auth) => SlackClient::new(auth).ok(),
        Err(_) => None,
    }
}

/// Skip test if no credentials are available
#[macro_export]
macro_rules! skip_if_no_client {
    ($client:expr) => {
        match $client {
            Some(c) => c,
            None => {
                eprintln!("Skipping test: No Slack credentials in environment");
                return;
            }
        }
    };
}

/// Get test channel ID from environment or use default
pub fn test_channel() -> String {
    std::env::var("SLACK_TEST_CHANNEL").unwrap_or_else(|_| "general".to_string())
}

/// Get test user ID from environment (for DM tests)
pub fn test_user_id() -> Option<String> {
    std::env::var("SLACK_TEST_USER_ID").ok()
}

/// Generate a unique test message to avoid collisions
pub fn unique_message(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("[TEST-{}] {} - {}", timestamp, prefix, uuid_v4())
}

/// Simple UUID v4 generator for test uniqueness
fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!(
        "{:x}-{:x}-4{:x}-{:x}-{:x}",
        (t >> 96) as u32,
        (t >> 80) as u16,
        ((t >> 64) as u16) & 0x0fff,
        ((t >> 48) as u16 & 0x3fff) | 0x8000,
        t as u64 & 0xffffffffffff
    )
}

/// Cleanup helper - delete a message after test
pub async fn cleanup_message(client: &SlackClient, channel: &str, ts: &str) {
    let _ = client.chat().delete_message(channel, ts).await;
}

/// Cleanup helper - leave a channel after test
pub async fn cleanup_leave_channel(client: &SlackClient, channel: &str) {
    let _ = client.conversations().leave(channel).await;
}
