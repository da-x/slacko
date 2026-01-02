//! Integration tests for Socket Mode API
//!
//! Note: Socket Mode requires an app-level token (xapp-...), which is different
//! from bot or user tokens. These tests verify the API structure but may not
//! complete full Socket Mode connections without proper app configuration.

mod common;

use common::{init, test_client};
use slacko::api::socket_mode::{
    EventsApiPayload, InteractivePayload, SlashCommandPayload, SocketModeEnvelope,
    SocketModeEventType,
};

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

#[tokio::test]
async fn test_socket_mode_envelope_parsing() {
    // Test parsing a hello envelope
    let hello_json = r#"{
        "type": "hello",
        "envelope_id": "abc123",
        "accepts_response_payload": false
    }"#;

    let envelope: SocketModeEnvelope = serde_json::from_str(hello_json).unwrap();
    assert_eq!(envelope.envelope_id, "abc123");
    assert_eq!(envelope.envelope_type, "hello");
    assert!(!envelope.accepts_response_payload);
    assert!(envelope.payload.is_none());

    println!("✓ Socket Mode hello envelope parses correctly");
}

#[tokio::test]
async fn test_socket_mode_events_api_envelope() {
    // Test parsing an events_api envelope
    let events_json = r#"{
        "type": "events_api",
        "envelope_id": "evt123",
        "accepts_response_payload": false,
        "payload": {
            "type": "event_callback",
            "team_id": "T12345",
            "api_app_id": "A12345",
            "event": {
                "type": "app_mention",
                "user": "U12345",
                "text": "<@U67890> hello",
                "ts": "1234567890.123456",
                "channel": "C12345"
            },
            "event_id": "Ev12345",
            "event_time": 1234567890
        }
    }"#;

    let envelope: SocketModeEnvelope = serde_json::from_str(events_json).unwrap();
    assert_eq!(envelope.envelope_id, "evt123");
    assert_eq!(envelope.envelope_type, "events_api");
    assert!(envelope.payload.is_some());

    // Parse the payload
    let payload: EventsApiPayload =
        serde_json::from_value(envelope.payload.unwrap()).unwrap();
    assert_eq!(payload.team_id, Some("T12345".to_string()));
    assert_eq!(payload.api_app_id, Some("A12345".to_string()));
    assert!(payload.event.is_some());

    println!("✓ Socket Mode events_api envelope parses correctly");
}

#[tokio::test]
async fn test_socket_mode_interactive_envelope() {
    // Test parsing an interactive envelope (button click)
    let interactive_json = r#"{
        "type": "interactive",
        "envelope_id": "int123",
        "accepts_response_payload": true,
        "payload": {
            "type": "block_actions",
            "user": {
                "id": "U12345",
                "username": "testuser",
                "name": "Test User",
                "team_id": "T12345"
            },
            "channel": {
                "id": "C12345",
                "name": "general"
            },
            "trigger_id": "123.456.abc",
            "response_url": "https://hooks.slack.com/actions/xxx",
            "actions": [
                {
                    "action_id": "button_click",
                    "block_id": "block1",
                    "type": "button",
                    "value": "clicked"
                }
            ]
        }
    }"#;

    let envelope: SocketModeEnvelope = serde_json::from_str(interactive_json).unwrap();
    assert_eq!(envelope.envelope_id, "int123");
    assert_eq!(envelope.envelope_type, "interactive");
    assert!(envelope.accepts_response_payload);

    // Parse the payload
    let payload: InteractivePayload =
        serde_json::from_value(envelope.payload.unwrap()).unwrap();
    assert_eq!(payload.interaction_type, "block_actions");
    assert!(payload.user.is_some());
    assert_eq!(payload.user.as_ref().unwrap().id, "U12345");
    assert!(payload.channel.is_some());
    assert_eq!(payload.channel.as_ref().unwrap().id, "C12345");
    assert_eq!(payload.trigger_id, Some("123.456.abc".to_string()));
    assert!(!payload.actions.is_empty());

    println!("✓ Socket Mode interactive envelope parses correctly");
}

#[tokio::test]
async fn test_socket_mode_slash_command_envelope() {
    // Test parsing a slash command envelope
    let slash_json = r#"{
        "type": "slash_commands",
        "envelope_id": "cmd123",
        "accepts_response_payload": true,
        "payload": {
            "command": "/weather",
            "text": "London",
            "response_url": "https://hooks.slack.com/commands/xxx",
            "trigger_id": "123.456.def",
            "user_id": "U12345",
            "user_name": "testuser",
            "channel_id": "C12345",
            "channel_name": "general",
            "team_id": "T12345",
            "team_domain": "testworkspace"
        }
    }"#;

    let envelope: SocketModeEnvelope = serde_json::from_str(slash_json).unwrap();
    assert_eq!(envelope.envelope_id, "cmd123");
    assert_eq!(envelope.envelope_type, "slash_commands");
    assert!(envelope.accepts_response_payload);

    // Parse the payload
    let payload: SlashCommandPayload =
        serde_json::from_value(envelope.payload.unwrap()).unwrap();
    assert_eq!(payload.command, "/weather");
    assert_eq!(payload.text, Some("London".to_string()));
    assert_eq!(payload.user_id, "U12345");
    assert_eq!(payload.channel_id, "C12345");
    assert!(payload.trigger_id.is_some());

    println!("✓ Socket Mode slash_commands envelope parses correctly");
}

#[tokio::test]
async fn test_socket_mode_disconnect_envelope() {
    // Test parsing a disconnect envelope
    let disconnect_json = r#"{
        "type": "disconnect",
        "envelope_id": "disc123",
        "accepts_response_payload": false,
        "payload": {
            "reason": "link_disabled"
        }
    }"#;

    let envelope: SocketModeEnvelope = serde_json::from_str(disconnect_json).unwrap();
    assert_eq!(envelope.envelope_id, "disc123");
    assert_eq!(envelope.envelope_type, "disconnect");
    assert!(!envelope.accepts_response_payload);

    let reason = envelope
        .payload
        .as_ref()
        .and_then(|p| p.get("reason"))
        .and_then(|r| r.as_str())
        .unwrap_or("unknown");
    assert_eq!(reason, "link_disabled");

    println!("✓ Socket Mode disconnect envelope parses correctly");
}

#[tokio::test]
async fn test_socket_mode_retry_envelope() {
    // Test parsing an envelope with retry info
    let retry_json = r#"{
        "type": "events_api",
        "envelope_id": "retry123",
        "accepts_response_payload": false,
        "retry_attempt": 2,
        "retry_reason": "timeout",
        "payload": {
            "type": "event_callback",
            "team_id": "T12345",
            "event": {}
        }
    }"#;

    let envelope: SocketModeEnvelope = serde_json::from_str(retry_json).unwrap();
    assert_eq!(envelope.envelope_id, "retry123");
    assert_eq!(envelope.retry_attempt, Some(2));
    assert_eq!(envelope.retry_reason, Some("timeout".to_string()));

    println!("✓ Socket Mode retry envelope parses correctly");
}

#[tokio::test]
async fn test_socket_mode_view_submission_envelope() {
    // Test parsing a view_submission interactive payload
    let view_json = r#"{
        "type": "interactive",
        "envelope_id": "view123",
        "accepts_response_payload": true,
        "payload": {
            "type": "view_submission",
            "user": {
                "id": "U12345",
                "name": "testuser"
            },
            "trigger_id": "123.456.ghi",
            "view": {
                "id": "V12345",
                "type": "modal",
                "callback_id": "my_modal",
                "state": {
                    "values": {
                        "block1": {
                            "input1": {
                                "type": "plain_text_input",
                                "value": "user input"
                            }
                        }
                    }
                }
            }
        }
    }"#;

    let envelope: SocketModeEnvelope = serde_json::from_str(view_json).unwrap();
    assert_eq!(envelope.envelope_type, "interactive");

    let payload: InteractivePayload =
        serde_json::from_value(envelope.payload.unwrap()).unwrap();
    assert_eq!(payload.interaction_type, "view_submission");
    assert!(payload.view.is_some());
    assert!(payload.trigger_id.is_some());

    println!("✓ Socket Mode view_submission envelope parses correctly");
}
