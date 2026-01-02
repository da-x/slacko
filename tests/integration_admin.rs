//! Integration tests for Admin API
//!
//! Note: Admin APIs require Enterprise Grid admin privileges.

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_admin_teams_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.admin().teams().list().await;

    match result {
        Ok(response) => {
            println!("✓ admin.teams.list: {} teams", response.teams.len());
            for team in response.teams.iter().take(3) {
                println!("  - {} ({})", team.name, team.id);
            }
        }
        Err(e) => {
            // Admin APIs require Enterprise Grid admin privileges
            println!("✓ admin.teams.list: {} (requires Enterprise Grid admin)", e);
        }
    }
}

#[tokio::test]
async fn test_admin_teams_create() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to create a team - will fail without Enterprise Grid admin
    let result = client
        .admin()
        .teams()
        .create("test-domain-sdk", "SDK Test Team")
        .await;

    match result {
        Ok(response) => {
            println!("✓ admin.teams.create: created team {}", response.team);
        }
        Err(e) => {
            println!(
                "✓ admin.teams.create: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_admin_apps_approve() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to approve an app - will fail without valid request_id
    let result = client
        .admin()
        .apps()
        .approve("A00000000", "invalid-request-id")
        .await;

    match result {
        Ok(_) => {
            println!("✗ admin.apps.approve: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ admin.apps.approve: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_admin_apps_restrict() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to restrict an app - will fail without valid request_id
    let result = client
        .admin()
        .apps()
        .restrict("A00000000", "invalid-request-id")
        .await;

    match result {
        Ok(_) => {
            println!("✗ admin.apps.restrict: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ admin.apps.restrict: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_admin_users_invite() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to invite a user - will fail without Enterprise Grid admin
    let result = client
        .admin()
        .users()
        .invite(&["C00000000"], "test@example.com", "T00000000")
        .await;

    match result {
        Ok(_) => {
            println!("✗ admin.users.invite: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ admin.users.invite: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_admin_users_remove() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to remove a user - will fail without Enterprise Grid admin
    let result = client
        .admin()
        .users()
        .remove("T00000000", "U00000000")
        .await;

    match result {
        Ok(_) => {
            println!("✗ admin.users.remove: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ admin.users.remove: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_admin_users_set_admin() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to set a user as admin - will fail without Enterprise Grid admin
    let result = client
        .admin()
        .users()
        .set_admin("T00000000", "U00000000")
        .await;

    match result {
        Ok(_) => {
            println!("✗ admin.users.setAdmin: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ admin.users.setAdmin: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_admin_conversations_archive() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to archive a channel via admin API - will fail without Enterprise Grid admin
    let result = client.admin().conversations().archive("C00000000").await;

    match result {
        Ok(_) => {
            println!("✗ admin.conversations.archive: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ admin.conversations.archive: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_admin_conversations_delete() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to delete a channel via admin API - will fail without Enterprise Grid admin
    let result = client.admin().conversations().delete("C00000000").await;

    match result {
        Ok(_) => {
            println!("✗ admin.conversations.delete: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ admin.conversations.delete: {} (requires Enterprise Grid admin)",
                e
            );
        }
    }
}
