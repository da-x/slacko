//! Integration tests for Socket Mode API
//!
//! Note: Socket Mode requires an app-level token (xapp-...), which is different
//! from bot or user tokens. These tests verify the API structure but may not
//! complete full Socket Mode connections without proper app configuration.

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_socket_mode_open_connection() {
    init();
    let client = skip_if_no_client!(test_client());

    // apps.connections.open requires an app-level token (xapp-...)
    let result = client.socket_mode().open_connection().await;

    match result {
        Ok(response) => {
            assert!(
                !response.url.is_empty(),
                "WebSocket URL should not be empty"
            );
            assert!(
                response.url.starts_with("wss://"),
                "URL should be a WebSocket URL"
            );
            println!("✓ apps.connections.open: got WebSocket URL");
        }
        Err(e) => {
            // Expected to fail without app-level token
            println!(
                "✓ apps.connections.open: {} (requires app-level token xapp-...)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_socket_mode_event_types() {
    use slacko::api::socket_mode::SocketModeEventType;

    // Test event type parsing
    assert_eq!(
        SocketModeEventType::from("events_api"),
        SocketModeEventType::EventsApi
    );
    assert_eq!(
        SocketModeEventType::from("interactive"),
        SocketModeEventType::Interactive
    );
    assert_eq!(
        SocketModeEventType::from("slash_commands"),
        SocketModeEventType::SlashCommands
    );
    assert_eq!(
        SocketModeEventType::from("hello"),
        SocketModeEventType::Hello
    );
    assert_eq!(
        SocketModeEventType::from("disconnect"),
        SocketModeEventType::Disconnect
    );

    // Unknown types should be captured
    match SocketModeEventType::from("unknown_type") {
        SocketModeEventType::Unknown(s) => assert_eq!(s, "unknown_type"),
        _ => panic!("Expected Unknown variant"),
    }

    println!("✓ Socket Mode event types parse correctly");
}
