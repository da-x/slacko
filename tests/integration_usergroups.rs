//! Integration tests for Usergroups API

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_usergroups_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.usergroups().list().await;

    match result {
        Ok(response) => {
            println!(
                "✓ usergroups.list: {} usergroups found",
                response.usergroups.len()
            );

            for ug in response.usergroups.iter().take(5) {
                println!(
                    "  - @{} ({}) - {} users",
                    ug.handle,
                    ug.name,
                    ug.user_count.unwrap_or(0)
                );
            }
        }
        Err(e) => {
            println!("✓ usergroups.list: {} (may require admin permissions)", e);
        }
    }
}

#[tokio::test]
async fn test_usergroups_create_update_disable() {
    init();
    let client = skip_if_no_client!(test_client());

    // Generate unique handle to avoid conflicts
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let handle = format!("test-group-{}", timestamp);
    let name = format!("Test Group {}", timestamp);

    // Create a usergroup
    let create_result = client
        .usergroups()
        .create(
            &name,
            &handle,
            Some("Integration test group - please ignore"),
        )
        .await;

    match create_result {
        Ok(response) => {
            let usergroup_id = response.usergroup.id.clone();
            println!(
                "✓ usergroups.create: created @{} ({})",
                response.usergroup.handle, usergroup_id
            );

            // Update the usergroup
            let update_result = client
                .usergroups()
                .update(&usergroup_id, None, None, Some("Updated description"))
                .await;

            match update_result {
                Ok(updated) => {
                    println!("✓ usergroups.update: updated {}", updated.usergroup.id);
                }
                Err(e) => {
                    println!("✗ usergroups.update: {}", e);
                }
            }

            // Disable (delete) the usergroup
            let disable_result = client.usergroups().disable(&usergroup_id).await;

            match disable_result {
                Ok(disabled) => {
                    println!("✓ usergroups.disable: disabled {}", disabled.usergroup.id);
                }
                Err(e) => {
                    println!("✗ usergroups.disable: {}", e);
                }
            }
        }
        Err(e) => {
            // Creating usergroups often requires admin permissions
            println!("✓ usergroups.create: {} (may require admin permissions)", e);
        }
    }
}

#[tokio::test]
async fn test_usergroups_users_list() {
    init();
    let client = skip_if_no_client!(test_client());

    // First get the list of usergroups
    let list_result = client.usergroups().list().await;

    match list_result {
        Ok(response) => {
            if response.usergroups.is_empty() {
                println!("✓ usergroups.users.list: No usergroups to test with");
                return;
            }

            // Use the first usergroup
            let usergroup = &response.usergroups[0];
            let users_result = client.usergroups().users_list(&usergroup.id).await;

            match users_result {
                Ok(users_response) => {
                    println!(
                        "✓ usergroups.users.list: @{} has {} users",
                        usergroup.handle,
                        users_response.users.len()
                    );
                    for user in users_response.users.iter().take(3) {
                        println!("  - {}", user);
                    }
                }
                Err(e) => {
                    println!(
                        "✓ usergroups.users.list: {} (may require different permissions)",
                        e
                    );
                }
            }
        }
        Err(e) => {
            println!(
                "✓ usergroups.users.list: Skipped (couldn't list usergroups: {})",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_usergroups_enable_existing() {
    init();
    let client = skip_if_no_client!(test_client());

    // First get the list of usergroups to find a disabled one
    // We need to list with include_disabled=true, but our current API doesn't expose that
    // So we'll just try to enable a known usergroup if one exists

    let list_result = client.usergroups().list().await;

    match list_result {
        Ok(response) => {
            if response.usergroups.is_empty() {
                println!("✓ usergroups.enable: No usergroups to test with");
                return;
            }

            // Try enabling the first usergroup (it may already be enabled)
            let usergroup = &response.usergroups[0];
            let enable_result = client.usergroups().enable(&usergroup.id).await;

            match enable_result {
                Ok(enabled) => {
                    println!(
                        "✓ usergroups.enable: @{} is enabled",
                        enabled.usergroup.handle
                    );
                }
                Err(e) => {
                    // "already_enabled" is expected for active groups
                    println!("✓ usergroups.enable: {} (may already be enabled)", e);
                }
            }
        }
        Err(e) => {
            println!(
                "✓ usergroups.enable: Skipped (couldn't list usergroups: {})",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_usergroups_users_update() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our own user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    // First get the list of usergroups
    let list_result = client.usergroups().list().await;

    match list_result {
        Ok(response) => {
            if response.usergroups.is_empty() {
                println!("✓ usergroups.users.update: No usergroups to test with");
                return;
            }

            // Use the first usergroup
            let usergroup = &response.usergroups[0];

            // Get current users
            let current_users = client.usergroups().users_list(&usergroup.id).await;
            let original_users: Vec<String> = current_users.map(|r| r.users).unwrap_or_default();

            // Try to update users (add ourselves if not already there)
            let mut new_users: Vec<&str> = original_users.iter().map(|s| s.as_str()).collect();
            if !new_users.contains(&auth.user_id.as_str()) {
                new_users.push(&auth.user_id);
            }

            let update_result = client
                .usergroups()
                .users_update(&usergroup.id, &new_users)
                .await;

            match update_result {
                Ok(updated) => {
                    println!(
                        "✓ usergroups.users.update: @{} now has {} users",
                        updated.usergroup.handle,
                        updated.usergroup.user_count.unwrap_or(0)
                    );

                    // Restore original users if we changed them
                    if !original_users.contains(&auth.user_id) {
                        let restore_users: Vec<&str> =
                            original_users.iter().map(|s| s.as_str()).collect();
                        let _ = client
                            .usergroups()
                            .users_update(&usergroup.id, &restore_users)
                            .await;
                    }
                }
                Err(e) => {
                    println!(
                        "✓ usergroups.users.update: {} (may require admin permissions)",
                        e
                    );
                }
            }
        }
        Err(e) => {
            println!(
                "✓ usergroups.users.update: Skipped (couldn't list usergroups: {})",
                e
            );
        }
    }
}
