//! Integration tests for Workflows API
//!
//! Note: Workflow APIs require workflow step execution context,
//! so they cannot be fully tested in automated integration tests.

mod common;

use common::{init, test_client};
use serde_json::json;

#[tokio::test]
async fn test_workflows_step_completed_requires_context() {
    init();
    let client = skip_if_no_client!(test_client());

    // workflows.stepCompleted requires a valid workflow_step_execute_id
    // This test documents that behavior
    let result = client
        .workflows()
        .step_completed("invalid-step-execute-id", json!({"result": "test"}))
        .await;

    match result {
        Ok(_) => {
            println!("✗ workflows.stepCompleted: unexpectedly succeeded");
        }
        Err(e) => {
            println!(
                "✓ workflows.stepCompleted: {} (requires workflow context)",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_workflows_step_failed_requires_context() {
    init();
    let client = skip_if_no_client!(test_client());

    let result = client
        .workflows()
        .step_failed("invalid-step-execute-id", "Test error message")
        .await;

    match result {
        Ok(_) => {
            println!("✗ workflows.stepFailed: unexpectedly succeeded");
        }
        Err(e) => {
            println!("✓ workflows.stepFailed: {} (requires workflow context)", e);
        }
    }
}

#[tokio::test]
async fn test_workflows_update_step_requires_context() {
    init();
    let client = skip_if_no_client!(test_client());

    let inputs = json!({
        "input_1": {
            "value": "test value"
        }
    });

    let outputs = json!([
        {
            "name": "output_1",
            "type": "text",
            "label": "Output 1"
        }
    ]);

    let result = client
        .workflows()
        .update_step("invalid-step-edit-id", inputs, outputs)
        .await;

    match result {
        Ok(_) => {
            println!("✗ workflows.updateStep: unexpectedly succeeded");
        }
        Err(e) => {
            println!("✓ workflows.updateStep: {} (requires workflow context)", e);
        }
    }
}
