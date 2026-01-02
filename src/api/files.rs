//! Files API
//!
//! Methods for uploading and managing files.

use crate::client::SlackClient;
use crate::error::{Result, SlackError};
use crate::types::{File, ResponseMetadata, SlackResponse};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

/// Files API client
pub struct FilesApi {
    client: SlackClient,
}

impl FilesApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Upload a file
    ///
    /// # Arguments
    ///
    /// * `channels` - Channel IDs to share the file in
    /// * `content` - File content as bytes
    /// * `filename` - Filename
    pub async fn upload(
        &self,
        channels: &[&str],
        content: Vec<u8>,
        filename: &str,
    ) -> Result<FileUploadResponse> {
        let url = format!("{}/files.upload", self.client.base_url);
        let headers = self.client.auth.build_headers();

        // Build multipart form
        let file_part = Part::bytes(content)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| SlackError::config_error(format!("Invalid mime type: {}", e)))?;

        let mut form = Form::new()
            .text("channels", channels.join(","))
            .part("file", file_part);

        // Add filename as a form field too
        form = form.text("filename", filename.to_string());

        let response = self
            .client
            .http
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        // Check for rate limiting
        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);
            return Err(SlackError::RateLimitExceeded { retry_after });
        }

        let slack_response: SlackResponse<FileUploadResponse> = response.json().await?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(SlackError::api_error("files.upload", error_msg));
        }

        slack_response
            .data
            .ok_or_else(|| SlackError::api_error("files.upload", "No data in response"))
    }

    /// Upload a file with custom parameters (for text content)
    pub async fn upload_with_options(
        &self,
        params: FileUploadRequest,
    ) -> Result<FileUploadResponse> {
        self.client.post("files.upload", &params).await
    }

    /// Upload a file with additional options
    ///
    /// # Arguments
    ///
    /// * `channels` - Channel IDs to share the file in
    /// * `content` - File content as bytes
    /// * `filename` - Filename
    /// * `options` - Additional upload options (title, initial_comment, thread_ts)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use slacko::{SlackClient, AuthConfig};
    /// # use slacko::api::files::FileUploadOptions;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SlackClient::new(AuthConfig::oauth("token"))?;
    /// let options = FileUploadOptions::new()
    ///     .title("My Document")
    ///     .initial_comment("Here's the file you requested")
    ///     .thread_ts("1234567890.123456");
    ///
    /// client.files()
    ///     .upload_to_thread(&["C12345"], b"file content".to_vec(), "doc.txt", options)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_to_thread(
        &self,
        channels: &[&str],
        content: Vec<u8>,
        filename: &str,
        options: FileUploadOptions,
    ) -> Result<FileUploadResponse> {
        let url = format!("{}/files.upload", self.client.base_url);
        let headers = self.client.auth.build_headers();

        let file_part = Part::bytes(content)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| SlackError::config_error(format!("Invalid mime type: {}", e)))?;

        let mut form = Form::new()
            .text("channels", channels.join(","))
            .text("filename", filename.to_string())
            .part("file", file_part);

        if let Some(title) = options.title {
            form = form.text("title", title);
        }
        if let Some(initial_comment) = options.initial_comment {
            form = form.text("initial_comment", initial_comment);
        }
        if let Some(thread_ts) = options.thread_ts {
            form = form.text("thread_ts", thread_ts);
        }
        if let Some(filetype) = options.filetype {
            form = form.text("filetype", filetype);
        }

        let response = self
            .client
            .http
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if response.status().as_u16() == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);
            return Err(SlackError::RateLimitExceeded { retry_after });
        }

        let slack_response: SlackResponse<FileUploadResponse> = response.json().await?;

        if !slack_response.ok {
            let error_msg = slack_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(SlackError::api_error("files.upload", error_msg));
        }

        slack_response
            .data
            .ok_or_else(|| SlackError::api_error("files.upload", "No data in response"))
    }

    /// Get information about a file
    ///
    /// # Arguments
    ///
    /// * `file` - File ID
    pub async fn info(&self, file: &str) -> Result<FileInfoResponse> {
        let params = [("file", file)];

        self.client.get("files.info", &params).await
    }

    /// List files
    pub async fn list(&self) -> Result<FilesListResponse> {
        let params = FilesListRequest {
            user: None,
            channel: None,
            count: Some(100),
            page: None,
        };

        self.client.post("files.list", &params).await
    }

    /// List files with custom parameters
    pub async fn list_with_options(&self, params: FilesListRequest) -> Result<FilesListResponse> {
        self.client.post("files.list", &params).await
    }

    /// Delete a file
    ///
    /// # Arguments
    ///
    /// * `file` - File ID
    pub async fn delete(&self, file: &str) -> Result<FileDeleteResponse> {
        let params = FileDeleteRequest {
            file: file.to_string(),
        };

        self.client.post("files.delete", &params).await
    }

    /// Share a file to a channel
    ///
    /// # Arguments
    ///
    /// * `file` - File ID
    /// * `channel` - Channel ID
    pub async fn share(&self, file: &str, channel: &str) -> Result<FileShareResponse> {
        let params = FileShareRequest {
            file: file.to_string(),
            channel: channel.to_string(),
        };

        self.client.post("files.sharedPublicURL", &params).await
    }

    /// Revoke public/external sharing access for a file
    ///
    /// # Arguments
    ///
    /// * `file` - File ID
    ///
    /// # Note
    ///
    /// This disables the public URL that was created via `sharedPublicURL`.
    /// The file will no longer be accessible via that URL.
    pub async fn revoke_public_url(&self, file: &str) -> Result<RevokePublicUrlResponse> {
        let params = RevokePublicUrlRequest {
            file: file.to_string(),
        };

        self.client.post("files.revokePublicURL", &params).await
    }

    // ========== Remote Files API ==========

    /// Add a remote file
    ///
    /// Registers an external file with Slack without uploading the file content.
    ///
    /// # Arguments
    ///
    /// * `external_id` - Creator-defined unique ID for the file
    /// * `external_url` - URL of the remote file
    /// * `title` - Title of the file
    pub async fn remote_add(
        &self,
        external_id: &str,
        external_url: &str,
        title: &str,
    ) -> Result<RemoteFileResponse> {
        let params = RemoteAddRequest {
            external_id: external_id.to_string(),
            external_url: external_url.to_string(),
            title: title.to_string(),
            filetype: None,
            indexable_file_contents: None,
            preview_image: None,
        };

        self.client.post("files.remote.add", &params).await
    }

    /// Add a remote file with full options
    pub async fn remote_add_with_options(
        &self,
        params: RemoteAddRequest,
    ) -> Result<RemoteFileResponse> {
        self.client.post("files.remote.add", &params).await
    }

    /// Get information about a remote file
    ///
    /// # Arguments
    ///
    /// * `external_id` - Creator-defined unique ID for the file
    /// * `file` - Slack file ID (alternative to external_id)
    pub async fn remote_info(
        &self,
        external_id: Option<&str>,
        file: Option<&str>,
    ) -> Result<RemoteFileResponse> {
        let params = RemoteInfoRequest {
            external_id: external_id.map(|s| s.to_string()),
            file: file.map(|s| s.to_string()),
        };

        self.client.post("files.remote.info", &params).await
    }

    /// List remote files
    pub async fn remote_list(&self) -> Result<RemoteListResponse> {
        let params = RemoteListRequest {
            channel: None,
            cursor: None,
            limit: Some(100),
            ts_from: None,
            ts_to: None,
        };

        self.client.post("files.remote.list", &params).await
    }

    /// List remote files with options
    pub async fn remote_list_with_options(
        &self,
        params: RemoteListRequest,
    ) -> Result<RemoteListResponse> {
        self.client.post("files.remote.list", &params).await
    }

    /// Remove a remote file
    ///
    /// # Arguments
    ///
    /// * `external_id` - Creator-defined unique ID for the file
    /// * `file` - Slack file ID (alternative to external_id)
    pub async fn remote_remove(
        &self,
        external_id: Option<&str>,
        file: Option<&str>,
    ) -> Result<RemoteRemoveResponse> {
        let params = RemoteRemoveRequest {
            external_id: external_id.map(|s| s.to_string()),
            file: file.map(|s| s.to_string()),
        };

        self.client.post("files.remote.remove", &params).await
    }

    /// Share a remote file to channels
    ///
    /// # Arguments
    ///
    /// * `channels` - Comma-separated list of channel IDs
    /// * `external_id` - Creator-defined unique ID for the file
    /// * `file` - Slack file ID (alternative to external_id)
    pub async fn remote_share(
        &self,
        channels: &str,
        external_id: Option<&str>,
        file: Option<&str>,
    ) -> Result<RemoteFileResponse> {
        let params = RemoteShareRequest {
            channels: channels.to_string(),
            external_id: external_id.map(|s| s.to_string()),
            file: file.map(|s| s.to_string()),
        };

        self.client.post("files.remote.share", &params).await
    }

    /// Update a remote file
    ///
    /// # Arguments
    ///
    /// * `external_id` - Creator-defined unique ID for the file
    /// * `file` - Slack file ID (alternative to external_id)
    /// * `title` - New title for the file
    /// * `external_url` - New URL for the file
    pub async fn remote_update(
        &self,
        external_id: Option<&str>,
        file: Option<&str>,
        title: Option<&str>,
        external_url: Option<&str>,
    ) -> Result<RemoteFileResponse> {
        let params = RemoteUpdateRequest {
            external_id: external_id.map(|s| s.to_string()),
            file: file.map(|s| s.to_string()),
            title: title.map(|s| s.to_string()),
            external_url: external_url.map(|s| s.to_string()),
            filetype: None,
            indexable_file_contents: None,
            preview_image: None,
        };

        self.client.post("files.remote.update", &params).await
    }

    /// Update a remote file with full options
    pub async fn remote_update_with_options(
        &self,
        params: RemoteUpdateRequest,
    ) -> Result<RemoteFileResponse> {
        self.client.post("files.remote.update", &params).await
    }

    // ========== Files v2 Upload API ==========

    /// Get a URL for uploading a file (v2 API)
    ///
    /// This is the first step in the v2 file upload flow:
    /// 1. Call `get_upload_url_external` to get an upload URL
    /// 2. Upload the file content to the returned URL using a PUT request
    /// 3. Call `complete_upload_external` to finalize and share the file
    ///
    /// # Arguments
    ///
    /// * `filename` - Name of the file being uploaded
    /// * `length` - Size of the file in bytes
    /// * `alt_txt` - Alt text for the file (optional)
    /// * `snippet_type` - Syntax type for snippet files (optional)
    pub async fn get_upload_url_external(
        &self,
        filename: &str,
        length: u64,
        alt_txt: Option<&str>,
        snippet_type: Option<&str>,
    ) -> Result<GetUploadUrlExternalResponse> {
        let params = GetUploadUrlExternalRequest {
            filename: filename.to_string(),
            length,
            alt_txt: alt_txt.map(|s| s.to_string()),
            snippet_type: snippet_type.map(|s| s.to_string()),
        };

        self.client
            .post("files.getUploadURLExternal", &params)
            .await
    }

    /// Complete a file upload (v2 API)
    ///
    /// After uploading file content to the URL returned by `get_upload_url_external`,
    /// call this method to finalize the upload and share the file to channels.
    ///
    /// # Arguments
    ///
    /// * `files` - Array of file objects containing file_id and optional title
    /// * `channel_id` - Channel to share the files in (optional)
    /// * `initial_comment` - Initial comment to add with the files (optional)
    /// * `thread_ts` - Thread timestamp to reply in (optional)
    pub async fn complete_upload_external(
        &self,
        files: &[UploadedFileInfo],
        channel_id: Option<&str>,
        initial_comment: Option<&str>,
        thread_ts: Option<&str>,
    ) -> Result<CompleteUploadExternalResponse> {
        let params = CompleteUploadExternalRequest {
            files: files.to_vec(),
            channel_id: channel_id.map(|s| s.to_string()),
            initial_comment: initial_comment.map(|s| s.to_string()),
            thread_ts: thread_ts.map(|s| s.to_string()),
        };

        self.client
            .post("files.completeUploadExternal", &params)
            .await
    }

    // ========== File Comments API ==========

    /// Delete a file comment
    ///
    /// # Arguments
    ///
    /// * `file` - File ID containing the comment
    /// * `id` - Comment ID to delete
    pub async fn comments_delete(&self, file: &str, id: &str) -> Result<CommentsDeleteResponse> {
        let params = CommentsDeleteRequest {
            file: file.to_string(),
            id: id.to_string(),
        };

        self.client.post("files.comments.delete", &params).await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct FileUploadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
}

#[derive(Debug, Default)]
pub struct FileUploadOptions {
    pub title: Option<String>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<String>,
    pub filetype: Option<String>,
}

impl FileUploadOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn initial_comment(mut self, comment: &str) -> Self {
        self.initial_comment = Some(comment.to_string());
        self
    }

    pub fn thread_ts(mut self, ts: &str) -> Self {
        self.thread_ts = Some(ts.to_string());
        self
    }

    pub fn filetype(mut self, filetype: &str) -> Self {
        self.filetype = Some(filetype.to_string());
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct FileUploadResponse {
    pub file: File,
}

#[derive(Debug, Deserialize)]
pub struct FileInfoResponse {
    pub file: File,
}

#[derive(Debug, Serialize)]
pub struct FilesListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct FilesListResponse {
    pub files: Vec<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize)]
pub struct FileDeleteRequest {
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct FileDeleteResponse {}

#[derive(Debug, Serialize)]
pub struct FileShareRequest {
    pub file: String,
    pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct FileShareResponse {
    pub file: File,
}

#[derive(Debug, Serialize)]
pub struct RevokePublicUrlRequest {
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct RevokePublicUrlResponse {
    pub file: File,
}

// ========== Remote Files Types ==========

#[derive(Debug, Serialize)]
pub struct RemoteAddRequest {
    pub external_id: String,
    pub external_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexable_file_contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RemoteInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RemoteListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_to: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RemoteRemoveRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RemoteShareRequest {
    pub channels: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RemoteUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexable_file_contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RemoteFileResponse {
    pub file: File,
}

#[derive(Debug, Deserialize)]
pub struct RemoteListResponse {
    pub files: Vec<File>,
    #[serde(default)]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct RemoteRemoveResponse {}

// ========== Files v2 Upload Types ==========

#[derive(Debug, Serialize)]
pub struct GetUploadUrlExternalRequest {
    pub filename: String,
    pub length: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_txt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetUploadUrlExternalResponse {
    pub upload_url: String,
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFileInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UploadedFileInfo {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: None,
        }
    }

    pub fn with_title(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: Some(title.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CompleteUploadExternalRequest {
    pub files: Vec<UploadedFileInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteUploadExternalResponse {
    pub files: Vec<File>,
}

// ========== File Comments Types ==========

#[derive(Debug, Serialize)]
pub struct CommentsDeleteRequest {
    pub file: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentsDeleteResponse {}
