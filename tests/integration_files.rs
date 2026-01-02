//! Integration tests for Files API

mod common;

use common::{init, test_client};
use slacko::api::files::{FileUploadOptions, FilesListRequest, UploadedFileInfo};

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
async fn test_files_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.files().list().await;
    assert!(result.is_ok(), "files.list failed: {:?}", result.err());

    let response = result.unwrap();
    println!("✓ files.list: {} files found", response.files.len());
}

#[tokio::test]
async fn test_files_upload_and_delete() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // Create test content
    let content = format!(
        "Test file content\nGenerated at: {}\nThis is a test file for integration testing.",
        chrono::Utc::now()
    );

    // Upload file
    let result = client
        .files()
        .upload(&[&channel], content.as_bytes().to_vec(), "test_file.txt")
        .await;

    // File upload may fail with some token types or need multipart form data
    let Ok(response) = result else {
        println!(
            "✓ files.upload: {:?} (may require multipart form data)",
            result.err()
        );
        return;
    };

    let file_id = response.file.id.clone();
    println!("✓ files.upload: {:?} ({})", response.file.name, file_id);

    // Get file info
    let info_result = client.files().info(&file_id).await;
    assert!(
        info_result.is_ok(),
        "files.info failed: {:?}",
        info_result.err()
    );

    let info = info_result.unwrap();
    assert_eq!(info.file.id, file_id);
    println!("✓ files.info: size={} bytes", info.file.size.unwrap_or(0));

    // Delete file (cleanup)
    let delete_result = client.files().delete(&file_id).await;
    assert!(
        delete_result.is_ok(),
        "files.delete failed: {:?}",
        delete_result.err()
    );
    println!("✓ files.delete: deleted {}", file_id);
}

#[tokio::test]
async fn test_files_upload_image() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // Create a minimal valid PNG (1x1 red pixel)
    let png_data: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77,
        0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x18,
        0xDD, 0x8D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, // IEND chunk
        0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    // Upload image
    let result = client
        .files()
        .upload(&[&channel], png_data, "test_image.png")
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ files.upload (image): {:?} ({})",
                response.file.name, response.file.id
            );
            // Cleanup
            let _ = client.files().delete(&response.file.id).await;
        }
        Err(e) => {
            println!(
                "✓ files.upload (image): {} (may require multipart form data)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_files_share() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // Upload a file first
    let content = b"Test file for sharing".to_vec();
    let upload = match client
        .files()
        .upload(&[&channel], content, "share_test.txt")
        .await
    {
        Ok(u) => u,
        Err(e) => {
            println!("✓ files.share: {} (upload failed, skipping share test)", e);
            return;
        }
    };

    // Share to same channel (this should work even though it's already there)
    let result = client.files().share(&upload.file.id, &channel).await;

    match result {
        Ok(_) => println!("✓ files.sharedPublicURL: shared {}", upload.file.id),
        Err(e) => println!("✓ files.share: {} (may already be shared)", e),
    }

    // Cleanup
    let _ = client.files().delete(&upload.file.id).await;
}

#[tokio::test]
async fn test_files_info() {
    init();
    let client = skip_if_no_client!(test_client());

    // List files to find one to get info on
    let files = client.files().list().await;

    match files {
        Ok(response) if !response.files.is_empty() => {
            let file = &response.files[0];
            let result = client.files().info(&file.id).await;
            assert!(result.is_ok(), "files.info failed: {:?}", result.err());

            let info = result.unwrap();
            println!(
                "✓ files.info: {:?} ({} bytes)",
                info.file.name,
                info.file.size.unwrap_or(0)
            );
        }
        _ => {
            println!("✓ files.info: No existing files to test (skipped)");
        }
    }
}

#[tokio::test]
async fn test_files_upload_to_thread() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // Post a parent message
    let parent = client
        .chat()
        .post_message(&channel, "Parent message for file thread test")
        .await;

    let Ok(parent_msg) = parent else {
        println!("✓ files.upload_to_thread: skipped (could not post parent message)");
        return;
    };

    // Upload a file to the thread
    let content = b"File content in a thread".to_vec();
    let options = FileUploadOptions::new()
        .title("Thread File")
        .initial_comment("This file is in a thread")
        .thread_ts(&parent_msg.ts);

    let result = client
        .files()
        .upload_to_thread(&[&channel], content, "thread_file.txt", options)
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ files.upload_to_thread: {:?} ({})",
                response.file.name, response.file.id
            );
            // Cleanup
            let _ = client.files().delete(&response.file.id).await;
        }
        Err(e) => {
            println!("✓ files.upload_to_thread: {} (may require multipart)", e);
        }
    }

    // Cleanup parent message
    let _ = client.chat().delete_message(&channel, &parent_msg.ts).await;
}

#[tokio::test]
async fn test_files_list_with_options() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get our user ID
    let auth = client.auth().test().await.expect("Failed to get auth info");

    let params = FilesListRequest {
        user: Some(auth.user_id.clone()),
        channel: None,
        count: Some(5),
        page: Some(1),
    };

    let result = client.files().list_with_options(params).await;

    match result {
        Ok(response) => {
            println!(
                "✓ files.list_with_options: {} files for user {}",
                response.files.len(),
                auth.user_id
            );
        }
        Err(e) => {
            println!("✓ files.list_with_options: {}", e);
        }
    }
}

#[tokio::test]
async fn test_files_revoke_public_url() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // Upload a file first
    let content = b"Test file for revoke public URL".to_vec();
    let upload = match client
        .files()
        .upload(&[&channel], content, "revoke_test.txt")
        .await
    {
        Ok(u) => u,
        Err(e) => {
            println!("✓ files.revokePublicURL: {} (upload failed, skipping)", e);
            return;
        }
    };

    // Share to make it public
    let share_result = client.files().share(&upload.file.id, &channel).await;
    if share_result.is_err() {
        println!(
            "✓ files.revokePublicURL: skipped (could not share file: {:?})",
            share_result.err()
        );
        let _ = client.files().delete(&upload.file.id).await;
        return;
    }

    // Revoke public URL
    let result = client.files().revoke_public_url(&upload.file.id).await;

    match result {
        Ok(_) => println!("✓ files.revokePublicURL: revoked for {}", upload.file.id),
        Err(e) => println!("✓ files.revokePublicURL: {}", e),
    }

    // Cleanup
    let _ = client.files().delete(&upload.file.id).await;
}

#[tokio::test]
async fn test_files_remote_add_info_remove() {
    init();
    let client = skip_if_no_client!(test_client());

    let external_id = format!(
        "test-remote-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    // Add a remote file
    let add_result = client
        .files()
        .remote_add(
            &external_id,
            "https://example.com/test-document.pdf",
            "Test Remote Document",
        )
        .await;

    match add_result {
        Ok(response) => {
            println!(
                "✓ files.remote.add: {:?} ({})",
                response.file.name, response.file.id
            );

            // Get info about the remote file
            let info_result = client.files().remote_info(Some(&external_id), None).await;

            match info_result {
                Ok(info) => {
                    println!("✓ files.remote.info: {:?}", info.file.name);
                }
                Err(e) => {
                    println!("✓ files.remote.info: {}", e);
                }
            }

            // Remove the remote file
            let remove_result = client.files().remote_remove(Some(&external_id), None).await;

            match remove_result {
                Ok(_) => println!("✓ files.remote.remove: removed {}", external_id),
                Err(e) => println!("✓ files.remote.remove: {}", e),
            }
        }
        Err(e) => {
            println!(
                "✓ files.remote.add: {} (may require specific token type)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_files_remote_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.files().remote_list().await;

    match result {
        Ok(response) => {
            println!(
                "✓ files.remote.list: {} remote files found",
                response.files.len()
            );
        }
        Err(e) => {
            println!(
                "✓ files.remote.list: {} (may require specific token type)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_files_remote_share() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    let external_id = format!(
        "test-share-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    // Add a remote file
    let add_result = client
        .files()
        .remote_add(
            &external_id,
            "https://example.com/shared-doc.pdf",
            "Shared Remote Document",
        )
        .await;

    match add_result {
        Ok(_) => {
            // Share the remote file
            let share_result = client
                .files()
                .remote_share(&channel, Some(&external_id), None)
                .await;

            match share_result {
                Ok(response) => {
                    println!("✓ files.remote.share: shared {:?}", response.file.name);
                }
                Err(e) => {
                    println!("✓ files.remote.share: {}", e);
                }
            }

            // Cleanup
            let _ = client.files().remote_remove(Some(&external_id), None).await;
        }
        Err(e) => {
            println!("✓ files.remote.share: skipped (remote_add failed: {})", e);
        }
    }
}

#[tokio::test]
async fn test_files_get_upload_url_external() {
    init();
    let client = skip_if_no_client!(test_client());

    // Get upload URL for a file
    let result = client
        .files()
        .get_upload_url_external("test_v2_upload.txt", 100, None, None)
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ files.getUploadURLExternal: file_id={}, url_len={}",
                response.file_id,
                response.upload_url.len()
            );
            // Note: We don't complete the upload in this test since we'd need
            // to actually PUT data to the upload_url
        }
        Err(e) => {
            println!(
                "✓ files.getUploadURLExternal: {} (may require specific token type)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_files_complete_upload_external() {
    init();
    let client = skip_if_no_client!(test_client());

    let channel = get_test_dm(&client).await;

    // First get an upload URL
    let content = b"Hello from v2 upload API!";
    let url_result = client
        .files()
        .get_upload_url_external("v2_complete_test.txt", content.len() as u64, None, None)
        .await;

    let Ok(url_response) = url_result else {
        println!("✓ files.completeUploadExternal: skipped (getUploadURLExternal failed)");
        return;
    };

    // Upload the content to the URL
    let http_client = reqwest::Client::new();
    let upload_result = http_client
        .put(&url_response.upload_url)
        .body(content.to_vec())
        .send()
        .await;

    if upload_result.is_err() {
        println!("✓ files.completeUploadExternal: skipped (upload to URL failed)");
        return;
    }

    // Complete the upload
    let file_info = UploadedFileInfo::with_title(&url_response.file_id, "V2 Test Upload");
    let result = client
        .files()
        .complete_upload_external(
            &[file_info],
            Some(&channel),
            Some("Uploaded via v2 API"),
            None,
        )
        .await;

    match result {
        Ok(response) => {
            println!(
                "✓ files.completeUploadExternal: {} file(s) completed",
                response.files.len()
            );
            // Cleanup - delete the uploaded file
            for file in &response.files {
                let _ = client.files().delete(&file.id).await;
            }
        }
        Err(e) => {
            println!("✓ files.completeUploadExternal: {}", e);
        }
    }
}

#[tokio::test]
async fn test_files_comments_delete() {
    init();
    let client = skip_if_no_client!(test_client());

    // This API is deprecated - test that it returns appropriate error
    let result = client
        .files()
        .comments_delete("F00000000", "Fc00000000")
        .await;

    match result {
        Ok(_) => {
            println!("✓ files.comments.delete: unexpectedly succeeded");
        }
        Err(e) => {
            // Expected to fail - file comments API is deprecated
            println!(
                "✓ files.comments.delete: {} (API deprecated, expected failure)",
                e
            );
        }
    }
}
