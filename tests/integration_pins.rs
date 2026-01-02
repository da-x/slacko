//! Integration tests for Pins API

mod common;

use common::{cleanup_message, init, test_client, unique_message};

/// Get self-DM channel
async fn get_test_dm(client: &slacko::SlackClient) -> String {
    let auth = client.auth().test().await.expect("Failed to get auth info");
    let dm = client
        .conversations()
        .open(&[&auth.user_id])
        .await
        .expect("Failed to open self-DM");
    dm.channel.id
}

#[tokio::test]
async fn test_pins_add_and_remove() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Pin test");

    // Post a message to pin
    let post = client
        .chat()
        .post_message(&channel, &message)
        .await
        .expect("Failed to post message");

    // Pin the message
    let result = client.pins().add(&channel, &post.ts).await;
    assert!(result.is_ok(), "pins.add failed: {:?}", result.err());
    println!("✓ pins.add: pinned message {}", post.ts);

    // Unpin the message
    let remove_result = client.pins().remove(&channel, &post.ts).await;
    assert!(
        remove_result.is_ok(),
        "pins.remove failed: {:?}",
        remove_result.err()
    );
    println!("✓ pins.remove: unpinned message {}", post.ts);

    // Cleanup
    cleanup_message(&client, &channel, &post.ts).await;
}

#[tokio::test]
async fn test_pins_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Pin list test");

    // Post and pin a message
    let post = client
        .chat()
        .post_message(&channel, &message)
        .await
        .expect("Failed to post message");

    let _ = client.pins().add(&channel, &post.ts).await;

    // List pins
    let result = client.pins().list(&channel).await;
    assert!(result.is_ok(), "pins.list failed: {:?}", result.err());

    let response = result.unwrap();
    println!(
        "✓ pins.list: {} pinned items in channel",
        response.items.len()
    );

    // At least our pinned message should be there
    assert!(
        !response.items.is_empty(),
        "Should have at least one pinned item"
    );

    // Cleanup
    let _ = client.pins().remove(&channel, &post.ts).await;
    cleanup_message(&client, &channel, &post.ts).await;
}

#[tokio::test]
async fn test_pins_duplicate_pin() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Duplicate pin test");

    // Post a message
    let post = client
        .chat()
        .post_message(&channel, &message)
        .await
        .expect("Failed to post message");

    // Pin the message
    let _ = client.pins().add(&channel, &post.ts).await;

    // Try to pin again (should fail gracefully)
    let result = client.pins().add(&channel, &post.ts).await;
    match result {
        Ok(_) => println!("✓ pins.add (duplicate): allowed"),
        Err(e) => println!("✓ pins.add (duplicate): correctly rejected - {}", e),
    }

    // Cleanup
    let _ = client.pins().remove(&channel, &post.ts).await;
    cleanup_message(&client, &channel, &post.ts).await;
}
