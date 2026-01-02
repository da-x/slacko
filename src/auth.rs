//! Authentication configuration for Slack API

use crate::error::{Result, SlackError};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, COOKIE, USER_AGENT};

/// Authentication configuration
#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub(crate) auth_type: AuthType,
}

/// Authentication type
#[derive(Clone, Debug)]
pub enum AuthType {
    /// Stealth mode using xoxc token and xoxd cookie
    /// This uses browser-extracted tokens without requiring bot installation
    Stealth {
        xoxc_token: String,
        xoxd_cookie: String,
    },

    /// OAuth user token (xoxp-)
    OAuth { token: String },

    /// Bot token (xoxb-)
    Bot { token: String },
}

impl AuthConfig {
    /// Create a stealth mode authentication configuration
    ///
    /// # Arguments
    ///
    /// * `xoxc_token` - The xoxc token from browser
    /// * `xoxd_cookie` - The d cookie value from browser
    ///
    /// # Example
    ///
    /// ```
    /// use slacko::AuthConfig;
    ///
    /// let auth = AuthConfig::stealth("xoxc-123...", "xoxd-456...");
    /// ```
    pub fn stealth(xoxc_token: impl Into<String>, xoxd_cookie: impl Into<String>) -> Self {
        Self {
            auth_type: AuthType::Stealth {
                xoxc_token: xoxc_token.into(),
                xoxd_cookie: xoxd_cookie.into(),
            },
        }
    }

    /// Create an OAuth token authentication configuration
    ///
    /// # Arguments
    ///
    /// * `token` - The OAuth user token (starts with xoxp-)
    ///
    /// # Example
    ///
    /// ```
    /// use slacko::AuthConfig;
    ///
    /// let auth = AuthConfig::oauth("xoxp-123...");
    /// ```
    pub fn oauth(token: impl Into<String>) -> Self {
        Self {
            auth_type: AuthType::OAuth {
                token: token.into(),
            },
        }
    }

    /// Create a bot token authentication configuration
    ///
    /// # Arguments
    ///
    /// * `token` - The bot token (starts with xoxb-)
    ///
    /// # Example
    ///
    /// ```
    /// use slacko::AuthConfig;
    ///
    /// let auth = AuthConfig::bot("xoxb-123...");
    /// ```
    pub fn bot(token: impl Into<String>) -> Self {
        Self {
            auth_type: AuthType::Bot {
                token: token.into(),
            },
        }
    }

    /// Load authentication from environment variables
    ///
    /// Checks for the following environment variables in order:
    /// 1. `SLACK_XOXC_TOKEN` and `SLACK_XOXD_COOKIE` for stealth mode
    /// 2. `SLACK_XOXP_TOKEN` for OAuth
    /// 3. `SLACK_BOT_TOKEN` or `SLACK_TOKEN` for bot tokens
    ///
    /// # Example
    ///
    /// ```no_run
    /// use slacko::AuthConfig;
    ///
    /// let auth = AuthConfig::from_env().expect("No Slack credentials found");
    /// ```
    pub fn from_env() -> Result<Self> {
        // Try stealth mode first
        if let (Ok(xoxc), Ok(xoxd)) = (
            std::env::var("SLACK_XOXC_TOKEN"),
            std::env::var("SLACK_XOXD_COOKIE"),
        ) {
            return Ok(Self::stealth(xoxc, xoxd));
        }

        // Try OAuth token
        if let Ok(token) = std::env::var("SLACK_XOXP_TOKEN") {
            return Ok(Self::oauth(token));
        }

        // Try bot token
        if let Ok(token) =
            std::env::var("SLACK_BOT_TOKEN").or_else(|_| std::env::var("SLACK_TOKEN"))
        {
            return Ok(Self::bot(token));
        }

        Err(SlackError::config_error(
            "No Slack credentials found in environment. Set SLACK_XOXC_TOKEN + SLACK_XOXD_COOKIE, SLACK_XOXP_TOKEN, or SLACK_BOT_TOKEN",
        ))
    }

    /// Build HTTP headers for API requests
    pub(crate) fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        match &self.auth_type {
            AuthType::Stealth {
                xoxc_token,
                xoxd_cookie,
            } => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", xoxc_token))
                        .unwrap_or_else(|_| HeaderValue::from_static("")),
                );
                headers.insert(
                    COOKIE,
                    HeaderValue::from_str(&format!("d={}", xoxd_cookie))
                        .unwrap_or_else(|_| HeaderValue::from_static("")),
                );
            }
            AuthType::OAuth { token } | AuthType::Bot { token } => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token))
                        .unwrap_or_else(|_| HeaderValue::from_static("")),
                );
            }
        }

        headers.insert(USER_AGENT, HeaderValue::from_static("slack-sdk-rust/0.1.0"));

        headers
    }

    /// Get the authentication type as a string
    pub fn auth_type_str(&self) -> &str {
        match &self.auth_type {
            AuthType::Stealth { .. } => "stealth",
            AuthType::OAuth { .. } => "oauth",
            AuthType::Bot { .. } => "bot",
        }
    }
}
