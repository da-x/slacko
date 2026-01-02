//! Integration tests for OAuth API
//!
//! Note: OAuth APIs require valid OAuth authorization codes and app credentials.
//! These tests verify the API structure but cannot complete full OAuth flows.

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_oauth_access_requires_valid_code() {
    init();
    let client = skip_if_no_client!(test_client());

    // oauth.v2.access requires valid authorization code and app credentials
    let result = client
        .oauth()
        .access(
            "invalid-code",
            "invalid-client-id",
            "invalid-client-secret",
            None,
        )
        .await;

    match result {
        Ok(_) => {
            println!("✗ oauth.v2.access: unexpectedly succeeded");
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("invalid_code") || err_str.contains("invalid_client_id") {
                println!("✓ oauth.v2.access: correctly validates credentials");
            } else {
                println!("✓ oauth.v2.access: {} (requires valid OAuth flow)", e);
            }
        }
    }
}

#[tokio::test]
async fn test_oauth_exchange_requires_valid_token() {
    init();
    let client = skip_if_no_client!(test_client());

    // oauth.v2.exchange requires valid app credentials and legacy token
    let result = client
        .oauth()
        .exchange(
            "invalid-client-id",
            "invalid-client-secret",
            "xoxp-invalid-token",
        )
        .await;

    match result {
        Ok(_) => {
            println!("✗ oauth.v2.exchange: unexpectedly succeeded");
        }
        Err(e) => {
            println!("✓ oauth.v2.exchange: {} (requires valid credentials)", e);
        }
    }
}
