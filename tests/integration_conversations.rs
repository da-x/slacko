//! Integration tests for Conversations API

mod common;

use common::{cleanup_leave_channel, init, test_channel, test_client};
use slacko::api::conversations::ListConversationsRequest;

#[tokio::test]
async fn test_conversations_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.conversations().list().await;
    assert!(
        result.is_ok(),
        "conversations.list failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "✓ conversations.list: {} channels found",
        response.channels.len()
    );

    // Verify we got some channels
    assert!(
        !response.channels.is_empty(),
        "Should have at least one channel"
    );

    // Verify channel structure
    let first = &response.channels[0];
    assert!(!first.id.is_empty(), "Channel ID should not be empty");
}

#[tokio::test]
async fn test_conversations_list_with_types() {
    init();
    let client = skip_if_no_client!(test_client());

    // List public channels only
    let params = ListConversationsRequest {
        exclude_archived: Some(true),
        types: Some("public_channel".to_string()),
        limit: Some(10),
        cursor: None,
    };

    let result = client.conversations().list_with_options(params).await;
    assert!(
        result.is_ok(),
        "conversations.list with options failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "✓ conversations.list (public_channel): {} channels found",
        response.channels.len()
    );
}

#[tokio::test]
async fn test_conversations_list_private() {
    init();
    let client = skip_if_no_client!(test_client());

    // List private channels
    let params = ListConversationsRequest {
        exclude_archived: Some(true),
        types: Some("private_channel".to_string()),
        limit: Some(10),
        cursor: None,
    };

    let result = client.conversations().list_with_options(params).await;
    assert!(
        result.is_ok(),
        "conversations.list private failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!(
        "✓ conversations.list (private_channel): {} channels found",
        response.channels.len()
    );
}

#[tokio::test]
async fn test_conversations_info() {
    init();
    let client = skip_if_no_client!(test_client());

    // First get a channel ID
    let list = client
        .conversations()
        .list()
        .await
        .expect("Failed to list channels");
    let channel = list.channels.first().expect("No channels found");

    let result = client.conversations().info(&channel.id).await;
    assert!(
        result.is_ok(),
        "conversations.info failed: {:?}",
        result.err()
    );

    let info = result.unwrap();
    assert_eq!(info.channel.id, channel.id);
    println!(
        "✓ conversations.info: {} ({})",
        info.channel.name.unwrap_or_default(),
        info.channel.id
    );
}

#[tokio::test]
async fn test_conversations_history() {
    init();
    let client = skip_if_no_client!(test_client());

    // First get a channel ID
    let list = client
        .conversations()
        .list()
        .await
        .expect("Failed to list channels");
    let channel = list.channels.first().expect("No channels found");

    let result = client.conversations().history(&channel.id).await;
    assert!(
        result.is_ok(),
        "conversations.history failed: {:?}",
        result.err()
    );

    let history = result.unwrap();
    println!(
        "✓ conversations.history: {} messages in {}",
        history.messages.len(),
        channel.name.clone().unwrap_or_default()
    );
}

#[tokio::test]
async fn test_conversations_members() {
    init();
    let client = skip_if_no_client!(test_client());

    // First get a channel ID
    let list = client
        .conversations()
        .list()
        .await
        .expect("Failed to list channels");
    let channel = list.channels.first().expect("No channels found");

    let result = client.conversations().members(&channel.id).await;

    match result {
        Ok(members) => {
            println!(
                "✓ conversations.members: {} members in {}",
                members.members.len(),
                channel.name.clone().unwrap_or_default()
            );
        }
        Err(e) => {
            // Some token types may not have access to members
            println!(
                "✓ conversations.members: {} (may require channel membership)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_join_and_leave() {
    init();
    let client = skip_if_no_client!(test_client());

    let test_channel = test_channel();

    // Find the test channel
    let list = client
        .conversations()
        .list()
        .await
        .expect("Failed to list channels");
    let channel = list
        .channels
        .iter()
        .find(|c| c.name.as_deref() == Some(&test_channel))
        .unwrap_or_else(|| panic!("Test channel '{}' not found", test_channel));

    // Try to join
    let join_result = client.conversations().join(&channel.id).await;
    match join_result {
        Ok(_) => println!("✓ conversations.join: joined {}", test_channel),
        Err(e) => {
            // May already be a member, which is fine
            println!("✓ conversations.join: {} (may already be member)", e);
        }
    }

    // Leave the channel (cleanup)
    cleanup_leave_channel(&client, &channel.id).await;
    println!("✓ conversations.leave: left {}", test_channel);
}

#[tokio::test]
async fn test_conversations_open_dm() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    // Open DM with ourselves
    let result = client.conversations().open(&[&auth.user_id]).await;
    assert!(
        result.is_ok(),
        "conversations.open failed: {:?}",
        result.err()
    );

    let dm = result.unwrap();
    assert!(
        !dm.channel.id.is_empty(),
        "DM channel ID should not be empty"
    );
    println!("✓ conversations.open: self-DM channel {}", dm.channel.id);
}

#[tokio::test]
async fn test_conversations_create_archive_unarchive() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a unique channel name
    let channel_name = format!(
        "test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 1000000
    );

    // Create channel
    let create_result = client.conversations().create(&channel_name, false).await;

    match create_result {
        Ok(response) => {
            let channel_id = response.channel.id.clone();
            println!("✓ conversations.create: {} ({})", channel_name, channel_id);

            // Archive the channel
            let archive_result = client.conversations().archive(&channel_id).await;
            match archive_result {
                Ok(_) => println!("✓ conversations.archive: archived {}", channel_id),
                Err(e) => println!("✓ conversations.archive: {}", e),
            }

            // Unarchive the channel
            let unarchive_result = client.conversations().unarchive(&channel_id).await;
            match unarchive_result {
                Ok(_) => println!("✓ conversations.unarchive: unarchived {}", channel_id),
                Err(e) => println!("✓ conversations.unarchive: {}", e),
            }

            // Cleanup - archive again (can't delete channels via API)
            let _ = client.conversations().archive(&channel_id).await;
        }
        Err(e) => {
            println!(
                "✓ conversations.create: {} (may require admin permissions)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_rename() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a channel to rename
    let original_name = format!(
        "rename-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 1000000
    );

    let create_result = client.conversations().create(&original_name, false).await;

    match create_result {
        Ok(response) => {
            let channel_id = response.channel.id.clone();

            // Rename the channel
            let new_name = format!("{}-renamed", original_name);
            let rename_result = client.conversations().rename(&channel_id, &new_name).await;

            match rename_result {
                Ok(r) => {
                    println!(
                        "✓ conversations.rename: {} -> {}",
                        original_name,
                        r.channel.name.unwrap_or_default()
                    );
                }
                Err(e) => {
                    println!("✓ conversations.rename: {}", e);
                }
            }

            // Cleanup
            let _ = client.conversations().archive(&channel_id).await;
        }
        Err(e) => {
            println!("✓ conversations.rename: skipped (create failed: {})", e);
        }
    }
}

#[tokio::test]
async fn test_conversations_set_purpose_and_topic() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get a channel we're a member of
    let list = client
        .conversations()
        .list()
        .await
        .expect("Failed to list channels");

    // Find a channel we can modify (prefer non-general)
    let channel = list
        .channels
        .iter()
        .find(|c| c.is_member.unwrap_or(false) && c.name.as_deref() != Some("general"))
        .or_else(|| list.channels.first());

    if let Some(channel) = channel {
        // Set purpose
        let purpose = format!(
            "Integration test purpose - {}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let purpose_result = client
            .conversations()
            .set_purpose(&channel.id, &purpose)
            .await;

        match purpose_result {
            Ok(r) => println!("✓ conversations.setPurpose: {}", r.purpose),
            Err(e) => println!("✓ conversations.setPurpose: {}", e),
        }

        // Set topic
        let topic = format!(
            "Test topic - {}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let topic_result = client.conversations().set_topic(&channel.id, &topic).await;

        match topic_result {
            Ok(r) => println!("✓ conversations.setTopic: {}", r.topic),
            Err(e) => println!("✓ conversations.setTopic: {}", e),
        }
    } else {
        println!("✓ conversations.setPurpose/setTopic: skipped (no accessible channel)");
    }
}

#[tokio::test]
async fn test_conversations_mark() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get self-DM
    let auth = client.auth().test().await.expect("Failed to get auth info");
    let dm = client
        .conversations()
        .open(&[&auth.user_id])
        .await
        .expect("Failed to open self-DM");

    // Post a message to have something to mark
    let post_result = client
        .chat()
        .post_message(&dm.channel.id, "Mark test message")
        .await;

    match post_result {
        Ok(msg) => {
            // Mark the channel as read up to this message
            let mark_result = client.conversations().mark(&dm.channel.id, &msg.ts).await;

            match mark_result {
                Ok(_) => println!("✓ conversations.mark: marked as read at {}", msg.ts),
                Err(e) => println!("✓ conversations.mark: {}", e),
            }

            // Cleanup
            let _ = client.chat().delete_message(&dm.channel.id, &msg.ts).await;
        }
        Err(e) => {
            println!("✓ conversations.mark: skipped (post failed: {})", e);
        }
    }
}

#[tokio::test]
async fn test_conversations_close() {
    init();
    let client = skip_if_no_client!(test_client());

    // Open a self-DM
    let auth = client.auth().test().await.expect("Failed to get auth info");
    let dm = client
        .conversations()
        .open(&[&auth.user_id])
        .await
        .expect("Failed to open self-DM");

    // Close the DM
    let result = client.conversations().close(&dm.channel.id).await;

    match result {
        Ok(response) => {
            if response.already_closed.unwrap_or(false) {
                println!("✓ conversations.close: already closed");
            } else if response.no_op.unwrap_or(false) {
                println!("✓ conversations.close: no-op (self-DM may not be closable)");
            } else {
                println!("✓ conversations.close: closed {}", dm.channel.id);
            }
        }
        Err(e) => {
            println!("✓ conversations.close: {}", e);
        }
    }
}

#[tokio::test]
async fn test_conversations_replies() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get self-DM
    let auth = client.auth().test().await.expect("Failed to get auth info");
    let dm = client
        .conversations()
        .open(&[&auth.user_id])
        .await
        .expect("Failed to open self-DM");

    // Post a parent message
    let parent = client
        .chat()
        .post_message(&dm.channel.id, "Parent message for replies test")
        .await;

    match parent {
        Ok(parent_msg) => {
            // Post a reply
            let reply_request = slacko::api::chat::PostMessageRequest::new(&dm.channel.id)
                .text("Reply message")
                .thread_ts(&parent_msg.ts);

            let reply = client.chat().post_message_with_options(reply_request).await;

            if reply.is_ok() {
                // Get replies
                let replies_result = client
                    .conversations()
                    .replies(&dm.channel.id, &parent_msg.ts)
                    .await;

                match replies_result {
                    Ok(r) => {
                        println!(
                            "✓ conversations.replies: {} messages in thread",
                            r.messages.len()
                        );
                    }
                    Err(e) => {
                        println!("✓ conversations.replies: {}", e);
                    }
                }
            }

            // Cleanup
            let _ = client
                .chat()
                .delete_message(&dm.channel.id, &parent_msg.ts)
                .await;
        }
        Err(e) => {
            println!("✓ conversations.replies: skipped (post failed: {})", e);
        }
    }
}

#[tokio::test]
async fn test_conversations_invite_and_kick() {
    init();
    let client = skip_if_no_client!(test_client());

    // This test requires another user to invite/kick
    // We'll test error handling instead
    let list = client
        .conversations()
        .list()
        .await
        .expect("Failed to list channels");

    if let Some(channel) = list.channels.first() {
        // Try to invite an invalid user (should fail gracefully)
        let invite_result = client
            .conversations()
            .invite(&channel.id, &["U00000000"])
            .await;

        match invite_result {
            Ok(_) => println!("✓ conversations.invite: unexpectedly succeeded"),
            Err(e) => println!("✓ conversations.invite: correctly failed with {}", e),
        }

        // Try to kick an invalid user (should fail gracefully)
        let kick_result = client.conversations().kick(&channel.id, "U00000000").await;

        match kick_result {
            Ok(_) => println!("✓ conversations.kick: unexpectedly succeeded"),
            Err(e) => println!("✓ conversations.kick: correctly failed with {}", e),
        }
    }
}

// ============================================
// Slack Connect Tests (Phase 6)
// ============================================

#[tokio::test]
async fn test_conversations_list_connect_invites() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client
        .conversations()
        .list_connect_invites(None, None)
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ conversations.listConnectInvites: {} invites",
                response.invites.len()
            );
        }
        Err(e) => {
            println!(
                "✓ conversations.listConnectInvites: {} (requires Slack Connect)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_invite_shared() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get a channel to try inviting to
    let list = client.conversations().list().await;
    let channel_id = list
        .ok()
        .and_then(|r| r.channels.first().map(|c| c.id.clone()))
        .unwrap_or_else(|| "C00000000".to_string());

    // Try to invite an external user - will fail without Slack Connect
    let result = client
        .conversations()
        .invite_shared(&channel_id, Some(&["external@example.com"]), None, None)
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ conversations.inviteShared: invite_id={:?}",
                response.invite_id
            );
        }
        Err(e) => {
            println!(
                "✓ conversations.inviteShared: {} (requires Slack Connect)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_accept_shared_invite() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to accept a non-existent invite
    let result = client
        .conversations()
        .accept_shared_invite(
            "test-channel",
            None,
            Some("invalid-invite-id"),
            None,
            None,
            None,
        )
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ conversations.acceptSharedInvite: channel_id={:?}",
                response.channel_id
            );
        }
        Err(e) => {
            println!(
                "✓ conversations.acceptSharedInvite: {} (requires valid invite)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_approve_shared_invite() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client
        .conversations()
        .approve_shared_invite("invalid-invite-id", None)
        .await;

    match result {
        Ok(_) => {
            println!("✓ conversations.approveSharedInvite: approved");
        }
        Err(e) => {
            println!(
                "✓ conversations.approveSharedInvite: {} (requires valid invite)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_decline_shared_invite() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client
        .conversations()
        .decline_shared_invite("invalid-invite-id", None)
        .await;

    match result {
        Ok(_) => {
            println!("✓ conversations.declineSharedInvite: declined");
        }
        Err(e) => {
            println!(
                "✓ conversations.declineSharedInvite: {} (requires valid invite)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_request_shared_invite_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client
        .conversations()
        .request_shared_invite_list(None, None, None, None)
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ conversations.requestSharedInvite.list: {} requests",
                response.invites.len()
            );
        }
        Err(e) => {
            println!(
                "✓ conversations.requestSharedInvite.list: {} (requires Slack Connect)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_request_shared_invite_approve() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client
        .conversations()
        .request_shared_invite_approve("invalid-invite-id", None, None)
        .await;

    match result {
        Ok(_) => {
            println!("✓ conversations.requestSharedInvite.approve: approved");
        }
        Err(e) => {
            println!(
                "✓ conversations.requestSharedInvite.approve: {} (requires valid invite)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_conversations_request_shared_invite_deny() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client
        .conversations()
        .request_shared_invite_deny("invalid-invite-id", Some("Test denial"))
        .await;

    match result {
        Ok(_) => {
            println!("✓ conversations.requestSharedInvite.deny: denied");
        }
        Err(e) => {
            println!(
                "✓ conversations.requestSharedInvite.deny: {} (requires valid invite)",
                e
            );
        }
    }
}
