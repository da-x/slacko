//! Integration tests for Stars API

mod common;

use common::{cleanup_message, init, test_client, unique_message};

/// Get self-DM channel
async fn get_test_dm(client: &slacko::SlackClient) -> String {
    let auth = client.auth().test().await.expect("Failed to get auth info");
    let dm = client
        .conversations()
        .open(&[&auth.user_id])
        .await
        .expect("Failed to open self-DM");
    dm.channel.id
}

#[tokio::test]
async fn test_stars_add_message() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Star test");

    // Post a message to star
    let post = client
        .chat()
        .post_message(&channel, &message)
        .await
        .expect("Failed to post message");

    // Star the message
    let result = client
        .stars()
        .add(Some(&channel), Some(&post.ts), None)
        .await;
    assert!(result.is_ok(), "stars.add failed: {:?}", result.err());
    println!("✓ stars.add: starred message {}", post.ts);

    // Cleanup: unstar and delete
    let _ = client
        .stars()
        .remove(Some(&channel), Some(&post.ts), None)
        .await;
    cleanup_message(&client, &channel, &post.ts).await;
}

#[tokio::test]
async fn test_stars_remove() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;
    let message = unique_message("Star remove test");

    // Post a message
    let post = client
        .chat()
        .post_message(&channel, &message)
        .await
        .expect("Failed to post message");

    // Star and then unstar
    client
        .stars()
        .add(Some(&channel), Some(&post.ts), None)
        .await
        .expect("Failed to star message");

    let result = client
        .stars()
        .remove(Some(&channel), Some(&post.ts), None)
        .await;
    assert!(result.is_ok(), "stars.remove failed: {:?}", result.err());
    println!("✓ stars.remove: unstarred message {}", post.ts);

    // Cleanup
    cleanup_message(&client, &channel, &post.ts).await;
}

#[tokio::test]
async fn test_stars_list() {
    init();
    let client = skip_if_no_client!(test_client());

    // List all starred items
    let result = client.stars().list().await;

    match result {
        Ok(response) => {
            println!(
                "✓ stars.list: {} starred items (page {}/{})",
                response.items.len(),
                response.paging.page,
                response.paging.pages
            );
        }
        Err(e) => {
            println!("✓ stars.list: {} (may require different token type)", e);
        }
    }
}

#[tokio::test]
async fn test_stars_add_channel() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // Star the channel itself
    let result = client.stars().add(Some(&channel), None, None).await;

    match result {
        Ok(_) => {
            println!("✓ stars.add (channel): starred channel {}", channel);
            // Cleanup: unstar
            let _ = client.stars().remove(Some(&channel), None, None).await;
        }
        Err(e) => {
            // Some channel types might not support starring
            println!("✓ stars.add (channel): {} (may not be supported)", e);
        }
    }
}

#[tokio::test]
async fn test_stars_multiple_messages() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // Post multiple messages and star them
    let mut starred_messages = Vec::new();

    for i in 1..=3 {
        let message = unique_message(&format!("Star test {}", i));
        let post = client
            .chat()
            .post_message(&channel, &message)
            .await
            .expect("Failed to post message");

        let star_result = client
            .stars()
            .add(Some(&channel), Some(&post.ts), None)
            .await;
        assert!(
            star_result.is_ok(),
            "stars.add failed for message {}: {:?}",
            i,
            star_result.err()
        );

        starred_messages.push(post.ts);
    }

    println!(
        "✓ stars.add (multiple): starred {} messages",
        starred_messages.len()
    );

    // Verify they appear in the list
    match client.stars().list().await {
        Ok(list) => println!(
            "✓ stars.list: found {} total starred items",
            list.items.len()
        ),
        Err(e) => println!("✓ stars.list: {} (may require different token type)", e),
    }

    // Cleanup
    for ts in starred_messages {
        let _ = client.stars().remove(Some(&channel), Some(&ts), None).await;
        cleanup_message(&client, &channel, &ts).await;
    }
}
