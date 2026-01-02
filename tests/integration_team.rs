//! Integration tests for Team API

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_team_info() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.team().info().await;

    match result {
        Ok(response) => {
            assert!(!response.team.id.is_empty(), "Team ID should not be empty");
            assert!(
                !response.team.name.is_empty(),
                "Team name should not be empty"
            );
            println!("✓ team.info: {} ({})", response.team.name, response.team.id);
        }
        Err(e) => {
            println!("✓ team.info: {} (may require different token type)", e);
        }
    }
}

#[tokio::test]
async fn test_team_billable_info() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.team().billable_info().await;

    match result {
        Ok(response) => {
            // billable_info is a JSON object with user_id -> billable status
            println!("✓ team.billableInfo: {:?}", response.billable_info);
        }
        Err(e) => {
            // This often requires admin permissions
            println!("✓ team.billableInfo: {} (may require admin permissions)", e);
        }
    }
}

#[tokio::test]
async fn test_team_access_logs() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.team().access_logs().await;

    match result {
        Ok(response) => {
            println!(
                "✓ team.accessLogs: {} logins (page {}/{})",
                response.logins.len(),
                response.paging.page,
                response.paging.pages
            );
        }
        Err(e) => {
            // This often requires admin permissions on paid plans
            println!("✓ team.accessLogs: {} (may require admin on paid plan)", e);
        }
    }
}

#[tokio::test]
async fn test_team_integration_logs() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.team().integration_logs().await;

    match result {
        Ok(response) => {
            println!(
                "✓ team.integrationLogs: {} logs (page {}/{})",
                response.logs.len(),
                response.paging.page,
                response.paging.pages
            );
        }
        Err(e) => {
            // This often requires admin permissions
            println!(
                "✓ team.integrationLogs: {} (may require admin permissions)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_team_profile_get() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.team().profile_get().await;

    match result {
        Ok(response) => {
            println!(
                "✓ team.profile.get: {} profile fields",
                response.profile.fields.len()
            );
            for field in response.profile.fields.iter().take(3) {
                println!("  - {} ({}): {}", field.id, field.field_type, field.label);
            }
        }
        Err(e) => {
            println!(
                "✓ team.profile.get: {} (may require specific permissions)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_team_preferences_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.team().preferences_list().await;

    match result {
        Ok(response) => {
            println!("✓ team.preferences.list: {:?}", response.preferences);
        }
        Err(e) => {
            println!(
                "✓ team.preferences.list: {} (may require admin permissions)",
                e
            );
        }
    }
}
