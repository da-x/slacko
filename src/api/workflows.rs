//! Workflows API
//!
//! Methods for Workflow Builder integrations.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Workflows API client
pub struct WorkflowsApi {
    client: SlackClient,
}

impl WorkflowsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Complete or advance a workflow step execution
    ///
    /// # Arguments
    ///
    /// * `workflow_step_execute_id` - Workflow step execution ID
    /// * `outputs` - Output values from the step
    pub async fn step_completed(
        &self,
        workflow_step_execute_id: &str,
        outputs: serde_json::Value,
    ) -> Result<WorkflowsStepCompletedResponse> {
        let params = WorkflowsStepCompletedRequest {
            workflow_step_execute_id: workflow_step_execute_id.to_string(),
            outputs: Some(outputs),
        };

        self.client.post("workflows.stepCompleted", &params).await
    }

    /// Mark a workflow step execution as failed
    ///
    /// # Arguments
    ///
    /// * `workflow_step_execute_id` - Workflow step execution ID
    /// * `error` - Error message
    pub async fn step_failed(
        &self,
        workflow_step_execute_id: &str,
        error: &str,
    ) -> Result<WorkflowsStepFailedResponse> {
        let params = WorkflowsStepFailedRequest {
            workflow_step_execute_id: workflow_step_execute_id.to_string(),
            error: error.to_string(),
        };

        self.client.post("workflows.stepFailed", &params).await
    }

    /// Update the configuration for a workflow step
    ///
    /// # Arguments
    ///
    /// * `workflow_step_edit_id` - Workflow step edit ID
    /// * `inputs` - Input configuration
    /// * `outputs` - Output configuration
    pub async fn update_step(
        &self,
        workflow_step_edit_id: &str,
        inputs: serde_json::Value,
        outputs: serde_json::Value,
    ) -> Result<WorkflowsUpdateStepResponse> {
        let params = WorkflowsUpdateStepRequest {
            workflow_step_edit_id: workflow_step_edit_id.to_string(),
            inputs: Some(inputs),
            outputs: Some(outputs),
        };

        self.client.post("workflows.updateStep", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct WorkflowsStepCompletedRequest {
    pub workflow_step_execute_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowsStepCompletedResponse {}

#[derive(Debug, Serialize)]
pub struct WorkflowsStepFailedRequest {
    pub workflow_step_execute_id: String,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowsStepFailedResponse {}

#[derive(Debug, Serialize)]
pub struct WorkflowsUpdateStepRequest {
    pub workflow_step_edit_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowsUpdateStepResponse {}
