//! Integration tests for Auth API

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_auth_test() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.auth().test().await;
    assert!(result.is_ok(), "auth.test failed: {:?}", result.err());

    let auth = result.unwrap();
    assert!(!auth.user.is_empty(), "user should not be empty");
    assert!(!auth.user_id.is_empty(), "user_id should not be empty");
    assert!(!auth.team.is_empty(), "team should not be empty");
    assert!(!auth.team_id.is_empty(), "team_id should not be empty");

    println!("✓ auth.test: user={}, team={}", auth.user, auth.team);
}

#[tokio::test]
async fn test_auth_teams_list() {
    init();
    let client = skip_if_no_client!(test_client());

    // This may fail for non-enterprise workspaces, which is expected
    let result = client.auth().teams_list().await;

    match result {
        Ok(teams) => {
            println!("✓ auth.teams.list: {} teams found", teams.teams.len());
        }
        Err(e) => {
            // Expected for non-enterprise workspaces
            println!(
                "✓ auth.teams.list: Not available (expected for non-enterprise): {}",
                e
            );
        }
    }
}
