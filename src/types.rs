//! Common types used across the Slack SDK

use serde::{Deserialize, Serialize};

/// Standard Slack API response wrapper
#[derive(Debug, Deserialize, Serialize)]
pub struct SlackResponse<T> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(flatten)]
    pub data: Option<T>,
}

/// Pagination cursor for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub next_cursor: Option<String>,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_24: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_48: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_72: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_emoji: Option<String>,
}

/// Channel/Conversation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_channel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_im: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_member: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<Topic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_members: Option<u32>,
    /// For DMs (is_im=true), the user ID of the other participant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Channel topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_set: Option<i64>,
}

/// Channel purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purpose {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_set: Option<i64>,
}

/// Message information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(default)]
    pub ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_reply: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<Reaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<Edited>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edited {
    pub user: Option<String>,
    pub ts: Option<String>,
}

/// Message attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
}

/// Reaction to a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub name: String,
    pub count: u32,
    pub users: Vec<String>,
}

/// File information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_private: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_private_download: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

/// Team/Workspace information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<serde_json::Value>,
}

/// RTM connection information
#[derive(Debug, Clone, Deserialize)]
pub struct RtmConnectResponse {
    pub url: String,
    #[serde(rename = "self")]
    pub self_info: SelfInfo,
    pub team: Team,
}

/// Self information from RTM connection
#[derive(Debug, Clone, Deserialize)]
pub struct SelfInfo {
    pub id: String,
    pub name: String,
}

/// Block Kit block (for rich message formatting)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "section")]
    Section {
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<TextObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fields: Option<Vec<TextObject>>,
    },
    #[serde(rename = "divider")]
    Divider {},
    #[serde(rename = "header")]
    Header { text: TextObject },
    #[serde(rename = "context")]
    Context { elements: Vec<TextObject> },
}

/// Text object for Block Kit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextObject {
    #[serde(rename = "type")]
    pub text_type: TextType,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
}

/// Text type for Block Kit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextType {
    PlainText,
    Mrkdwn,
}

impl TextObject {
    /// Create a plain text object
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text_type: TextType::PlainText,
            text: text.into(),
            emoji: None,
        }
    }

    /// Create a markdown text object
    pub fn markdown(text: impl Into<String>) -> Self {
        Self {
            text_type: TextType::Mrkdwn,
            text: text.into(),
            emoji: None,
        }
    }
}
