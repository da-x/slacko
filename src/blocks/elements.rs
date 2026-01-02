//! Interactive elements for Block Kit
//!
//! These are the interactive components that can appear in blocks.

use super::objects::{ConfirmationDialog, OptionObject, TextObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Button element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonElement {
    #[serde(rename = "type")]
    type_field: String,
    pub text: TextObject,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_label: Option<String>,
}

impl ButtonElement {
    /// Create a new button
    pub fn new(action_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            type_field: "button".to_string(),
            text: TextObject::plain(text),
            action_id: action_id.into(),
            url: None,
            value: None,
            style: None,
            confirm: None,
            accessibility_label: None,
        }
    }

    /// Set the button URL
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the button value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the button style
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Make button primary style
    pub fn primary(mut self) -> Self {
        self.style = Some("primary".to_string());
        self
    }

    /// Make button danger style
    pub fn danger(mut self) -> Self {
        self.style = Some("danger".to_string());
        self
    }

    /// Add confirmation dialog
    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("ButtonElement is always serializable")
    }
}

/// Static select menu element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    pub placeholder: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<OptionObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl SelectElement {
    /// Create a static select menu
    pub fn new(action_id: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            type_field: "static_select".to_string(),
            action_id: action_id.into(),
            placeholder: TextObject::plain(placeholder),
            options: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Create a users select menu
    pub fn users(action_id: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            type_field: "users_select".to_string(),
            action_id: action_id.into(),
            placeholder: TextObject::plain(placeholder),
            options: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Create a conversations select menu
    pub fn conversations(action_id: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            type_field: "conversations_select".to_string(),
            action_id: action_id.into(),
            placeholder: TextObject::plain(placeholder),
            options: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Create a channels select menu
    pub fn channels(action_id: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            type_field: "channels_select".to_string(),
            action_id: action_id.into(),
            placeholder: TextObject::plain(placeholder),
            options: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Add options (for static select)
    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options = Some(options);
        self
    }

    /// Set initial option
    pub fn initial_option(mut self, option: OptionObject) -> Self {
        self.initial_option = Some(option);
        self
    }

    /// Add confirmation dialog
    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Focus on load
    pub fn focus_on_load(mut self) -> Self {
        self.focus_on_load = Some(true);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("SelectElement is always serializable")
    }
}

/// Multi-select menu element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSelectElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    pub placeholder: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<OptionObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_options: Option<Vec<OptionObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl MultiSelectElement {
    /// Create a multi-select menu
    pub fn new(action_id: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            type_field: "multi_static_select".to_string(),
            action_id: action_id.into(),
            placeholder: TextObject::plain(placeholder),
            options: None,
            initial_options: None,
            max_selected_items: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Add options
    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options = Some(options);
        self
    }

    /// Set max selected items
    pub fn max_selected_items(mut self, max: u32) -> Self {
        self.max_selected_items = Some(max);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("MultiSelectElement is always serializable")
    }
}

/// Overflow menu element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverflowElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    pub options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
}

impl OverflowElement {
    /// Create an overflow menu
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            type_field: "overflow".to_string(),
            action_id: action_id.into(),
            options: Vec::new(),
            confirm: None,
        }
    }

    /// Add an option
    pub fn option(mut self, option: OptionObject) -> Self {
        self.options.push(option);
        self
    }

    /// Add multiple options
    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options.extend(options);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("OverflowElement is always serializable")
    }
}

/// Date picker element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatePickerElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl DatePickerElement {
    /// Create a date picker
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            type_field: "datepicker".to_string(),
            action_id: action_id.into(),
            placeholder: None,
            initial_date: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Set placeholder
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(TextObject::plain(text));
        self
    }

    /// Set initial date (YYYY-MM-DD)
    pub fn initial_date(mut self, date: impl Into<String>) -> Self {
        self.initial_date = Some(date.into());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("DatePickerElement is always serializable")
    }
}

/// Time picker element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePickerElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl TimePickerElement {
    /// Create a time picker
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            type_field: "timepicker".to_string(),
            action_id: action_id.into(),
            placeholder: None,
            initial_time: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Set placeholder
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(TextObject::plain(text));
        self
    }

    /// Set initial time (HH:mm)
    pub fn initial_time(mut self, time: impl Into<String>) -> Self {
        self.initial_time = Some(time.into());
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("TimePickerElement is always serializable")
    }
}

/// Datetime picker element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatetimePickerElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl DatetimePickerElement {
    /// Create a datetime picker
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            type_field: "datetimepicker".to_string(),
            action_id: action_id.into(),
            initial_date_time: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Set initial datetime (Unix timestamp)
    pub fn initial_date_time(mut self, timestamp: i64) -> Self {
        self.initial_date_time = Some(timestamp);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("DatetimePickerElement is always serializable")
    }
}

/// Plain text input element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainTextInputElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl PlainTextInputElement {
    /// Create a plain text input
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            type_field: "plain_text_input".to_string(),
            action_id: action_id.into(),
            placeholder: None,
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
            dispatch_action_config: None,
            focus_on_load: None,
        }
    }

    /// Set placeholder
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(TextObject::plain(text));
        self
    }

    /// Set initial value
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.initial_value = Some(value.into());
        self
    }

    /// Enable multiline
    pub fn multiline(mut self) -> Self {
        self.multiline = Some(true);
        self
    }

    /// Set min length
    pub fn min_length(mut self, length: u32) -> Self {
        self.min_length = Some(length);
        self
    }

    /// Set max length
    pub fn max_length(mut self, length: u32) -> Self {
        self.max_length = Some(length);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("PlainTextInputElement is always serializable")
    }
}

/// Radio buttons element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioButtonsElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    pub options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl RadioButtonsElement {
    /// Create radio buttons
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            type_field: "radio_buttons".to_string(),
            action_id: action_id.into(),
            options: Vec::new(),
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Add an option
    pub fn option(mut self, option: OptionObject) -> Self {
        self.options.push(option);
        self
    }

    /// Add multiple options
    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options.extend(options);
        self
    }

    /// Set initial option
    pub fn initial_option(mut self, option: OptionObject) -> Self {
        self.initial_option = Some(option);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("RadioButtonsElement is always serializable")
    }
}

/// Checkboxes element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxesElement {
    #[serde(rename = "type")]
    type_field: String,
    pub action_id: String,
    pub options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_options: Option<Vec<OptionObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl CheckboxesElement {
    /// Create checkboxes
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            type_field: "checkboxes".to_string(),
            action_id: action_id.into(),
            options: Vec::new(),
            initial_options: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    /// Add an option
    pub fn option(mut self, option: OptionObject) -> Self {
        self.options.push(option);
        self
    }

    /// Add multiple options
    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options.extend(options);
        self
    }

    /// Convert to JSON value
    pub fn build(self) -> Value {
        serde_json::to_value(self).expect("CheckboxesElement is always serializable")
    }
}
