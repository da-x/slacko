//! Users API
//!
//! Methods for retrieving user information.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::{Channel, ResponseMetadata, User};
use serde::{Deserialize, Serialize};

/// Users API client
pub struct UsersApi {
    client: SlackClient,
}

impl UsersApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// Get information about a user
    ///
    /// # Arguments
    ///
    /// * `user` - User ID
    pub async fn info(&self, user: &str) -> Result<UserInfoResponse> {
        let params = [("user", user)];

        self.client.get("users.info", &params).await
    }

    /// List all users in a Slack team
    pub async fn list(&self) -> Result<UsersListResponse> {
        let params = UsersListRequest {
            limit: Some(100),
            cursor: None,
        };

        self.client.post("users.list", &params).await
    }

    /// List users with custom parameters
    pub async fn list_with_options(&self, params: UsersListRequest) -> Result<UsersListResponse> {
        self.client.post("users.list", &params).await
    }

    /// Get the profile of a user
    ///
    /// # Arguments
    ///
    /// * `user` - User ID
    pub async fn get_profile(&self, user: &str) -> Result<UserProfileResponse> {
        let params = [("user", user)];

        self.client.get("users.profile.get", &params).await
    }

    /// Set the profile of the authenticated user
    ///
    /// # Arguments
    ///
    /// * `profile` - Profile fields to set (as JSON)
    pub async fn set_profile(&self, profile: serde_json::Value) -> Result<UserProfileResponse> {
        let params = UserProfileSetRequest { profile };

        self.client.post("users.profile.set", &params).await
    }

    /// Set the presence of the authenticated user
    ///
    /// # Arguments
    ///
    /// * `presence` - Either "auto" or "away"
    pub async fn set_presence(&self, presence: &str) -> Result<UserPresenceResponse> {
        let params = UserPresenceRequest {
            presence: presence.to_string(),
        };

        self.client.post("users.setPresence", &params).await
    }

    /// Get presence of a user
    ///
    /// # Arguments
    ///
    /// * `user` - User ID
    pub async fn get_presence(&self, user: &str) -> Result<UserGetPresenceResponse> {
        let params = [("user", user)];

        self.client.get("users.getPresence", &params).await
    }

    /// Look up a user by email
    ///
    /// # Arguments
    ///
    /// * `email` - Email address
    pub async fn lookup_by_email(&self, email: &str) -> Result<UserInfoResponse> {
        let params = [("email", email)];

        self.client.get("users.lookupByEmail", &params).await
    }

    /// List conversations the calling user may access
    ///
    /// Returns channels, groups, mpims, and ims that the user has access to.
    pub async fn conversations(&self) -> Result<UserConversationsResponse> {
        let params = UserConversationsRequest {
            user: None,
            types: Some("public_channel,private_channel,mpim,im".to_string()),
            exclude_archived: Some(true),
            limit: Some(100),
            cursor: None,
        };

        self.client.post("users.conversations", &params).await
    }

    /// List conversations for a specific user
    ///
    /// # Arguments
    ///
    /// * `user` - User ID to list conversations for
    pub async fn conversations_for_user(&self, user: &str) -> Result<UserConversationsResponse> {
        let params = UserConversationsRequest {
            user: Some(user.to_string()),
            types: Some("public_channel,private_channel,mpim,im".to_string()),
            exclude_archived: Some(true),
            limit: Some(100),
            cursor: None,
        };

        self.client.post("users.conversations", &params).await
    }

    /// List conversations with custom options
    pub async fn conversations_with_options(
        &self,
        params: UserConversationsRequest,
    ) -> Result<UserConversationsResponse> {
        self.client.post("users.conversations", &params).await
    }

    /// Get the identity of the authenticated user
    ///
    /// This returns the user's identity as an OAuth token owner.
    /// Requires the `identity.basic` scope.
    pub async fn identity(&self) -> Result<UserIdentityResponse> {
        let params: [(&str, &str); 0] = [];
        self.client.get("users.identity", &params).await
    }

    /// Delete the user's profile photo
    ///
    /// Removes the authenticated user's profile photo and resets it to the default.
    pub async fn delete_photo(&self) -> Result<DeletePhotoResponse> {
        let params = serde_json::json!({});
        self.client.post("users.deletePhoto", &params).await
    }

    /// Set the user's profile photo
    ///
    /// # Arguments
    ///
    /// * `image` - Image data as bytes
    ///
    /// # Note
    ///
    /// The image should be a PNG or JPEG. Slack will resize it to fit.
    /// This method uses multipart form upload.
    pub async fn set_photo(&self, image: Vec<u8>) -> Result<SetPhotoResponse> {
        self.client
            .upload_file("users.setPhoto", image, "image", "photo.png")
            .await
    }

    /// Set the user's profile photo with crop parameters
    ///
    /// # Arguments
    ///
    /// * `image` - Image data as bytes
    /// * `crop_x` - X coordinate of the crop area
    /// * `crop_y` - Y coordinate of the crop area
    /// * `crop_w` - Width of the crop area
    pub async fn set_photo_with_crop(
        &self,
        image: Vec<u8>,
        crop_x: u32,
        crop_y: u32,
        crop_w: u32,
    ) -> Result<SetPhotoResponse> {
        self.client
            .upload_file_with_params(
                "users.setPhoto",
                image,
                "image",
                "photo.png",
                &[
                    ("crop_x", &crop_x.to_string()),
                    ("crop_y", &crop_y.to_string()),
                    ("crop_w", &crop_w.to_string()),
                ],
            )
            .await
    }

    /// Look up discoverable contacts by email
    ///
    /// Find users outside your workspace that can be invited
    /// to connect via Slack Connect.
    ///
    /// # Arguments
    ///
    /// * `email` - Email address to look up
    /// * `token` - Optional OAuth token override
    pub async fn discoverable_contacts_lookup(
        &self,
        email: &str,
    ) -> Result<DiscoverableContactsLookupResponse> {
        let params = DiscoverableContactsLookupRequest {
            email: email.to_string(),
        };

        self.client
            .post("users.discoverableContacts.lookup", &params)
            .await
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct UsersListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UsersListResponse {
    pub members: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct UserProfileResponse {
    pub profile: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct UserProfileSetRequest {
    pub profile: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct UserPresenceRequest {
    pub presence: String,
}

#[derive(Debug, Deserialize)]
pub struct UserPresenceResponse {}

#[derive(Debug, Deserialize)]
pub struct UserGetPresenceResponse {
    pub presence: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_away: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_away: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserConversationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserConversationsResponse {
    pub channels: Vec<Channel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct UserIdentityResponse {
    pub user: UserIdentity,
    pub team: TeamIdentity,
}

#[derive(Debug, Deserialize)]
pub struct UserIdentity {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub image_24: Option<String>,
    #[serde(default)]
    pub image_32: Option<String>,
    #[serde(default)]
    pub image_48: Option<String>,
    #[serde(default)]
    pub image_72: Option<String>,
    #[serde(default)]
    pub image_192: Option<String>,
    #[serde(default)]
    pub image_512: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TeamIdentity {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub image_34: Option<String>,
    #[serde(default)]
    pub image_44: Option<String>,
    #[serde(default)]
    pub image_68: Option<String>,
    #[serde(default)]
    pub image_88: Option<String>,
    #[serde(default)]
    pub image_102: Option<String>,
    #[serde(default)]
    pub image_132: Option<String>,
    #[serde(default)]
    pub image_230: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeletePhotoResponse {}

#[derive(Debug, Deserialize)]
pub struct SetPhotoResponse {
    pub profile: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct DiscoverableContactsLookupRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverableContactsLookupResponse {
    #[serde(default)]
    pub user: Option<DiscoverableContact>,
    #[serde(default)]
    pub enterprise_user: Option<DiscoverableContact>,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverableContact {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub team_id: Option<String>,
    #[serde(default)]
    pub team_name: Option<String>,
    #[serde(default)]
    pub avatar_hash: Option<String>,
}
