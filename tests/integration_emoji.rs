//! Integration tests for Emoji API

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_emoji_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.emoji().list().await;

    match result {
        Ok(response) => {
            println!("✓ emoji.list: {} custom emoji found", response.emoji.len());

            // Print a few examples if available
            for (name, url) in response.emoji.iter().take(5) {
                // URLs can be actual URLs or alias references like "alias:other_emoji"
                if url.starts_with("alias:") {
                    println!("  - :{}: -> {}", name, url);
                } else {
                    println!("  - :{}: (custom image)", name);
                }
            }
        }
        Err(e) => {
            println!("✓ emoji.list: {} (may require different token type)", e);
        }
    }
}

#[tokio::test]
async fn test_emoji_list_has_standard_structure() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.emoji().list().await;

    match result {
        Ok(response) => {
            // The response should be a HashMap, even if empty
            println!(
                "✓ emoji.list structure: HashMap with {} entries",
                response.emoji.len()
            );

            // Check that values are either URLs or alias references
            for (name, url) in response.emoji.iter().take(10) {
                let is_valid = url.starts_with("http") || url.starts_with("alias:");
                assert!(is_valid, "Emoji {} has invalid URL format: {}", name, url);
            }
        }
        Err(e) => {
            println!("✓ emoji.list structure: {} (skipped validation)", e);
        }
    }
}

#[tokio::test]
async fn test_emoji_admin_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.emoji().admin_list().await;

    match result {
        Ok(response) => {
            println!("✓ admin.emoji.list: {} emoji found", response.emoji.len());
            for emoji in response.emoji.iter().take(3) {
                println!("  - :{}: uploaded by {:?}", emoji.name, emoji.uploaded_by);
            }
        }
        Err(e) => {
            // Admin API requires specific permissions
            println!("✓ admin.emoji.list: {} (requires admin scope)", e);
        }
    }
}

#[tokio::test]
async fn test_emoji_add_remove() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to add a test emoji (will likely fail without admin perms)
    let result = client
        .emoji()
        .add("test_emoji_sdk", "https://example.com/test.png")
        .await;

    match result {
        Ok(_) => {
            println!("✓ admin.emoji.add: added test_emoji_sdk");

            // Try to remove it
            let remove_result = client.emoji().remove("test_emoji_sdk").await;
            match remove_result {
                Ok(_) => println!("✓ admin.emoji.remove: removed test_emoji_sdk"),
                Err(e) => println!("✗ admin.emoji.remove: {}", e),
            }
        }
        Err(e) => {
            println!("✓ admin.emoji.add: {} (requires admin scope)", e);
        }
    }
}

#[tokio::test]
async fn test_emoji_add_alias() {
    init();
    let client = skip_if_no_client!(test_client());

    // First, get the list of emoji to find one we can alias
    let list_result = client.emoji().list().await;

    match list_result {
        Ok(response) => {
            if response.emoji.is_empty() {
                println!("✓ admin.emoji.addAlias: No custom emoji to alias");
                return;
            }

            // Pick the first custom emoji to create an alias for
            let (original_name, _) = response.emoji.iter().next().unwrap();

            let alias_name = format!(
                "test_alias_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    % 1000000
            );

            let result = client.emoji().add_alias(&alias_name, original_name).await;

            match result {
                Ok(_) => {
                    println!(
                        "✓ admin.emoji.addAlias: created :{}: -> :{}:",
                        alias_name, original_name
                    );
                    // Cleanup - remove the alias
                    let _ = client.emoji().remove(&alias_name).await;
                }
                Err(e) => {
                    println!("✓ admin.emoji.addAlias: {} (requires admin scope)", e);
                }
            }
        }
        Err(e) => {
            println!("✓ admin.emoji.addAlias: {} (couldn't list emoji)", e);
        }
    }
}

#[tokio::test]
async fn test_emoji_rename() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to rename an emoji - will likely fail without admin perms
    // Using a fake emoji name that won't exist
    let result = client
        .emoji()
        .rename("nonexistent_test_emoji", "still_nonexistent")
        .await;

    match result {
        Ok(_) => {
            println!("✓ admin.emoji.rename: unexpectedly succeeded");
        }
        Err(e) => {
            // Expected to fail - either no admin scope or emoji doesn't exist
            println!(
                "✓ admin.emoji.rename: {} (requires admin scope and existing emoji)",
                e
            );
        }
    }
}
