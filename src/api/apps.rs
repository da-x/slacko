//! Apps API
//!
//! Methods for managing app configurations and permissions.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Apps API client
pub struct AppsApi {
    client: SlackClient,
}

impl AppsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Get information about an app's event subscriptions
    pub async fn event_authorizations_list(&self) -> Result<AppsEventAuthorizationsListResponse> {
        let params = AppsEventAuthorizationsListRequest {
            cursor: None,
            limit: Some(100),
        };

        self.client
            .post("apps.event.authorizations.list", &params)
            .await
    }

    /// Uninstall an app
    pub async fn uninstall(&self) -> Result<AppsUninstallResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.post("apps.uninstall", &params).await
    }

    /// List permissions for an app
    pub async fn permissions_info(&self) -> Result<AppsPermissionsInfoResponse> {
        let params: [(&str, &str); 0] = [];

        self.client.get("apps.permissions.info", &params).await
    }

    /// Request additional permissions for an app
    pub async fn permissions_request(
        &self,
        scopes: &[&str],
        trigger_id: &str,
    ) -> Result<AppsPermissionsRequestResponse> {
        let params = AppsPermissionsRequestRequest {
            scopes: scopes.join(","),
            trigger_id: trigger_id.to_string(),
        };

        self.client.post("apps.permissions.request", &params).await
    }

    /// List resources granted to an app
    pub async fn permissions_resources_list(&self) -> Result<AppsPermissionsResourcesListResponse> {
        let params = AppsPermissionsResourcesListRequest {
            cursor: None,
            limit: Some(100),
        };

        self.client
            .post("apps.permissions.resources.list", &params)
            .await
    }

    /// Create an app from a manifest
    ///
    /// # Arguments
    ///
    /// * `manifest` - App manifest as JSON
    pub async fn manifest_create(
        &self,
        manifest: serde_json::Value,
    ) -> Result<AppsManifestCreateResponse> {
        let params = AppsManifestCreateRequest { manifest };

        self.client.post("apps.manifest.create", &params).await
    }

    /// Delete an app created from a manifest
    ///
    /// # Arguments
    ///
    /// * `app_id` - The app ID to delete
    pub async fn manifest_delete(&self, app_id: &str) -> Result<AppsManifestDeleteResponse> {
        let params = AppsManifestDeleteRequest {
            app_id: app_id.to_string(),
        };

        self.client.post("apps.manifest.delete", &params).await
    }

    /// Export an app's manifest
    ///
    /// # Arguments
    ///
    /// * `app_id` - The app ID to export
    pub async fn manifest_export(&self, app_id: &str) -> Result<AppsManifestExportResponse> {
        let params = [("app_id", app_id)];

        self.client.get("apps.manifest.export", &params).await
    }

    /// Update an app from a manifest
    ///
    /// # Arguments
    ///
    /// * `app_id` - The app ID to update
    /// * `manifest` - New app manifest as JSON
    pub async fn manifest_update(
        &self,
        app_id: &str,
        manifest: serde_json::Value,
    ) -> Result<AppsManifestUpdateResponse> {
        let params = AppsManifestUpdateRequest {
            app_id: app_id.to_string(),
            manifest,
        };

        self.client.post("apps.manifest.update", &params).await
    }

    /// Validate an app manifest
    ///
    /// # Arguments
    ///
    /// * `manifest` - App manifest as JSON to validate
    pub async fn manifest_validate(
        &self,
        manifest: serde_json::Value,
    ) -> Result<AppsManifestValidateResponse> {
        let params = AppsManifestValidateRequest {
            manifest,
            app_id: None,
        };

        self.client.post("apps.manifest.validate", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct AppsEventAuthorizationsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AppsEventAuthorizationsListResponse {
    pub authorizations: Vec<AppAuthorization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::types::ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct AppAuthorization {
    pub enterprise_id: Option<String>,
    pub team_id: String,
    pub user_id: String,
    pub is_bot: bool,
}

#[derive(Debug, Deserialize)]
pub struct AppsUninstallResponse {}

#[derive(Debug, Deserialize)]
pub struct AppsPermissionsInfoResponse {
    pub info: PermissionsInfo,
}

#[derive(Debug, Deserialize)]
pub struct PermissionsInfo {
    pub team: TeamPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpim: Option<MpimPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub im: Option<ImPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_home: Option<AppHomePermissions>,
}

#[derive(Debug, Deserialize)]
pub struct TeamPermissions {
    pub scopes: Vec<String>,
    pub resources: TeamResources,
}

#[derive(Debug, Deserialize)]
pub struct TeamResources {
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelPermissions {
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GroupPermissions {
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MpimPermissions {
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImPermissions {
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppHomePermissions {
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AppsPermissionsRequestRequest {
    pub scopes: String,
    pub trigger_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AppsPermissionsRequestResponse {}

#[derive(Debug, Serialize)]
pub struct AppsPermissionsResourcesListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AppsPermissionsResourcesListResponse {
    pub resources: Vec<AppResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::types::ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct AppResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct AppsManifestCreateRequest {
    pub manifest: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct AppsManifestCreateResponse {
    pub app_id: String,
    pub credentials: AppCredentials,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_authorize_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppCredentials {
    pub client_id: String,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AppsManifestDeleteRequest {
    pub app_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AppsManifestDeleteResponse {}

#[derive(Debug, Deserialize)]
pub struct AppsManifestExportResponse {
    pub manifest: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct AppsManifestUpdateRequest {
    pub app_id: String,
    pub manifest: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct AppsManifestUpdateResponse {
    pub app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_updated: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AppsManifestValidateRequest {
    pub manifest: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppsManifestValidateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ManifestError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<ManifestWarning>>,
}

#[derive(Debug, Deserialize)]
pub struct ManifestError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ManifestWarning {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}
