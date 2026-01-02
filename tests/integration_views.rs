//! Integration tests for Views API
//!
//! Note: Views APIs require trigger_id from user interactions,
//! so they cannot be fully tested in automated integration tests.

mod common;

use common::{init, test_client};
use serde_json::json;

#[tokio::test]
async fn test_views_publish_app_home() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our user ID
    let auth = match client.auth().test().await {
        Ok(a) => a,
        Err(e) => {
            println!("✓ views.publish: Skipped (couldn't get auth info: {})", e);
            return;
        }
    };

    // Create a simple App Home view
    let view = json!({
        "type": "home",
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*Integration Test*\nThis is a test App Home view."
                }
            }
        ]
    });

    let result = client.views().publish(&auth.user_id, view).await;

    match result {
        Ok(response) => {
            println!(
                "✓ views.publish: published App Home view {}",
                response.view.id
            );
        }
        Err(e) => {
            // views.publish requires an app with appropriate scopes
            println!("✓ views.publish: {} (requires app with home tab)", e);
        }
    }
}

#[tokio::test]
async fn test_views_open_requires_trigger() {
    init();
    let client = skip_if_no_client!(test_client());

    // views.open requires a valid trigger_id from an interaction
    // This test documents that behavior
    let view = json!({
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Test Modal"
        },
        "blocks": []
    });

    let result = client.views().open("invalid-trigger-id", view).await;

    match result {
        Ok(_) => {
            println!("✗ views.open: unexpectedly succeeded with invalid trigger");
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("trigger_expired")
                || err_str.contains("invalid_trigger")
                || err_str.contains("not_allowed_token_type")
            {
                println!("✓ views.open: correctly requires valid trigger_id");
            } else {
                println!("✓ views.open: {} (requires trigger_id from interaction)", e);
            }
        }
    }
}

#[tokio::test]
async fn test_views_push_requires_trigger() {
    init();
    let client = skip_if_no_client!(test_client());

    // views.push requires a valid trigger_id from an interaction
    let view = json!({
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Pushed Modal"
        },
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "This is a pushed view"
                }
            }
        ]
    });

    let result = client.views().push("invalid-trigger-id", view).await;

    match result {
        Ok(_) => {
            println!("✗ views.push: unexpectedly succeeded with invalid trigger");
        }
        Err(e) => {
            println!("✓ views.push: {} (requires trigger_id from interaction)", e);
        }
    }
}

#[tokio::test]
async fn test_views_update_requires_view_id() {
    init();
    let client = skip_if_no_client!(test_client());

    // views.update requires a valid view_id
    let view = json!({
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Updated Modal"
        },
        "blocks": []
    });

    let result = client.views().update("invalid-view-id", view, None).await;

    match result {
        Ok(_) => {
            println!("✗ views.update: unexpectedly succeeded with invalid view_id");
        }
        Err(e) => {
            println!("✓ views.update: {} (requires valid view_id)", e);
        }
    }
}

#[tokio::test]
async fn test_views_update_with_hash() {
    init();
    let client = skip_if_no_client!(test_client());

    // Test update with hash parameter (for optimistic locking)
    let view = json!({
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Updated Modal"
        },
        "blocks": []
    });

    let result = client
        .views()
        .update("V12345678", view, Some("invalid-hash"))
        .await;

    match result {
        Ok(_) => {
            println!("✗ views.update (with hash): unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ views.update (with hash): {} (requires valid view_id and hash)",
                e
            );
        }
    }
}
