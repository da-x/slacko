//! Block builders for Block Kit
//!
//! Blocks are visual components that can be stacked and arranged to create app layouts.

use super::objects::TextObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Section block - displays text and optional accessory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionBlock {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TextObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory: Option<Value>,
}

impl SectionBlock {
    /// Create a new section block
    pub fn new() -> Self {
        Self {
            type_field: "section".to_string(),
            text: None,
            block_id: None,
            fields: None,
            accessory: None,
        }
    }

    /// Set plain text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(TextObject::plain(text));
        self
    }

    /// Set markdown text
    pub fn markdown(mut self, text: impl Into<String>) -> Self {
        self.text = Some(TextObject::markdown(text));
        self
    }

    /// Set text object directly
    pub fn text_object(mut self, text: TextObject) -> Self {
        self.text = Some(text);
        self
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Add fields (up to 10)
    pub fn fields(mut self, fields: Vec<TextObject>) -> Self {
        self.fields = Some(fields);
        self
    }

    /// Add a field
    pub fn field(mut self, field: TextObject) -> Self {
        self.fields.get_or_insert_with(Vec::new).push(field);
        self
    }

    /// Set accessory element
    pub fn accessory(mut self, accessory: Value) -> Self {
        self.accessory = Some(accessory);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("SectionBlock is always serializable")
    }
}

impl Default for SectionBlock {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions block - holds interactive elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsBlock {
    #[serde(rename = "type")]
    type_field: String,
    pub elements: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl ActionsBlock {
    /// Create a new actions block
    pub fn new() -> Self {
        Self {
            type_field: "actions".to_string(),
            elements: Vec::new(),
            block_id: None,
        }
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Add an element
    pub fn element(mut self, element: Value) -> Self {
        self.elements.push(element);
        self
    }

    /// Add multiple elements
    pub fn elements(mut self, elements: Vec<Value>) -> Self {
        self.elements.extend(elements);
        self
    }

    /// Add a button (convenience method)
    pub fn button(mut self, action_id: impl Into<String>, text: impl Into<String>) -> Self {
        use super::elements::ButtonElement;
        self.elements
            .push(ButtonElement::new(action_id, text).build());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("ActionsBlock is always serializable")
    }
}

impl Default for ActionsBlock {
    fn default() -> Self {
        Self::new()
    }
}

/// Context block - displays contextual info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBlock {
    #[serde(rename = "type")]
    type_field: String,
    pub elements: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl ContextBlock {
    /// Create a new context block
    pub fn new() -> Self {
        Self {
            type_field: "context".to_string(),
            elements: Vec::new(),
            block_id: None,
        }
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Add a text element
    pub fn text(mut self, text: TextObject) -> Self {
        self.elements
            .push(serde_json::to_value(text).expect("TextObject is always serializable"));
        self
    }

    /// Add markdown text (convenience)
    pub fn markdown(mut self, text: impl Into<String>) -> Self {
        self.elements.push(
            serde_json::to_value(TextObject::markdown(text))
                .expect("TextObject is always serializable"),
        );
        self
    }

    /// Add an image
    pub fn image(mut self, image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        self.elements.push(serde_json::json!({
            "type": "image",
            "image_url": image_url.into(),
            "alt_text": alt_text.into()
        }));
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("ContextBlock is always serializable")
    }
}

impl Default for ContextBlock {
    fn default() -> Self {
        Self::new()
    }
}

/// Divider block - visual separator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividerBlock {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl DividerBlock {
    /// Create a new divider block
    pub fn new() -> Self {
        Self {
            type_field: "divider".to_string(),
            block_id: None,
        }
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("DividerBlock is always serializable")
    }
}

impl Default for DividerBlock {
    fn default() -> Self {
        Self::new()
    }
}

/// Header block - large text header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderBlock {
    #[serde(rename = "type")]
    type_field: String,
    pub text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl HeaderBlock {
    /// Create a new header block
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            type_field: "header".to_string(),
            text: TextObject::plain(text),
            block_id: None,
        }
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("HeaderBlock is always serializable")
    }
}

/// Image block - displays an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageBlock {
    #[serde(rename = "type")]
    type_field: String,
    pub image_url: String,
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl ImageBlock {
    /// Create a new image block
    pub fn new(image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        Self {
            type_field: "image".to_string(),
            image_url: image_url.into(),
            alt_text: alt_text.into(),
            title: None,
            block_id: None,
        }
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(TextObject::plain(title));
        self
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("ImageBlock is always serializable")
    }
}

/// Input block - for collecting user input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputBlock {
    #[serde(rename = "type")]
    type_field: String,
    pub label: TextObject,
    pub element: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action: Option<bool>,
}

impl InputBlock {
    /// Create a new input block
    pub fn new(label: impl Into<String>, element: Value) -> Self {
        Self {
            type_field: "input".to_string(),
            label: TextObject::plain(label),
            element,
            block_id: None,
            hint: None,
            optional: None,
            dispatch_action: None,
        }
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Set hint text
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(TextObject::plain(hint));
        self
    }

    /// Mark as optional
    pub fn optional(mut self) -> Self {
        self.optional = Some(true);
        self
    }

    /// Enable dispatch action
    pub fn dispatch_action(mut self) -> Self {
        self.dispatch_action = Some(true);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("InputBlock is always serializable")
    }
}

/// File block - displays a remote file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileBlock {
    #[serde(rename = "type")]
    type_field: String,
    pub external_id: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl FileBlock {
    /// Create a new file block
    pub fn new(external_id: impl Into<String>) -> Self {
        Self {
            type_field: "file".to_string(),
            external_id: external_id.into(),
            source: "remote".to_string(),
            block_id: None,
        }
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("FileBlock is always serializable")
    }
}

/// Video block - displays a video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoBlock {
    #[serde(rename = "type")]
    type_field: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub alt_text: String,
    pub title: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl VideoBlock {
    /// Create a new video block
    pub fn new(
        video_url: impl Into<String>,
        thumbnail_url: impl Into<String>,
        alt_text: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            type_field: "video".to_string(),
            video_url: video_url.into(),
            thumbnail_url: thumbnail_url.into(),
            alt_text: alt_text.into(),
            title: TextObject::plain(title),
            title_url: None,
            author_name: None,
            provider_name: None,
            provider_icon_url: None,
            description: None,
            block_id: None,
        }
    }

    /// Set title URL
    pub fn title_url(mut self, url: impl Into<String>) -> Self {
        self.title_url = Some(url.into());
        self
    }

    /// Set author name
    pub fn author_name(mut self, name: impl Into<String>) -> Self {
        self.author_name = Some(name.into());
        self
    }

    /// Set provider name
    pub fn provider_name(mut self, name: impl Into<String>) -> Self {
        self.provider_name = Some(name.into());
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(TextObject::plain(desc));
        self
    }

    /// Set block ID
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("VideoBlock is always serializable")
    }
}

/// Message builder - composes a complete Block Kit message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn: Option<bool>,
}

impl MessageBuilder {
    /// Create a new message builder
    pub fn new() -> Self {
        Self {
            text: None,
            blocks: None,
            thread_ts: None,
            mrkdwn: None,
        }
    }

    /// Set fallback text (required for notifications)
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Add a block
    pub fn block(mut self, block: Value) -> Self {
        self.blocks.get_or_insert_with(Vec::new).push(block);
        self
    }

    /// Add multiple blocks
    pub fn blocks(mut self, blocks: Vec<Value>) -> Self {
        self.blocks.get_or_insert_with(Vec::new).extend(blocks);
        self
    }

    /// Set thread timestamp (reply in thread)
    pub fn thread_ts(mut self, ts: impl Into<String>) -> Self {
        self.thread_ts = Some(ts.into());
        self
    }

    /// Enable markdown in text
    pub fn mrkdwn(mut self) -> Self {
        self.mrkdwn = Some(true);
        self
    }

    /// Quick helper: add a header
    pub fn header(self, text: impl Into<String>) -> Self {
        self.block(HeaderBlock::new(text).build())
    }

    /// Quick helper: add a section with markdown
    pub fn section(self, text: impl Into<String>) -> Self {
        self.block(SectionBlock::new().markdown(text).build())
    }

    /// Quick helper: add a divider
    pub fn divider(self) -> Self {
        self.block(DividerBlock::new().build())
    }

    /// Quick helper: add an image
    pub fn image(self, url: impl Into<String>, alt: impl Into<String>) -> Self {
        self.block(ImageBlock::new(url, alt).build())
    }

    /// Build into JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("MessageBuilder is always serializable")
    }

    /// Build into blocks-only (for Views API)
    pub fn build_blocks(self) -> Vec<Value> {
        self.blocks.unwrap_or_default()
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
