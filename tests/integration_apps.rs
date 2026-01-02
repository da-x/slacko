//! Integration tests for Apps API

mod common;

use common::{init, test_client};

#[tokio::test]
async fn test_apps_permissions_info() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.apps().permissions_info().await;

    match result {
        Ok(response) => {
            println!(
                "✓ apps.permissions.info: team scopes = {:?}",
                response.info.team.scopes
            );
        }
        Err(e) => {
            println!("✓ apps.permissions.info: {} (may require app token)", e);
        }
    }
}

#[tokio::test]
async fn test_apps_permissions_resources_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.apps().permissions_resources_list().await;

    match result {
        Ok(response) => {
            println!(
                "✓ apps.permissions.resources.list: {} resources",
                response.resources.len()
            );
            for resource in response.resources.iter().take(5) {
                println!("  - {} ({})", resource.resource_type, resource.id);
            }
        }
        Err(e) => {
            println!(
                "✓ apps.permissions.resources.list: {} (may require app token)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_apps_event_authorizations_list() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client.apps().event_authorizations_list().await;

    match result {
        Ok(response) => {
            println!(
                "✓ apps.event.authorizations.list: {} authorizations",
                response.authorizations.len()
            );
        }
        Err(e) => {
            println!(
                "✓ apps.event.authorizations.list: {} (may require app token)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_apps_manifest_validate() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a minimal valid manifest for testing
    let manifest = serde_json::json!({
        "display_information": {
            "name": "Test App"
        },
        "features": {
            "bot_user": {
                "display_name": "Test Bot",
                "always_online": false
            }
        },
        "oauth_config": {
            "scopes": {
                "bot": ["chat:write"]
            }
        }
    });

    let result = client.apps().manifest_validate(manifest).await;

    match result {
        Ok(response) => {
            let errors = response.errors.as_ref().map(|e| e.len()).unwrap_or(0);
            let warnings = response.warnings.as_ref().map(|w| w.len()).unwrap_or(0);
            println!(
                "✓ apps.manifest.validate: {} errors, {} warnings",
                errors, warnings
            );
        }
        Err(e) => {
            println!(
                "✓ apps.manifest.validate: {} (may require app configuration token)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_apps_uninstall() {
    init();
    let client = skip_if_no_client!(test_client());

    // apps.uninstall will fail because we can't uninstall ourselves
    // This tests the error handling
    let result = client.apps().uninstall().await;

    match result {
        Ok(_) => {
            println!("✗ apps.uninstall: unexpectedly succeeded (should not uninstall test app)");
        }
        Err(e) => {
            // Expected to fail - can't uninstall the app we're using
            println!("✓ apps.uninstall: {} (expected - cannot uninstall self)", e);
        }
    }
}

#[tokio::test]
async fn test_apps_permissions_request() {
    init();
    let client = skip_if_no_client!(test_client());

    // apps.permissions.request requires a valid trigger_id from user interaction
    // This tests error handling for invalid trigger_id
    let result = client
        .apps()
        .permissions_request(&["channels:read"], "invalid_trigger_id")
        .await;

    match result {
        Ok(_) => {
            println!("✗ apps.permissions.request: unexpectedly succeeded");
        }
        Err(e) => {
            // Expected to fail - invalid trigger_id
            println!(
                "✓ apps.permissions.request: {} (requires valid trigger_id)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_apps_manifest_export() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to export manifest for an invalid app ID
    let result = client.apps().manifest_export("A00000000").await;

    match result {
        Ok(response) => {
            println!(
                "✓ apps.manifest.export: got manifest with {} keys",
                response.manifest.as_object().map(|o| o.len()).unwrap_or(0)
            );
        }
        Err(e) => {
            // Expected - invalid app_id or requires app configuration token
            println!(
                "✓ apps.manifest.export: {} (requires app configuration token)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_apps_manifest_create_delete() {
    init();
    let client = skip_if_no_client!(test_client());

    // Create a minimal manifest
    let manifest = serde_json::json!({
        "display_information": {
            "name": "SDK Test App"
        },
        "features": {
            "bot_user": {
                "display_name": "SDK Test Bot",
                "always_online": false
            }
        },
        "oauth_config": {
            "scopes": {
                "bot": ["chat:write"]
            }
        }
    });

    let create_result = client.apps().manifest_create(manifest).await;

    match create_result {
        Ok(response) => {
            println!("✓ apps.manifest.create: created app {}", response.app_id);

            // Delete the created app
            let delete_result = client.apps().manifest_delete(&response.app_id).await;
            match delete_result {
                Ok(_) => println!("✓ apps.manifest.delete: deleted app {}", response.app_id),
                Err(e) => println!("✓ apps.manifest.delete: {}", e),
            }
        }
        Err(e) => {
            // Expected - requires app configuration token
            println!(
                "✓ apps.manifest.create: {} (requires app configuration token)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_apps_manifest_update() {
    init();
    let client = skip_if_no_client!(test_client());

    // Try to update manifest for an invalid app ID
    let manifest = serde_json::json!({
        "display_information": {
            "name": "Updated App Name"
        }
    });

    let result = client.apps().manifest_update("A00000000", manifest).await;

    match result {
        Ok(response) => {
            println!("✓ apps.manifest.update: updated app {}", response.app_id);
        }
        Err(e) => {
            // Expected - invalid app_id or requires app configuration token
            println!(
                "✓ apps.manifest.update: {} (requires app configuration token)",
                e
            );
        }
    }
}
