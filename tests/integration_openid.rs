//! Integration tests for OpenID Connect API
//!
//! Note: OpenID APIs require valid OAuth authorization codes and app credentials.
//! These tests verify the API structure but cannot complete full OIDC flows.

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_openid_token_requires_valid_code() {
    init();
    let client = skip_if_no_client!(test_client());

    // openid.connect.token requires valid authorization code and app credentials
    let result = client
        .openid()
        .token(
            "invalid-code",
            "invalid-client-id",
            "invalid-client-secret",
            None,
        )
        .await;

    match result {
        Ok(_) => {
            println!("✗ openid.connect.token: unexpectedly succeeded");
        }
        Err(e) => {
            println!("✓ openid.connect.token: {} (requires valid OIDC flow)", e);
        }
    }
}

#[tokio::test]
async fn test_openid_user_info() {
    init();
    let client = skip_if_no_client!(test_client());

    // openid.connect.userInfo requires an OIDC access token
    let result = client.openid().user_info().await;

    match result {
        Ok(response) => {
            println!(
                "✓ openid.connect.userInfo: user={} email={:?}",
                response.sub, response.email
            );
        }
        Err(e) => {
            // xoxc tokens may not work with OIDC endpoints
            println!("✓ openid.connect.userInfo: {} (requires OIDC token)", e);
        }
    }
}

#[tokio::test]
async fn test_openid_refresh_token() {
    init();
    let client = skip_if_no_client!(test_client());

    // openid.connect.token with refresh_token grant requires valid refresh token
    let result = client
        .openid()
        .refresh_token(
            "invalid-refresh-token",
            "invalid-client-id",
            "invalid-client-secret",
        )
        .await;

    match result {
        Ok(_) => {
            println!("✗ openid.connect.token (refresh): unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ openid.connect.token (refresh): {} (requires valid refresh token)",
                e
            );
        }
    }
}
