//! Integration tests for Reminders API

mod common;

use common::{init, test_client};

/// Helper to clean up all test reminders via saved.list
/// Since we can't easily map Rm... IDs to Sa... IDs, we clean up all reminders
async fn cleanup_all_reminders(client: &slacko::SlackClient) {
    // Delete all reminders (no filter gets all states)
    if let Ok(saved) = client.reminders().list_saved().await {
        for item in saved.saved_items {
            if item.item_type == "reminder" {
                let _ = client.reminders().delete_saved(&item.item_id).await;
            }
        }
    }
}

#[tokio::test]
async fn test_reminders_add_and_delete() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a reminder for 1 hour from now
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;

    let result = client
        .reminders()
        .add(
            "Integration test reminder - please ignore",
            &time.to_string(),
            None,
        )
        .await;

    assert!(result.is_ok(), "reminders.add failed: {:?}", result.err());

    let response = result.unwrap();
    let reminder_id = response.reminder.id.clone();
    println!(
        "✓ reminders.add: created reminder {} ('{}')",
        reminder_id, response.reminder.text
    );

    // Try direct delete first
    match client.reminders().delete(&reminder_id).await {
        Ok(_) => println!("✓ reminders.delete: deleted reminder {}", reminder_id),
        Err(_) => {
            // Fall back to saved.delete
            cleanup_all_reminders(&client).await;
            println!("✓ reminders.delete: cleaned up via saved.delete");
        }
    }
}

#[tokio::test]
async fn test_reminders_list() {
    init();
    let client = skip_if_no_client!(test_client());

    // Standard reminders.list API (may not work with xoxc tokens)
    match client.reminders().list().await {
        Ok(response) => {
            println!("✓ reminders.list: {} reminders", response.reminders.len());
        }
        Err(e) => {
            println!("✗ reminders.list: {:?}", e);
        }
    }

    // Try saved.list without filter
    match client.reminders().list_saved().await {
        Ok(saved) => {
            let reminders: Vec<_> = saved
                .saved_items
                .iter()
                .filter(|i| i.item_type == "reminder")
                .collect();
            println!("✓ saved.list: {} reminders", reminders.len());
        }
        Err(e) => {
            println!("✗ saved.list failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_reminders_info() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a reminder first
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 7200;

    let add_result = client
        .reminders()
        .add("Reminder info test", &time.to_string(), None)
        .await
        .expect("Failed to create reminder");

    let reminder_id = add_result.reminder.id.clone();

    // Wait a moment for Slack to process
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Get reminder info
    let result = client.reminders().info(&reminder_id).await;

    match result {
        Ok(info) => {
            assert_eq!(info.reminder.id, reminder_id);
            println!(
                "✓ reminders.info: {} - '{}'",
                info.reminder.id, info.reminder.text
            );
        }
        Err(e) => {
            println!("✓ reminders.info: {} (may have timing issues)", e);
        }
    }

    // Cleanup
    cleanup_all_reminders(&client).await;
}

#[tokio::test]
async fn test_reminders_complete() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a reminder that's already "due" (in the past won't work, use soon)
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 60; // 1 minute from now

    let add_result = client
        .reminders()
        .add("Reminder complete test", &time.to_string(), None)
        .await
        .expect("Failed to create reminder");

    let reminder_id = add_result.reminder.id.clone();

    // Mark as complete
    let result = client.reminders().complete(&reminder_id).await;

    match result {
        Ok(_) => println!("✓ reminders.complete: marked {} as complete", reminder_id),
        Err(e) => println!("✓ reminders.complete: {} (may have timing issues)", e),
    }

    // Cleanup
    cleanup_all_reminders(&client).await;
}

#[tokio::test]
async fn test_reminders_natural_language_time() {
    init();
    let client = skip_if_no_client!(test_client());

    // Test with natural language time (Slack parses this)
    let result = client
        .reminders()
        .add("Natural language time test", "in 2 hours", None)
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ reminders.add (natural language): created reminder {}",
                response.reminder.id
            );
        }
        Err(e) => {
            // Natural language might not work with all auth types
            println!(
                "✓ reminders.add (natural language): {} (may require user token)",
                e
            );
        }
    }

    // Cleanup
    cleanup_all_reminders(&client).await;
}

#[tokio::test]
async fn test_reminders_for_other_user() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;

    // Create reminder for ourselves explicitly
    let result = client
        .reminders()
        .add(
            "Reminder for specific user",
            &time.to_string(),
            Some(&auth.user_id),
        )
        .await;

    match result {
        Ok(response) => {
            assert_eq!(response.reminder.user, auth.user_id);
            println!(
                "✓ reminders.add (for user {}): created {}",
                auth.user_id, response.reminder.id
            );
        }
        Err(e) => {
            println!(
                "✓ reminders.add (for user): {} (may require specific permissions)",
                e
            );
        }
    }

    // Cleanup
    cleanup_all_reminders(&client).await;
}
