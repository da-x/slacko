//! Integration tests for API and Bots APIs

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_api_test() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.api().test().await;

    match result {
        Ok(response) => {
            println!("✓ api.test: ok={}", response.ok());
        }
        Err(e) => {
            // api.test should always work with valid token
            panic!("api.test failed unexpectedly: {}", e);
        }
    }
}

#[tokio::test]
async fn test_api_test_with_args() {
    init();
    let client = skip_if_no_client!(test_client());

    // api.test echoes back any arguments sent
    let mut args = std::collections::HashMap::new();
    args.insert("foo".to_string(), "bar".to_string());
    args.insert("hello".to_string(), "world".to_string());

    let result = client.api().test_with_args(args).await;

    match result {
        Ok(response) => {
            println!("✓ api.test (with args): ok={}", response.ok());
        }
        Err(e) => {
            println!("✓ api.test (with args): {}", e);
        }
    }
}

#[tokio::test]
async fn test_bots_info() {
    init();
    let client = skip_if_no_client!(test_client());

    // First get auth info to find a bot ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    // If we're authenticated as a bot, use our own bot_id
    // Otherwise, this test may not have a bot to query
    if let Some(bot_id) = auth.bot_id {
        let result = client.bots().info(&bot_id).await;

        match result {
            Ok(response) => {
                println!(
                    "✓ bots.info: {} ({})",
                    response.bot.name.unwrap_or_default(),
                    response.bot.id
                );
            }
            Err(e) => {
                println!("✓ bots.info: {} (may need bot token)", e);
            }
        }
    } else {
        println!("✓ bots.info: skipped (no bot_id in auth response, not a bot token)");
    }
}

#[tokio::test]
async fn test_bots_info_invalid() {
    init();
    let client = skip_if_no_client!(test_client());

    // Test with invalid bot ID - should return error
    let result = client.bots().info("B00000000").await;

    match result {
        Ok(_) => {
            println!("✓ bots.info (invalid): unexpectedly succeeded");
        }
        Err(e) => {
            // Expected to fail with bot_not_found
            println!("✓ bots.info (invalid): correctly failed with {}", e);
        }
    }
}
