//! Error types for the Slack SDK

/// Result type alias for Slack SDK operations
pub type Result<T> = std::result::Result<T, SlackError>;

/// Error types for Slack SDK operations
#[derive(Debug, thiserror::Error)]
pub enum SlackError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API error returned by Slack
    #[error("Slack API error: {code} - {message}")]
    ApiError { code: String, message: String },

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimitExceeded { retry_after: u64 },

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl SlackError {
    /// Create an API error from Slack's error response
    pub fn api_error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ApiError {
            code: code.into(),
            message: message.into(),
        }
    }

    /// Create a WebSocket error
    pub fn websocket_error(msg: impl Into<String>) -> Self {
        Self::WebSocketError(msg.into())
    }

    /// Create an authentication error
    pub fn auth_error(msg: impl Into<String>) -> Self {
        Self::AuthError(msg.into())
    }

    /// Create a configuration error
    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Check if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, SlackError::RateLimitExceeded { .. })
    }

    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(self, SlackError::AuthError(_))
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for SlackError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        SlackError::WebSocketError(err.to_string())
    }
}
