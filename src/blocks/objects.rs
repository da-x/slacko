//! Composition objects for Block Kit
//!
//! These are the building blocks used within blocks and elements.

use serde::{Deserialize, Serialize};

/// Text object for Block Kit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextObject {
    #[serde(rename = "type")]
    pub text_type: TextType,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<bool>,
}

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
            verbatim: None,
        }
    }

    /// Create a markdown text object
    pub fn markdown(text: impl Into<String>) -> Self {
        Self {
            text_type: TextType::Mrkdwn,
            text: text.into(),
            emoji: None,
            verbatim: None,
        }
    }

    /// Enable emoji parsing (for plain_text only)
    pub fn emoji(mut self, enabled: bool) -> Self {
        self.emoji = Some(enabled);
        self
    }

    /// Disable auto-formatting (for mrkdwn only)
    pub fn verbatim(mut self, enabled: bool) -> Self {
        self.verbatim = Some(enabled);
        self
    }
}

/// Option object for select menus and radio buttons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionObject {
    pub text: TextObject,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl OptionObject {
    /// Create a new option
    pub fn new(text: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            text: TextObject::plain(text),
            value: value.into(),
            description: None,
            url: None,
        }
    }

    /// Add a description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(TextObject::plain(desc));
        self
    }

    /// Add a URL (for overflow menu items)
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// Option group for select menus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionGroupObject {
    pub label: TextObject,
    pub options: Vec<OptionObject>,
}

impl OptionGroupObject {
    /// Create a new option group
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: TextObject::plain(label),
            options: Vec::new(),
        }
    }

    /// Add an option to the group
    pub fn option(mut self, option: OptionObject) -> Self {
        self.options.push(option);
        self
    }

    /// Add multiple options
    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options.extend(options);
        self
    }
}

/// Confirmation dialog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationDialog {
    pub title: TextObject,
    pub text: TextObject,
    pub confirm: TextObject,
    pub deny: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl ConfirmationDialog {
    /// Create a new confirmation dialog
    pub fn new(
        title: impl Into<String>,
        text: impl Into<String>,
        confirm: impl Into<String>,
        deny: impl Into<String>,
    ) -> Self {
        Self {
            title: TextObject::plain(title),
            text: TextObject::plain(text),
            confirm: TextObject::plain(confirm),
            deny: TextObject::plain(deny),
            style: None,
        }
    }

    /// Set the style (primary or danger)
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Set as danger style
    pub fn danger(mut self) -> Self {
        self.style = Some("danger".to_string());
        self
    }

    /// Set as primary style
    pub fn primary(mut self) -> Self {
        self.style = Some("primary".to_string());
        self
    }
}

/// Filter for conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_external_shared_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_bot_users: Option<bool>,
}

impl ConversationFilter {
    /// Create a new conversation filter
    pub fn new() -> Self {
        Self {
            include: None,
            exclude_external_shared_channels: None,
            exclude_bot_users: None,
        }
    }

    /// Include specific conversation types
    pub fn include(mut self, types: Vec<String>) -> Self {
        self.include = Some(types);
        self
    }

    /// Exclude external shared channels
    pub fn exclude_external_shared_channels(mut self) -> Self {
        self.exclude_external_shared_channels = Some(true);
        self
    }

    /// Exclude bot users
    pub fn exclude_bot_users(mut self) -> Self {
        self.exclude_bot_users = Some(true);
        self
    }
}

impl Default for ConversationFilter {
    fn default() -> Self {
        Self::new()
    }
}
