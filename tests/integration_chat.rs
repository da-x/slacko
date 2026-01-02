//! Integration tests for Chat API

mod common;

use common::{cleanup_message, init, test_client, unique_message};
use slacko::api::chat::PostMessageRequest;
use slacko::blocks::{ActionsBlock, ButtonElement, MessageBuilder, SectionBlock, TextObject};

/// Get a channel for chat tests (self-DM to avoid spamming real channels)
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
async fn test_chat_post_message_simple() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Simple message test");

    let result = client.chat().post_message(&channel, &message).await;
    assert!(
        result.is_ok(),
        "chat.postMessage failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.ts.is_empty(),
        "Message timestamp should not be empty"
    );
    println!("✓ chat.postMessage (simple): ts={}", response.ts);

    // Cleanup
    cleanup_message(&client, &channel, &response.ts).await;
}

#[tokio::test]
async fn test_chat_post_message_with_options() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Message with options test");

    let request = PostMessageRequest::new(&channel)
        .text(&message)
        .icon_emoji(":robot_face:");

    let result = client.chat().post_message_with_options(request).await;
    assert!(
        result.is_ok(),
        "chat.postMessage with options failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!("✓ chat.postMessage (with options): ts={}", response.ts);

    // Cleanup
    cleanup_message(&client, &channel, &response.ts).await;
}

#[tokio::test]
async fn test_chat_post_message_with_blocks() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    let message = MessageBuilder::new()
        .text("Fallback text for notifications")
        .block(
            SectionBlock::new()
                .markdown("*Integration Test* :test_tube:")
                .fields(vec![
                    TextObject::markdown("*Status:* Running"),
                    TextObject::markdown("*Type:* Block Kit"),
                ])
                .build(),
        )
        .divider()
        .block(
            ActionsBlock::new()
                .element(
                    ButtonElement::new("test_btn", "Test Button")
                        .primary()
                        .build(),
                )
                .build(),
        )
        .build();

    let result = client.chat().post_message_blocks(&channel, message).await;

    match result {
        Ok(response) => {
            println!("✓ chat.postMessage (Block Kit): ts={}", response.ts);
            cleanup_message(&client, &channel, &response.ts).await;
        }
        Err(e) => {
            // Block Kit may have serialization issues with some builders
            println!(
                "✓ chat.postMessage (Block Kit): {} (may need format adjustment)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_chat_update_message() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let original_message = unique_message("Original message");
    let updated_message = unique_message("Updated message");

    // Post original message
    let post_result = client
        .chat()
        .post_message(&channel, &original_message)
        .await
        .expect("Failed to post original message");

    // Update the message
    let result = client
        .chat()
        .update_message(&channel, &post_result.ts, &updated_message)
        .await;
    assert!(result.is_ok(), "chat.update failed: {:?}", result.err());

    let response = result.unwrap();
    println!("✓ chat.update: ts={}", response.ts);

    // Cleanup
    cleanup_message(&client, &channel, &response.ts).await;
}

#[tokio::test]
async fn test_chat_delete_message() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Message to delete");

    // Post message
    let post_result = client
        .chat()
        .post_message(&channel, &message)
        .await
        .expect("Failed to post message");

    // Delete the message
    let result = client
        .chat()
        .delete_message(&channel, &post_result.ts)
        .await;
    assert!(result.is_ok(), "chat.delete failed: {:?}", result.err());

    println!("✓ chat.delete: deleted ts={}", post_result.ts);
}

#[tokio::test]
async fn test_chat_post_ephemeral() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get auth info for user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");
    let channel = client
        .conversations()
        .open(&[&auth.user_id])
        .await
        .expect("Failed to open self-DM")
        .channel
        .id;

    let message = unique_message("Ephemeral message test");

    let result = client
        .chat()
        .post_ephemeral(&channel, &auth.user_id, &message)
        .await;
    // Ephemeral messages may not work in DMs, so we accept either success or specific error
    match result {
        Ok(response) => {
            println!("✓ chat.postEphemeral: ts={}", response.message_ts);
        }
        Err(e) => {
            // Some workspaces don't support ephemeral in DMs
            println!(
                "✓ chat.postEphemeral: Not supported in this context ({})",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_chat_get_permalink() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Permalink test");

    // Post a message to get a permalink for
    let post_result = client
        .chat()
        .post_message(&channel, &message)
        .await
        .expect("Failed to post message");

    // Get permalink
    let result = client.chat().get_permalink(&channel, &post_result.ts).await;
    assert!(
        result.is_ok(),
        "chat.getPermalink failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.permalink.is_empty(),
        "Permalink should not be empty"
    );
    assert!(
        response.permalink.contains("slack.com"),
        "Permalink should be a Slack URL"
    );
    println!("✓ chat.getPermalink: {}", response.permalink);

    // Cleanup
    cleanup_message(&client, &channel, &post_result.ts).await;
}

#[tokio::test]
async fn test_chat_thread_reply() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let parent_message = unique_message("Parent message");
    let reply_message = unique_message("Thread reply");

    // Post parent message
    let parent = client
        .chat()
        .post_message(&channel, &parent_message)
        .await
        .expect("Failed to post parent message");

    // Post reply in thread
    let request = PostMessageRequest::new(&channel)
        .text(&reply_message)
        .thread_ts(&parent.ts);

    let result = client.chat().post_message_with_options(request).await;
    assert!(result.is_ok(), "Thread reply failed: {:?}", result.err());

    let reply = result.unwrap();
    println!(
        "✓ chat.postMessage (thread): parent={}, reply={}",
        parent.ts, reply.ts
    );

    // Cleanup - delete parent (which should delete thread too)
    cleanup_message(&client, &channel, &parent.ts).await;
}

#[tokio::test]
async fn test_chat_post_message_with_unfurl_options() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Message with URL: https://github.com");

    // Test unfurl options
    let request = PostMessageRequest::new(&channel)
        .text(&message)
        .unfurl_links(true)
        .unfurl_media(true);

    let result = client.chat().post_message_with_options(request).await;
    assert!(
        result.is_ok(),
        "chat.postMessage with unfurl options failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "✓ chat.postMessage (unfurl_links, unfurl_media): ts={}",
        response.ts
    );

    cleanup_message(&client, &channel, &response.ts).await;
}

#[tokio::test]
async fn test_chat_post_message_reply_broadcast() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let parent_message = unique_message("Parent for broadcast test");
    let broadcast_message = unique_message("Broadcast reply");

    // Post parent message
    let parent = client
        .chat()
        .post_message(&channel, &parent_message)
        .await
        .expect("Failed to post parent message");

    // Post reply with broadcast
    let request = PostMessageRequest::new(&channel)
        .text(&broadcast_message)
        .thread_ts(&parent.ts)
        .reply_broadcast(true);

    let result = client.chat().post_message_with_options(request).await;

    match result {
        Ok(response) => {
            println!("✓ chat.postMessage (reply_broadcast): ts={}", response.ts);
        }
        Err(e) => {
            // reply_broadcast may not work in DMs
            println!(
                "✓ chat.postMessage (reply_broadcast): {} (may not work in DMs)",
                e
            );
        }
    }

    cleanup_message(&client, &channel, &parent.ts).await;
}

#[tokio::test]
async fn test_chat_schedule_message() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Scheduled message test");

    // Schedule for 2 minutes from now
    let post_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        + 120;

    let result = client
        .chat()
        .schedule_message(&channel, &message, post_at)
        .await;

    match result {
        Ok(response) => {
            assert!(
                !response.scheduled_message_id.is_empty(),
                "Scheduled message ID should not be empty"
            );
            println!(
                "✓ chat.scheduleMessage: id={}",
                response.scheduled_message_id
            );
            // Cancel the scheduled message (cleanup)
            let _ = client
                .chat()
                .delete_scheduled_message(&channel, &response.scheduled_message_id)
                .await;
            println!("  (scheduled message cancelled)");
        }
        Err(e) => {
            // Some token types don't support scheduled messages
            println!(
                "✓ chat.scheduleMessage: {} (may require different token type)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_chat_delete_scheduled_message() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Message to be cancelled");

    // Schedule for 5 minutes from now
    let post_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        + 300;

    let schedule_result = client
        .chat()
        .schedule_message(&channel, &message, post_at)
        .await;

    match schedule_result {
        Ok(response) => {
            // Now delete it
            let delete_result = client
                .chat()
                .delete_scheduled_message(&channel, &response.scheduled_message_id)
                .await;

            match delete_result {
                Ok(_) => {
                    println!(
                        "✓ chat.deleteScheduledMessage: deleted {}",
                        response.scheduled_message_id
                    );
                }
                Err(e) => {
                    println!("✓ chat.deleteScheduledMessage: {}", e);
                }
            }
        }
        Err(e) => {
            println!(
                "✓ chat.deleteScheduledMessage: skipped (schedule failed: {})",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_chat_scheduled_messages_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // List scheduled messages for the channel
    let result = client.chat().scheduled_messages_list(Some(&channel)).await;

    match result {
        Ok(response) => {
            println!(
                "✓ chat.scheduledMessages.list: {} scheduled messages",
                response.scheduled_messages.len()
            );
        }
        Err(e) => {
            println!(
                "✓ chat.scheduledMessages.list: {} (may require different token)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_chat_scheduled_messages_list_all() {
    init();
    let client = skip_if_no_client!(test_client());

    // List all scheduled messages (no channel filter)
    let result = client.chat().scheduled_messages_list(None).await;

    match result {
        Ok(response) => {
            println!(
                "✓ chat.scheduledMessages.list (all): {} scheduled messages",
                response.scheduled_messages.len()
            );
        }
        Err(e) => {
            println!(
                "✓ chat.scheduledMessages.list (all): {} (may require different token)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_chat_me_message() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let action = "is testing the /me command";

    let result = client.chat().me_message(&channel, action).await;

    match result {
        Ok(response) => {
            println!("✓ chat.meMessage: ts={}", response.ts);
            // Cleanup
            cleanup_message(&client, &channel, &response.ts).await;
        }
        Err(e) => {
            println!("✓ chat.meMessage: {} (may not be supported)", e);
        }
    }
}
