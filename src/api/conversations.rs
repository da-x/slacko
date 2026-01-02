//! Conversations API
//!
//! Methods for managing channels, groups, and DMs.

use crate::client::SlackClient;
use crate::error::Result;
use crate::types::{Channel, Message, ResponseMetadata};
use serde::{Deserialize, Serialize};

/// Conversations API client
pub struct ConversationsApi {
    client: SlackClient,
}

impl ConversationsApi {
    pub(crate) fn new(client: SlackClient) -> Self {
        Self { client }
    }

    /// List all channels in a Slack team
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use slacko::{SlackClient, AuthConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SlackClient::new(AuthConfig::oauth("token"))?;
    /// let channels = client.conversations().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<ListConversationsResponse> {
        let params = ListConversationsRequest {
            exclude_archived: Some(false),
            types: Some("public_channel,private_channel".to_string()),
            limit: Some(100),
            cursor: None,
        };

        self.client.post("conversations.list", &params).await
    }

    /// List channels with custom parameters
    pub async fn list_with_options(
        &self,
        params: ListConversationsRequest,
    ) -> Result<ListConversationsResponse> {
        self.client.post("conversations.list", &params).await
    }

    /// Get information about a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn info(&self, channel: &str) -> Result<ConversationInfoResponse> {
        let params = [("channel", channel)];

        self.client.get("conversations.info", &params).await
    }

    /// Join a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn join(&self, channel: &str) -> Result<JoinConversationResponse> {
        let params = JoinConversationRequest {
            channel: channel.to_string(),
        };

        self.client.post("conversations.join", &params).await
    }

    /// Leave a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn leave(&self, channel: &str) -> Result<LeaveConversationResponse> {
        let params = LeaveConversationRequest {
            channel: channel.to_string(),
        };

        self.client.post("conversations.leave", &params).await
    }

    /// Create a new conversation
    ///
    /// # Arguments
    ///
    /// * `name` - Channel name
    /// * `is_private` - Whether the channel should be private
    pub async fn create(&self, name: &str, is_private: bool) -> Result<CreateConversationResponse> {
        let params = CreateConversationRequest {
            name: name.to_string(),
            is_private: Some(is_private),
        };

        self.client.post("conversations.create", &params).await
    }

    /// Archive a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn archive(&self, channel: &str) -> Result<ArchiveConversationResponse> {
        let params = ArchiveConversationRequest {
            channel: channel.to_string(),
        };

        self.client.post("conversations.archive", &params).await
    }

    /// Unarchive a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn unarchive(&self, channel: &str) -> Result<UnarchiveConversationResponse> {
        let params = UnarchiveConversationRequest {
            channel: channel.to_string(),
        };

        self.client.post("conversations.unarchive", &params).await
    }

    /// Get conversation history (messages)
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn history(&self, channel: &str) -> Result<ConversationHistoryResponse> {
        let params = ConversationHistoryRequest {
            channel: channel.to_string(),
            limit: Some(100),
            cursor: None,
            oldest: None,
            latest: None,
            inclusive: None,
        };

        self.client.post("conversations.history", &params).await
    }

    /// Get conversation history with custom parameters
    pub async fn history_with_options(
        &self,
        params: ConversationHistoryRequest,
    ) -> Result<ConversationHistoryResponse> {
        self.client.post("conversations.history", &params).await
    }

    /// Get replies to a thread
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `ts` - Thread timestamp
    pub async fn replies(&self, channel: &str, ts: &str) -> Result<ConversationRepliesResponse> {
        let params = ConversationRepliesRequest {
            channel: channel.to_string(),
            ts: ts.to_string(),
            limit: Some(100),
            cursor: None,
        };

        self.client.post("conversations.replies", &params).await
    }

    /// Invite users to a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `users` - User IDs to invite
    pub async fn invite(
        &self,
        channel: &str,
        users: &[&str],
    ) -> Result<InviteConversationResponse> {
        let params = InviteConversationRequest {
            channel: channel.to_string(),
            users: users.join(","),
        };

        self.client.post("conversations.invite", &params).await
    }

    /// Kick a user from a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `user` - User ID to kick
    pub async fn kick(&self, channel: &str, user: &str) -> Result<KickConversationResponse> {
        let params = KickConversationRequest {
            channel: channel.to_string(),
            user: user.to_string(),
        };

        self.client.post("conversations.kick", &params).await
    }

    /// List members of a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    pub async fn members(&self, channel: &str) -> Result<ConversationMembersResponse> {
        let params = ConversationMembersRequest {
            channel: channel.to_string(),
            limit: Some(100),
            cursor: None,
        };

        self.client.post("conversations.members", &params).await
    }

    /// Open or resume a direct message or multi-person DM
    ///
    /// # Arguments
    ///
    /// * `users` - Array of user IDs to open a DM with
    pub async fn open(&self, users: &[&str]) -> Result<OpenConversationResponse> {
        let params = OpenConversationRequest {
            users: Some(
                users
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            channel: None,
            return_im: Some(true),
        };

        self.client.post("conversations.open", &params).await
    }

    /// Rename a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `name` - New channel name
    ///
    /// # Note
    ///
    /// This method requires appropriate permissions and only works on channels
    /// that the calling user/bot has the ability to rename.
    pub async fn rename(&self, channel: &str, name: &str) -> Result<RenameConversationResponse> {
        let params = RenameConversationRequest {
            channel: channel.to_string(),
            name: name.to_string(),
        };

        self.client.post("conversations.rename", &params).await
    }

    /// Set the purpose of a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `purpose` - New purpose text
    pub async fn set_purpose(&self, channel: &str, purpose: &str) -> Result<SetPurposeResponse> {
        let params = SetPurposeRequest {
            channel: channel.to_string(),
            purpose: purpose.to_string(),
        };

        self.client.post("conversations.setPurpose", &params).await
    }

    /// Set the topic of a conversation
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `topic` - New topic text
    pub async fn set_topic(&self, channel: &str, topic: &str) -> Result<SetTopicResponse> {
        let params = SetTopicRequest {
            channel: channel.to_string(),
            topic: topic.to_string(),
        };

        self.client.post("conversations.setTopic", &params).await
    }

    /// Set the read cursor in a channel
    ///
    /// Marks the given channel as read by moving the read cursor.
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID
    /// * `ts` - Timestamp of the message to mark as read
    pub async fn mark(&self, channel: &str, ts: &str) -> Result<MarkConversationResponse> {
        let params = MarkConversationRequest {
            channel: channel.to_string(),
            ts: ts.to_string(),
        };

        self.client.post("conversations.mark", &params).await
    }

    /// Close a direct message or multi-person direct message
    ///
    /// # Arguments
    ///
    /// * `channel` - Channel ID of the DM or MPDM to close
    pub async fn close(&self, channel: &str) -> Result<CloseConversationResponse> {
        let params = CloseConversationRequest {
            channel: channel.to_string(),
        };

        self.client.post("conversations.close", &params).await
    }

    // ============================================
    // Slack Connect Methods
    // ============================================

    /// Accept a Slack Connect channel invite
    ///
    /// # Arguments
    ///
    /// * `channel_name` - Name for the channel
    /// * `channel_id` - ID of the shared channel being invited to (optional if invite_id provided)
    /// * `invite_id` - ID of the invite (optional if channel_id provided)
    /// * `free_trial_accepted` - Whether the free trial was accepted
    /// * `is_private` - Whether the channel should be private
    /// * `team_id` - Team ID for Enterprise Grid
    pub async fn accept_shared_invite(
        &self,
        channel_name: &str,
        channel_id: Option<&str>,
        invite_id: Option<&str>,
        free_trial_accepted: Option<bool>,
        is_private: Option<bool>,
        team_id: Option<&str>,
    ) -> Result<AcceptSharedInviteResponse> {
        let params = AcceptSharedInviteRequest {
            channel_name: channel_name.to_string(),
            channel_id: channel_id.map(|s| s.to_string()),
            invite_id: invite_id.map(|s| s.to_string()),
            free_trial_accepted,
            is_private,
            team_id: team_id.map(|s| s.to_string()),
        };

        self.client
            .post("conversations.acceptSharedInvite", &params)
            .await
    }

    /// Approve a Slack Connect channel invite request
    ///
    /// # Arguments
    ///
    /// * `invite_id` - ID of the invite to approve
    /// * `target_team` - Team ID of the target workspace (optional)
    pub async fn approve_shared_invite(
        &self,
        invite_id: &str,
        target_team: Option<&str>,
    ) -> Result<ApproveSharedInviteResponse> {
        let params = ApproveSharedInviteRequest {
            invite_id: invite_id.to_string(),
            target_team: target_team.map(|s| s.to_string()),
        };

        self.client
            .post("conversations.approveSharedInvite", &params)
            .await
    }

    /// Decline a Slack Connect channel invite
    ///
    /// # Arguments
    ///
    /// * `invite_id` - ID of the invite to decline
    /// * `target_team` - Team ID of the target workspace (optional)
    pub async fn decline_shared_invite(
        &self,
        invite_id: &str,
        target_team: Option<&str>,
    ) -> Result<DeclineSharedInviteResponse> {
        let params = DeclineSharedInviteRequest {
            invite_id: invite_id.to_string(),
            target_team: target_team.map(|s| s.to_string()),
        };

        self.client
            .post("conversations.declineSharedInvite", &params)
            .await
    }

    /// Send a Slack Connect invite to an external workspace
    ///
    /// # Arguments
    ///
    /// * `channel` - ID of the channel to share
    /// * `emails` - List of email addresses to invite
    /// * `external_limited` - Whether to invite as external limited members
    pub async fn invite_shared(
        &self,
        channel: &str,
        emails: Option<&[&str]>,
        external_limited: Option<bool>,
        user_ids: Option<&[&str]>,
    ) -> Result<InviteSharedResponse> {
        let params = InviteSharedRequest {
            channel: channel.to_string(),
            emails: emails.map(|e| e.iter().map(|s| s.to_string()).collect()),
            external_limited,
            user_ids: user_ids.map(|u| u.iter().map(|s| s.to_string()).collect()),
        };

        self.client
            .post("conversations.inviteShared", &params)
            .await
    }

    /// List pending Slack Connect channel invites
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor
    /// * `team_id` - Team ID for Enterprise Grid
    pub async fn list_connect_invites(
        &self,
        cursor: Option<&str>,
        team_id: Option<&str>,
    ) -> Result<ListConnectInvitesResponse> {
        let params = ListConnectInvitesRequest {
            cursor: cursor.map(|s| s.to_string()),
            team_id: team_id.map(|s| s.to_string()),
        };

        self.client
            .post("conversations.listConnectInvites", &params)
            .await
    }

    // ============================================
    // Request Shared Invite Methods
    // ============================================

    /// Approve a request to join a Slack Connect channel
    ///
    /// # Arguments
    ///
    /// * `invite_id` - ID of the request to approve
    /// * `channel_id` - ID of the channel (optional)
    pub async fn request_shared_invite_approve(
        &self,
        invite_id: &str,
        channel_id: Option<&str>,
        is_sponsored: Option<bool>,
    ) -> Result<RequestSharedInviteApproveResponse> {
        let params = RequestSharedInviteApproveRequest {
            invite_id: invite_id.to_string(),
            channel_id: channel_id.map(|s| s.to_string()),
            is_sponsored,
        };

        self.client
            .post("conversations.requestSharedInvite.approve", &params)
            .await
    }

    /// Deny a request to join a Slack Connect channel
    ///
    /// # Arguments
    ///
    /// * `invite_id` - ID of the request to deny
    /// * `message` - Optional message explaining denial
    pub async fn request_shared_invite_deny(
        &self,
        invite_id: &str,
        message: Option<&str>,
    ) -> Result<RequestSharedInviteDenyResponse> {
        let params = RequestSharedInviteDenyRequest {
            invite_id: invite_id.to_string(),
            message: message.map(|s| s.to_string()),
        };

        self.client
            .post("conversations.requestSharedInvite.deny", &params)
            .await
    }

    /// List pending requests to join Slack Connect channels
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor
    /// * `include_approved` - Include approved requests
    /// * `include_denied` - Include denied requests
    /// * `limit` - Maximum number of results
    pub async fn request_shared_invite_list(
        &self,
        cursor: Option<&str>,
        include_approved: Option<bool>,
        include_denied: Option<bool>,
        limit: Option<u32>,
    ) -> Result<RequestSharedInviteListResponse> {
        let params = RequestSharedInviteListRequest {
            cursor: cursor.map(|s| s.to_string()),
            include_approved,
            include_denied,
            limit,
        };

        self.client
            .post("conversations.requestSharedInvite.list", &params)
            .await
    }

    // ============================================
    // Canvas Methods
    // ============================================

    /// Create a canvas in a channel
    ///
    /// # Arguments
    ///
    /// * `channel_id` - ID of the channel
    /// * `document_content` - Content of the canvas in markdown or document format
    pub async fn canvases_create(
        &self,
        channel_id: &str,
        document_content: Option<&serde_json::Value>,
    ) -> Result<CanvasesCreateResponse> {
        let params = CanvasesCreateRequest {
            channel_id: channel_id.to_string(),
            document_content: document_content.cloned(),
        };

        self.client
            .post("conversations.canvases.create", &params)
            .await
    }

    /// Set external invite permissions for a Slack Connect channel
    ///
    /// # Arguments
    ///
    /// * `channel` - ID of the channel
    /// * `action` - Permission action (e.g., "upgrade", "downgrade")
    pub async fn external_invite_permissions_set(
        &self,
        channel: &str,
        action: &str,
    ) -> Result<ExternalInvitePermissionsSetResponse> {
        let params = ExternalInvitePermissionsSetRequest {
            channel: channel.to_string(),
            action: action.to_string(),
        };

        self.client
            .post("conversations.externalInvitePermissions.set", &params)
            .await
    }
}

// Request/Response types

#[derive(Debug, Serialize)]
pub struct OpenConversationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_im: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct OpenConversationResponse {
    pub channel: Channel,
}

#[derive(Debug, Serialize)]
pub struct ListConversationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListConversationsResponse {
    pub channels: Vec<Channel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct ConversationInfoResponse {
    pub channel: Channel,
}

#[derive(Debug, Serialize)]
pub struct JoinConversationRequest {
    pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinConversationResponse {
    pub channel: Channel,
}

#[derive(Debug, Serialize)]
pub struct LeaveConversationRequest {
    pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct LeaveConversationResponse {}

#[derive(Debug, Serialize)]
pub struct CreateConversationRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateConversationResponse {
    pub channel: Channel,
}

#[derive(Debug, Serialize)]
pub struct ArchiveConversationRequest {
    pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct ArchiveConversationResponse {}

#[derive(Debug, Serialize)]
pub struct UnarchiveConversationRequest {
    pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct UnarchiveConversationResponse {}

#[derive(Debug, Serialize)]
pub struct ConversationHistoryRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ConversationHistoryResponse {
    pub messages: Vec<Message>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize)]
pub struct ConversationRepliesRequest {
    pub channel: String,
    pub ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConversationRepliesResponse {
    pub messages: Vec<Message>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize)]
pub struct InviteConversationRequest {
    pub channel: String,
    pub users: String,
}

#[derive(Debug, Deserialize)]
pub struct InviteConversationResponse {
    pub channel: Channel,
}

#[derive(Debug, Serialize)]
pub struct KickConversationRequest {
    pub channel: String,
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct KickConversationResponse {}

#[derive(Debug, Serialize)]
pub struct ConversationMembersRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConversationMembersResponse {
    pub members: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize)]
pub struct RenameConversationRequest {
    pub channel: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RenameConversationResponse {
    pub channel: Channel,
}

#[derive(Debug, Serialize)]
pub struct SetPurposeRequest {
    pub channel: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct SetPurposeResponse {
    pub purpose: String,
}

#[derive(Debug, Serialize)]
pub struct SetTopicRequest {
    pub channel: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
pub struct SetTopicResponse {
    pub topic: String,
}

#[derive(Debug, Serialize)]
pub struct MarkConversationRequest {
    pub channel: String,
    pub ts: String,
}

#[derive(Debug, Deserialize)]
pub struct MarkConversationResponse {}

#[derive(Debug, Serialize)]
pub struct CloseConversationRequest {
    pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct CloseConversationResponse {
    #[serde(default)]
    pub no_op: Option<bool>,
    #[serde(default)]
    pub already_closed: Option<bool>,
}

// Slack Connect types

#[derive(Debug, Serialize)]
pub struct AcceptSharedInviteRequest {
    pub channel_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AcceptSharedInviteResponse {
    #[serde(default)]
    pub implicit_approval: Option<bool>,
    #[serde(default)]
    pub channel_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApproveSharedInviteRequest {
    pub invite_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_team: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveSharedInviteResponse {}

#[derive(Debug, Serialize)]
pub struct DeclineSharedInviteRequest {
    pub invite_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_team: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeclineSharedInviteResponse {}

#[derive(Debug, Serialize)]
pub struct InviteSharedRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct InviteSharedResponse {
    #[serde(default)]
    pub invite_id: Option<String>,
    #[serde(default)]
    pub conf_code: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub is_legacy_shared_channel: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ListConnectInvitesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListConnectInvitesResponse {
    #[serde(default)]
    pub invites: Vec<ConnectInvite>,
    #[serde(default)]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectInvite {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub date_created: Option<i64>,
    #[serde(default)]
    pub date_invalid: Option<i64>,
    #[serde(default)]
    pub inviting_team: Option<ConnectInviteTeam>,
    #[serde(default)]
    pub inviting_user: Option<ConnectInviteUser>,
    #[serde(default)]
    pub recipient_email: Option<String>,
    #[serde(default)]
    pub recipient_user_id: Option<String>,
    #[serde(default)]
    pub channel: Option<ConnectInviteChannel>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectInviteTeam {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub icon: Option<serde_json::Value>,
    #[serde(default)]
    pub is_verified: Option<bool>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub date_created: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectInviteUser {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub profile: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectInviteChannel {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub is_private: Option<bool>,
    #[serde(default)]
    pub is_im: Option<bool>,
}

// Request Shared Invite types

#[derive(Debug, Serialize)]
pub struct RequestSharedInviteApproveRequest {
    pub invite_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sponsored: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RequestSharedInviteApproveResponse {
    #[serde(default)]
    pub invite_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RequestSharedInviteDenyRequest {
    pub invite_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RequestSharedInviteDenyResponse {}

#[derive(Debug, Serialize)]
pub struct RequestSharedInviteListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_approved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_denied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct RequestSharedInviteListResponse {
    #[serde(default)]
    pub invites: Vec<SharedInviteRequest>,
    #[serde(default)]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct SharedInviteRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub channel: Option<serde_json::Value>,
    #[serde(default)]
    pub is_sponsored: Option<bool>,
    #[serde(default)]
    pub invite_type: Option<String>,
    #[serde(default)]
    pub date_created: Option<i64>,
    #[serde(default)]
    pub date_last_updated: Option<i64>,
    #[serde(default)]
    pub requesting_user: Option<serde_json::Value>,
    #[serde(default)]
    pub requesting_team: Option<serde_json::Value>,
    #[serde(default)]
    pub target_user: Option<serde_json::Value>,
    #[serde(default)]
    pub status: Option<String>,
}

// Canvas types

#[derive(Debug, Serialize)]
pub struct CanvasesCreateRequest {
    pub channel_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_content: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct CanvasesCreateResponse {
    #[serde(default)]
    pub canvas_id: Option<String>,
}

// External Invite Permissions types

#[derive(Debug, Serialize)]
pub struct ExternalInvitePermissionsSetRequest {
    pub channel: String,
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalInvitePermissionsSetResponse {}
