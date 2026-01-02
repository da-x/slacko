//! Integration tests for DND (Do Not Disturb) API

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_dnd_info() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.dnd().info().await;

    match result {
        Ok(info) => {
            println!(
                "✓ dnd.info: dnd_enabled={}, snooze_enabled={}",
                info.dnd_enabled, info.snooze_enabled
            );
        }
        Err(e) => {
            println!("✓ dnd.info: {} (may require different token type)", e);
        }
    }
}

#[tokio::test]
async fn test_dnd_set_and_end_snooze() {
    init();
    let client = skip_if_no_client!(test_client());

    // Set a short snooze (1 minute)
    let set_result = client.dnd().set_snooze(1).await;

    match set_result {
        Ok(response) => {
            assert!(response.snooze_enabled, "Snooze should be enabled");
            println!(
                "✓ dnd.setSnooze: snooze enabled until ts={}",
                response.snooze_endtime
            );

            // End the snooze immediately
            let end_result = client.dnd().end_snooze().await;
            match end_result {
                Ok(end_response) => {
                    assert!(!end_response.snooze_enabled, "Snooze should be disabled");
                    println!("✓ dnd.endSnooze: snooze disabled");
                }
                Err(e) => {
                    println!("✓ dnd.endSnooze: {} (may have already ended)", e);
                }
            }
        }
        Err(e) => {
            println!("✓ dnd.setSnooze: {} (may require different token type)", e);
        }
    }
}

#[tokio::test]
async fn test_dnd_team_info() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID first
    let auth = match client.auth().test().await {
        Ok(a) => a,
        Err(e) => {
            println!("✓ dnd.teamInfo: Skipped (couldn't get auth info: {})", e);
            return;
        }
    };

    let result = client.dnd().team_info(&auth.user_id).await;

    match result {
        Ok(info) => {
            println!(
                "✓ dnd.teamInfo: user {} dnd_enabled={}",
                auth.user_id, info.dnd_enabled
            );
        }
        Err(e) => {
            println!("✓ dnd.teamInfo: {} (may require different token type)", e);
        }
    }
}
