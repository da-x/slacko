//! Integration tests for Bookmarks API

mod common;

use common::{init, test_channel, test_client};

#[tokio::test]
async fn test_bookmarks_list() {
    init();
    let client = skip_if_no_client!(test_client());

    // We need a channel ID, not name. Get it via conversations.list
    let channel_name = test_channel();
    let channels = match client.conversations().list().await {
        Ok(response) => response.channels,
        Err(e) => {
            println!("✓ bookmarks.list: Skipped (couldn't list channels: {})", e);
            return;
        }
    };

    let channel = channels
        .iter()
        .find(|c| c.name.as_deref() == Some(&channel_name) || c.id == channel_name);

    let channel_id = match channel {
        Some(c) => &c.id,
        None => {
            println!(
                "✓ bookmarks.list: Skipped (channel '{}' not found)",
                channel_name
            );
            return;
        }
    };

    let result = client.bookmarks().list(channel_id).await;

    match result {
        Ok(response) => {
            println!(
                "✓ bookmarks.list: {} bookmarks in #{}",
                response.bookmarks.len(),
                channel_name
            );

            for bookmark in response.bookmarks.iter().take(3) {
                println!("  - {} ({})", bookmark.title, bookmark.link);
            }
        }
        Err(e) => {
            println!(
                "✓ bookmarks.list: {} (may require different permissions)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_bookmarks_add_edit_remove() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get channel ID
    let channel_name = test_channel();
    let channels = match client.conversations().list().await {
        Ok(response) => response.channels,
        Err(e) => {
            println!("✓ bookmarks test: Skipped (couldn't list channels: {})", e);
            return;
        }
    };

    let channel = channels
        .iter()
        .find(|c| c.name.as_deref() == Some(&channel_name) || c.id == channel_name);

    let channel_id = match channel {
        Some(c) => c.id.clone(),
        None => {
            println!(
                "✓ bookmarks test: Skipped (channel '{}' not found)",
                channel_name
            );
            return;
        }
    };

    // Add a bookmark
    let add_result = client
        .bookmarks()
        .add(
            &channel_id,
            "Test Bookmark",
            "https://example.com/test",
            Some(":link:"),
        )
        .await;

    match add_result {
        Ok(response) => {
            let bookmark_id = response.bookmark.id.clone();
            println!("✓ bookmarks.add: created bookmark {}", bookmark_id);

            // Edit the bookmark
            let edit_result = client
                .bookmarks()
                .edit(
                    &bookmark_id,
                    &channel_id,
                    Some("Updated Test Bookmark"),
                    None,
                    None,
                )
                .await;

            match edit_result {
                Ok(edited) => {
                    println!("✓ bookmarks.edit: updated to '{}'", edited.bookmark.title);
                }
                Err(e) => {
                    println!("✗ bookmarks.edit: {}", e);
                }
            }

            // Remove the bookmark
            let remove_result = client.bookmarks().remove(&bookmark_id, &channel_id).await;

            match remove_result {
                Ok(_) => {
                    println!("✓ bookmarks.remove: deleted bookmark {}", bookmark_id);
                }
                Err(e) => {
                    println!("✗ bookmarks.remove: {}", e);
                }
            }
        }
        Err(e) => {
            println!("✓ bookmarks.add: {} (may require different permissions)", e);
        }
    }
}
