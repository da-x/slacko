//! OpenID Connect API
//!
//! Methods for OpenID Connect (OIDC) authentication flows.
//! Use this for implementing Sign in with Slack and identity verification.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// OpenID Connect API client
pub struct OpenIDApi {
    client: SlackClient,
}

/// Request for openid.connect.token
#[derive(Debug, Serialize)]
pub struct OpenIDTokenRequest {
    /// The authorization code from the OAuth redirect
    pub code: String,
    /// Your app's client ID
    pub client_id: String,
    /// Your app's client secret
    pub client_secret: String,
    /// The redirect URI used in the authorization request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// Grant type (always "authorization_code" for code exchange)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    /// Refresh token for token refresh flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

/// Response from openid.connect.token
#[derive(Debug, Deserialize)]
pub struct OpenIDTokenResponse {
    pub ok: bool,
    pub access_token: String,
    pub token_type: String,
    pub id_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
}

/// Response from openid.connect.userInfo
#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub ok: bool,
    pub sub: String, // User ID
    #[serde(rename = "https://slack.com/user_id")]
    pub user_id: Option<String>,
    #[serde(rename = "https://slack.com/team_id")]
    pub team_id: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
    #[serde(rename = "https://slack.com/team_name")]
    pub team_name: Option<String>,
    #[serde(rename = "https://slack.com/team_domain")]
    pub team_domain: Option<String>,
    #[serde(rename = "https://slack.com/user_image_24")]
    pub user_image_24: Option<String>,
    #[serde(rename = "https://slack.com/user_image_32")]
    pub user_image_32: Option<String>,
    #[serde(rename = "https://slack.com/user_image_48")]
    pub user_image_48: Option<String>,
    #[serde(rename = "https://slack.com/user_image_72")]
    pub user_image_72: Option<String>,
    #[serde(rename = "https://slack.com/user_image_192")]
    pub user_image_192: Option<String>,
    #[serde(rename = "https://slack.com/user_image_512")]
    pub user_image_512: Option<String>,
}

impl OpenIDApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Exchange an authorization code for an OpenID Connect token
    ///
    /// This method exchanges a verification code for an access token and ID token.
    /// The ID token contains claims about the authenticated user.
    ///
    /// # Arguments
    ///
    /// * `code` - The authorization code from the OAuth redirect
    /// * `client_id` - Your app's client ID
    /// * `client_secret` - Your app's client secret
    /// * `redirect_uri` - Optional redirect URI (must match authorization request)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use slacko::{SlackClient, AuthConfig};
    /// # let client = SlackClient::new(AuthConfig::bot("xoxb-token"))?;
    /// let openid = client.openid();
    ///
    /// // After user authorizes via Sign in with Slack
    /// let response = openid.token(
    ///     "authorization_code_here",
    ///     "your_client_id",
    ///     "your_client_secret",
    ///     Some("https://yourapp.com/auth/callback")
    /// ).await?;
    ///
    /// println!("ID token: {}", response.id_token);
    /// println!("Access token: {}", response.access_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn token(
        &self,
        code: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: Option<&str>,
    ) -> Result<OpenIDTokenResponse> {
        let request = OpenIDTokenRequest {
            code: code.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.map(|s| s.to_string()),
            grant_type: Some("authorization_code".to_string()),
            refresh_token: None,
        };
        self.client.post("openid.connect.token", &request).await
    }

    /// Refresh an OpenID Connect access token
    ///
    /// Exchange a refresh token for a new access token and ID token.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - The refresh token from a previous token exchange
    /// * `client_id` - Your app's client ID
    /// * `client_secret` - Your app's client secret
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use slacko::{SlackClient, AuthConfig};
    /// # let client = SlackClient::new(AuthConfig::bot("xoxb-token"))?;
    /// let openid = client.openid();
    ///
    /// let response = openid.refresh_token(
    ///     "refresh_token_here",
    ///     "your_client_id",
    ///     "your_client_secret"
    /// ).await?;
    ///
    /// println!("New access token: {}", response.access_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<OpenIDTokenResponse> {
        let request = OpenIDTokenRequest {
            code: String::new(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: None,
            grant_type: Some("refresh_token".to_string()),
            refresh_token: Some(refresh_token.to_string()),
        };
        self.client.post("openid.connect.token", &request).await
    }

    /// Get user identity information
    ///
    /// Retrieve information about the authenticated user using an access token
    /// obtained from the OpenID Connect flow.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use slacko::{SlackClient, AuthConfig};
    /// # let client = SlackClient::new(AuthConfig::bot("xoxb-token"))?;
    /// let openid = client.openid();
    ///
    /// // Use the access token from token exchange
    /// let user_info = openid.user_info().await?;
    ///
    /// println!("User ID: {}", user_info.sub);
    /// println!("Email: {:?}", user_info.email);
    /// println!("Name: {:?}", user_info.name);
    /// println!("Team: {:?}", user_info.team_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn user_info(&self) -> Result<UserInfoResponse> {
        self.client.post("openid.connect.userInfo", &()).await
    }
}
