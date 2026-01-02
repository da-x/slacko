//! OAuth v2 API
//!
//! Methods for OAuth token exchange and management.
//! Use this for building public Slack apps with OAuth flows.

use crate::client::SlackClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// OAuth v2 API client
pub struct OAuthApi {
    client: SlackClient,
}

/// Request for oauth.v2.access
#[derive(Debug, Serialize)]
pub struct OAuthAccessRequest {
    /// The authorization code from the OAuth redirect
    pub code: String,
    /// Your app's client ID
    pub client_id: String,
    /// Your app's client secret
    pub client_secret: String,
    /// The redirect URI used in the authorization request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
}

/// Response from oauth.v2.access
#[derive(Debug, Deserialize)]
pub struct OAuthAccessResponse {
    pub ok: bool,
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub bot_user_id: Option<String>,
    pub app_id: String,
    pub team: TeamInfo,
    pub enterprise: Option<EnterpriseInfo>,
    pub authed_user: Option<AuthedUser>,
    pub incoming_webhook: Option<IncomingWebhook>,
    pub is_enterprise_install: Option<bool>,
}

/// Team information in OAuth response
#[derive(Debug, Deserialize)]
pub struct TeamInfo {
    pub id: String,
    pub name: String,
}

/// Enterprise information in OAuth response
#[derive(Debug, Deserialize)]
pub struct EnterpriseInfo {
    pub id: String,
    pub name: String,
}

/// Authed user information in OAuth response
#[derive(Debug, Deserialize)]
pub struct AuthedUser {
    pub id: String,
    pub scope: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
}

/// Incoming webhook configuration
#[derive(Debug, Deserialize)]
pub struct IncomingWebhook {
    pub channel: String,
    pub channel_id: String,
    pub configuration_url: String,
    pub url: String,
}

/// Request for oauth.v2.exchange
#[derive(Debug, Serialize)]
pub struct OAuthExchangeRequest {
    /// Your app's client ID
    pub client_id: String,
    /// Your app's client secret
    pub client_secret: String,
    /// The legacy token to exchange (xoxp-, xoxb-, or xoxa-)
    pub token: String,
}

/// Response from oauth.v2.exchange
#[derive(Debug, Deserialize)]
pub struct OAuthExchangeResponse {
    pub ok: bool,
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub team: TeamInfo,
    pub enterprise: Option<EnterpriseInfo>,
    pub is_enterprise_install: Option<bool>,
}

impl OAuthApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Exchange a temporary OAuth code for an access token
    ///
    /// This method exchanges a verification code granted by the user for an access token.
    /// Call this method after the user authorizes your app in the OAuth flow.
    ///
    /// # Arguments
    ///
    /// * `code` - The authorization code from the OAuth redirect
    /// * `client_id` - Your app's client ID
    /// * `client_secret` - Your app's client secret
    /// * `redirect_uri` - Optional redirect URI (must match the one used in authorization)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use slacko::{SlackClient, AuthConfig};
    /// # let client = SlackClient::new(AuthConfig::bot("xoxb-token"))?;
    /// let oauth = client.oauth();
    ///
    /// // After user authorizes and you receive the code
    /// let response = oauth.access(
    ///     "authorization_code_here",
    ///     "your_client_id",
    ///     "your_client_secret",
    ///     Some("https://yourapp.com/oauth/callback")
    /// ).await?;
    ///
    /// println!("Access token: {}", response.access_token);
    /// println!("Team: {}", response.team.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn access(
        &self,
        code: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: Option<&str>,
    ) -> Result<OAuthAccessResponse> {
        let request = OAuthAccessRequest {
            code: code.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.map(|s| s.to_string()),
        };
        self.client.post("oauth.v2.access", &request).await
    }

    /// Exchange a legacy token for a new workspace token
    ///
    /// This method exchanges a legacy Slack API token (xoxp-, xoxb-, or xoxa-)
    /// for a new workspace token with granular scopes.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Your app's client ID
    /// * `client_secret` - Your app's client secret
    /// * `token` - The legacy token to exchange
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use slacko::{SlackClient, AuthConfig};
    /// # let client = SlackClient::new(AuthConfig::bot("xoxb-token"))?;
    /// let oauth = client.oauth();
    ///
    /// let response = oauth.exchange(
    ///     "your_client_id",
    ///     "your_client_secret",
    ///     "xoxp-legacy-token"
    /// ).await?;
    ///
    /// println!("New access token: {}", response.access_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn exchange(
        &self,
        client_id: &str,
        client_secret: &str,
        token: &str,
    ) -> Result<OAuthExchangeResponse> {
        let request = OAuthExchangeRequest {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            token: token.to_string(),
        };
        self.client.post("oauth.v2.exchange", &request).await
    }
}
