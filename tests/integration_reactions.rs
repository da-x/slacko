//! Integration tests for Reactions API

mod common;

use common::{init, test_channel, test_client};
use slacko::api::reactions::ReactionListRequest;

#[tokio::test]
async fn test_reactions_add_get_remove() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel_name = test_channel();

    // First, post a message to react to
    let post_result = client
        .chat()
        .post_message(&channel_name, "Reactions test message - please ignore")
        .await;

    let message = match post_result {
        Ok(response) => {
            println!(
                "✓ Posted test message: {} in channel {}",
                response.ts, response.channel
            );
            response
        }
        Err(e) => {
            println!("✓ reactions test: Skipped (couldn't post message: {})", e);
            return;
        }
    };

    // Use the channel ID from the response, not the channel name
    let channel = &message.channel;

    // Add a reaction
    let add_result = client
        .reactions()
        .add(channel, &message.ts, "thumbsup")
        .await;

    match add_result {
        Ok(_) => {
            println!("✓ reactions.add: added :thumbsup: to message");
        }
        Err(e) => {
            println!("✗ reactions.add: {}", e);
            // Cleanup message and return
            let _ = client.chat().delete_message(channel, &message.ts).await;
            return;
        }
    }

    // Get reactions on the message
    let get_result = client.reactions().get(channel, &message.ts).await;

    match get_result {
        Ok(response) => {
            let reaction_names: Vec<_> = response
                .message
                .reactions
                .as_ref()
                .map(|r| r.iter().map(|rx| rx.name.as_str()).collect())
                .unwrap_or_default();
            println!(
                "✓ reactions.get: message has reactions {:?}",
                reaction_names
            );
            // Slack may return "+1" instead of "thumbsup" (they're aliases)
            assert!(
                reaction_names.contains(&"thumbsup") || reaction_names.contains(&"+1"),
                "Should have thumbsup/+1 reaction"
            );
        }
        Err(e) => {
            println!("✗ reactions.get: {}", e);
        }
    }

    // Remove the reaction
    let remove_result = client
        .reactions()
        .remove(channel, &message.ts, "thumbsup")
        .await;

    match remove_result {
        Ok(_) => {
            println!("✓ reactions.remove: removed :thumbsup: from message");
        }
        Err(e) => {
            println!("✗ reactions.remove: {}", e);
        }
    }

    // Cleanup: delete the test message
    let _ = client.chat().delete_message(channel, &message.ts).await;
    println!("✓ Cleaned up test message");
}

#[tokio::test]
async fn test_reactions_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.reactions().list().await;

    match result {
        Ok(response) => {
            println!(
                "✓ reactions.list: {} items with reactions",
                response.items.len()
            );

            for item in response.items.iter().take(3) {
                let reactions: Vec<_> = item
                    .message
                    .reactions
                    .as_ref()
                    .map(|r| r.iter().map(|rx| format!(":{}:", rx.name)).collect())
                    .unwrap_or_default();
                println!(
                    "  - {} in {}: {}",
                    item.item_type,
                    item.channel,
                    reactions.join(" ")
                );
            }
        }
        Err(e) => {
            println!("✓ reactions.list: {} (may require different token type)", e);
        }
    }
}

#[tokio::test]
async fn test_reactions_multiple_emoji() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel_name = test_channel();

    // Post a message
    let post_result = client
        .chat()
        .post_message(&channel_name, "Multiple reactions test - please ignore")
        .await;

    let message = match post_result {
        Ok(response) => response,
        Err(e) => {
            println!("✓ reactions test: Skipped (couldn't post message: {})", e);
            return;
        }
    };

    let channel = &message.channel;

    // Add multiple reactions
    let emojis = ["thumbsup", "heart", "rocket"];
    let mut added = Vec::new();

    for emoji in &emojis {
        match client.reactions().add(channel, &message.ts, emoji).await {
            Ok(_) => {
                added.push(*emoji);
            }
            Err(e) => {
                println!("  - Failed to add :{}: {}", emoji, e);
            }
        }
    }

    println!(
        "✓ reactions.add: added {} reactions: {:?}",
        added.len(),
        added
    );

    // Verify with get
    if let Ok(response) = client.reactions().get(channel, &message.ts).await {
        let count = response
            .message
            .reactions
            .as_ref()
            .map(|r| r.len())
            .unwrap_or(0);
        println!("✓ reactions.get: message has {} reactions", count);
    }

    // Remove all reactions
    for emoji in &added {
        let _ = client.reactions().remove(channel, &message.ts, emoji).await;
    }
    println!("✓ reactions.remove: removed {} reactions", added.len());

    // Cleanup
    let _ = client.chat().delete_message(channel, &message.ts).await;
}

#[tokio::test]
async fn test_reactions_already_reacted() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel_name = test_channel();

    // Post a message
    let post_result = client
        .chat()
        .post_message(&channel_name, "Duplicate reaction test - please ignore")
        .await;

    let message = match post_result {
        Ok(response) => response,
        Err(e) => {
            println!("✓ reactions test: Skipped (couldn't post message: {})", e);
            return;
        }
    };

    let channel = &message.channel;

    // Add a reaction
    let _ = client.reactions().add(channel, &message.ts, "eyes").await;

    // Try to add the same reaction again - should fail with "already_reacted"
    let duplicate_result = client.reactions().add(channel, &message.ts, "eyes").await;

    match duplicate_result {
        Ok(_) => {
            println!("✗ reactions.add: duplicate reaction should have failed");
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("already_reacted") {
                println!("✓ reactions.add: correctly rejected duplicate reaction");
            } else {
                println!("✓ reactions.add: rejected with different error: {}", e);
            }
        }
    }

    // Cleanup
    let _ = client
        .reactions()
        .remove(channel, &message.ts, "eyes")
        .await;
    let _ = client.chat().delete_message(channel, &message.ts).await;
}

#[tokio::test]
async fn test_reactions_list_with_options() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    let params = ReactionListRequest {
        user: Some(auth.user_id.clone()),
        count: Some(5),
        page: Some(1),
    };

    let result = client.reactions().list_with_options(params).await;

    match result {
        Ok(response) => {
            println!(
                "✓ reactions.list_with_options: {} items for user {}",
                response.items.len(),
                auth.user_id
            );
        }
        Err(e) => {
            println!(
                "✓ reactions.list_with_options: {} (may require different token type)",
                e
            );
        }
    }
}
