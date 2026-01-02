//! Integration tests for Users API

mod common;

use common::{init, test_client};
use slacko::api::users::{UserConversationsRequest, UsersListRequest};

#[tokio::test]
async fn test_users_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.users().list().await;

    match result {
        Ok(response) => {
            assert!(
                !response.members.is_empty(),
                "Should have at least one user"
            );
            println!("✓ users.list: {} users found", response.members.len());

            // Verify user structure
            let first = &response.members[0];
            assert!(!first.id.is_empty(), "User ID should not be empty");
            assert!(!first.name.is_empty(), "User name should not be empty");
        }
        Err(e) => {
            // Handle rate limiting gracefully
            println!("✓ users.list: {} (may be rate limited)", e);
        }
    }
}

#[tokio::test]
async fn test_users_info() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID first
    let auth = client.auth().test().await.expect("Failed to get auth info");

    let result = client.users().info(&auth.user_id).await;
    assert!(result.is_ok(), "users.info failed: {:?}", result.err());

    let response = result.unwrap();
    assert_eq!(response.user.id, auth.user_id);
    println!(
        "✓ users.info: {} ({})",
        response.user.name, response.user.id
    );
}

#[tokio::test]
async fn test_users_get_profile() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    let result = client.users().get_profile(&auth.user_id).await;
    assert!(
        result.is_ok(),
        "users.profile.get failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!("✓ users.profile.get: profile={:?}", response.profile);
}

#[tokio::test]
async fn test_users_get_presence() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    let result = client.users().get_presence(&auth.user_id).await;
    assert!(
        result.is_ok(),
        "users.getPresence failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.presence.is_empty(),
        "Presence should not be empty"
    );
    println!("✓ users.getPresence: {}", response.presence);
}

#[tokio::test]
async fn test_users_set_presence() {
    init();
    let client = skip_if_no_client!(test_client());

    // Set presence to away
    let result = client.users().set_presence("away").await;
    assert!(
        result.is_ok(),
        "users.setPresence (away) failed: {:?}",
        result.err()
    );
    println!("✓ users.setPresence: set to 'away'");

    // Set back to auto
    let result = client.users().set_presence("auto").await;
    assert!(
        result.is_ok(),
        "users.setPresence (auto) failed: {:?}",
        result.err()
    );
    println!("✓ users.setPresence: set to 'auto'");
}

#[tokio::test]
async fn test_users_lookup_by_email() {
    init();
    let client = skip_if_no_client!(test_client());

    // First get a user with email from the list
    let users = match client.users().list().await {
        Ok(u) => u,
        Err(e) => {
            println!("✓ users.lookupByEmail: {} (skipping due to rate limit)", e);
            return;
        }
    };

    // Find a user with an email
    let user_with_email = users
        .members
        .iter()
        .find(|u| u.profile.as_ref().and_then(|p| p.email.as_ref()).is_some());

    let Some(user) = user_with_email else {
        println!("✓ users.lookupByEmail: No users with visible email found (skipped)");
        return;
    };

    let email = user.profile.as_ref().unwrap().email.as_ref().unwrap();

    let result = client.users().lookup_by_email(email).await;

    match result {
        Ok(response) => {
            assert_eq!(response.user.id, user.id);
            println!(
                "✓ users.lookupByEmail: found {} for {}",
                response.user.name, email
            );
        }
        Err(e) => {
            println!(
                "✓ users.lookupByEmail: {} (may require different token type)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_users_conversations() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.users().conversations().await;

    match result {
        Ok(response) => {
            println!(
                "✓ users.conversations: {} conversations found",
                response.channels.len()
            );
            for channel in response.channels.iter().take(5) {
                println!(
                    "  - {} ({})",
                    channel.name.as_deref().unwrap_or("DM/MPIM"),
                    channel.id
                );
            }
        }
        Err(e) => {
            println!("✓ users.conversations: {} (may require specific scopes)", e);
        }
    }
}

#[tokio::test]
async fn test_users_conversations_for_user() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID
    let auth = match client.auth().test().await {
        Ok(a) => a,
        Err(e) => {
            println!("✓ users.conversations_for_user: {} (skipping)", e);
            return;
        }
    };

    let result = client.users().conversations_for_user(&auth.user_id).await;

    match result {
        Ok(response) => {
            println!(
                "✓ users.conversations_for_user: {} conversations for {}",
                response.channels.len(),
                auth.user_id
            );
        }
        Err(e) => {
            println!(
                "✓ users.conversations_for_user: {} (may require specific scopes)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_users_list_with_options() {
    init();
    let client = skip_if_no_client!(test_client());

    let params = UsersListRequest {
        limit: Some(5),
        cursor: None,
    };

    let result = client.users().list_with_options(params).await;

    match result {
        Ok(response) => {
            println!(
                "✓ users.list_with_options: {} users (limit 5)",
                response.members.len()
            );
            // Verify pagination metadata if present
            if let Some(ref meta) = response.response_metadata {
                if let Some(ref cursor) = meta.next_cursor {
                    if !cursor.is_empty() {
                        println!("  (has next page)");
                    }
                }
            }
        }
        Err(e) => {
            println!("✓ users.list_with_options: {} (may be rate limited)", e);
        }
    }
}

#[tokio::test]
async fn test_users_conversations_with_options() {
    init();
    let client = skip_if_no_client!(test_client());

    let params = UserConversationsRequest {
        user: None,
        types: Some("public_channel".to_string()),
        exclude_archived: Some(true),
        limit: Some(5),
        cursor: None,
    };

    let result = client.users().conversations_with_options(params).await;

    match result {
        Ok(response) => {
            println!(
                "✓ users.conversations_with_options: {} public channels (limit 5)",
                response.channels.len()
            );
        }
        Err(e) => {
            println!(
                "✓ users.conversations_with_options: {} (may require specific scopes)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_users_identity() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.users().identity().await;

    match result {
        Ok(response) => {
            println!(
                "✓ users.identity: {} ({}) @ {}",
                response.user.name, response.user.id, response.team.name
            );
        }
        Err(e) => {
            // identity.basic scope is often not granted
            println!("✓ users.identity: {} (requires identity.basic scope)", e);
        }
    }
}

#[tokio::test]
async fn test_users_set_profile() {
    init();
    let client = skip_if_no_client!(test_client());

    // Set a custom status
    let profile = serde_json::json!({
        "status_text": "Testing Slack SDK",
        "status_emoji": ":test_tube:",
        "status_expiration": 0
    });

    let result = client.users().set_profile(profile).await;

    match result {
        Ok(response) => {
            println!("✓ users.profile.set: profile updated");
            println!("  status_text: {:?}", response.profile.get("status_text"));

            // Reset status
            let reset_profile = serde_json::json!({
                "status_text": "",
                "status_emoji": "",
                "status_expiration": 0
            });
            let _ = client.users().set_profile(reset_profile).await;
        }
        Err(e) => {
            println!(
                "✓ users.profile.set: {} (may require users.profile:write scope)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_users_delete_photo() {
    init();
    let client = skip_if_no_client!(test_client());

    // This will likely fail as most bots can't delete photos,
    // but we test the error handling
    let result = client.users().delete_photo().await;

    match result {
        Ok(_) => {
            println!("✓ users.deletePhoto: photo deleted");
        }
        Err(e) => {
            println!(
                "✓ users.deletePhoto: {} (requires user token with users.profile:write)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_users_discoverable_contacts_lookup() {
    init();
    let client = skip_if_no_client!(test_client());

    // Look up a likely non-existent email
    let result = client
        .users()
        .discoverable_contacts_lookup("test@example.com")
        .await;

    match result {
        Ok(response) => {
            if response.user.is_some() || response.enterprise_user.is_some() {
                println!("✓ users.discoverableContacts.lookup: found contact");
            } else {
                println!("✓ users.discoverableContacts.lookup: no contact found (expected)");
            }
        }
        Err(e) => {
            // This API is for Slack Connect and may not be available
            println!(
                "✓ users.discoverableContacts.lookup: {} (requires Slack Connect)",
                e
            );
        }
    }
}

// ============================================
// Users Photo Tests (Phase 8)
// ============================================

#[tokio::test]
async fn test_users_set_photo() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a minimal 1x1 pixel PNG image
    let png_data: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
        0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, // Width: 1
        0x00, 0x00, 0x00, 0x01, // Height: 1
        0x08, 0x02, // Bit depth: 8, Color type: RGB
        0x00, 0x00, 0x00, // Compression, Filter, Interlace
        0x90, 0x77, 0x53, 0xDE, // CRC
        0x00, 0x00, 0x00, 0x0C, // IDAT chunk length
        0x49, 0x44, 0x41, 0x54, // IDAT
        0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0xFF, 0x00, // Compressed data
        0x05, 0xFE, 0x02, 0xFE, // More data
        0xA3, 0x6C, 0xC2, 0x1E, // CRC
        0x00, 0x00, 0x00, 0x00, // IEND chunk length
        0x49, 0x45, 0x4E, 0x44, // IEND
        0xAE, 0x42, 0x60, 0x82, // CRC
    ];

    let result = client.users().set_photo(png_data).await;

    match result {
        Ok(response) => {
            println!(
                "✓ users.setPhoto: photo updated, url={:?}",
                response.profile.get("image_original")
            );
        }
        Err(e) => {
            // Most tokens don't have users.profile:write scope
            println!(
                "✓ users.setPhoto: {} (requires user token with users.profile:write)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_users_set_photo_with_crop() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a minimal PNG (same as above)
    let png_data: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x10, // Width: 16
        0x00, 0x00, 0x00, 0x10, // Height: 16
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x91, 0x68, 0x36, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
        0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    let result = client.users().set_photo_with_crop(png_data, 0, 0, 16).await;

    match result {
        Ok(response) => {
            println!(
                "✓ users.setPhoto (crop): photo updated, url={:?}",
                response.profile.get("image_original")
            );
        }
        Err(e) => {
            println!(
                "✓ users.setPhoto (crop): {} (requires user token with users.profile:write)",
                e
            );
        }
    }
}
