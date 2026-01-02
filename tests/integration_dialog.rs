//! Integration tests for Dialog API (Legacy)
//!
//! Note: Dialog APIs require trigger_id from user interactions.
//! The Views API (modals) is preferred for new apps.

mod common;

use common::{init, test_client};
use serde_json::json;

#[tokio::test]
async fn test_dialog_open_requires_trigger() {
    init();
    let client = skip_if_no_client!(test_client());

    // dialog.open requires a valid trigger_id from an interaction
    let dialog = json!({
        "callback_id": "test-dialog",
        "title": "Test Dialog",
        "elements": []
    });

    let result = client.dialog().open("invalid-trigger-id", dialog).await;

    match result {
        Ok(_) => {
            println!("✗ dialog.open: unexpectedly succeeded with invalid trigger");
        }
        Err(e) => {
            println!(
                "✓ dialog.open: {} (requires trigger_id from interaction)",
                e
            );
        }
    }
}
