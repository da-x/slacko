//! Block Kit builders
//!
//! This module provides builder patterns for Slack's Block Kit components.
//! Block Kit allows you to create rich, interactive messages with buttons,
//! select menus, date pickers, and more.
//!
//! # Example
//!
//! ```no_run
//! use slacko::blocks::{MessageBuilder, SectionBlock, ActionsBlock};
//!
//! let message = MessageBuilder::new()
//!     .text("Fallback text")
//!     .block(SectionBlock::new()
//!         .markdown("*Welcome* to the team!")
//!         .build())
//!     .block(ActionsBlock::new()
//!         .button("approve_btn", "Approve")
//!         .button("reject_btn", "Reject")
//!         .build())
//!     .build();
//! ```

pub mod elements;
pub mod layout;
pub mod objects;

pub use elements::{
    ButtonElement, CheckboxesElement, DatePickerElement, DatetimePickerElement, MultiSelectElement,
    OverflowElement, PlainTextInputElement, RadioButtonsElement, SelectElement, TimePickerElement,
};
pub use layout::{
    ActionsBlock, ContextBlock, DividerBlock, FileBlock, HeaderBlock, ImageBlock, InputBlock,
    MessageBuilder, SectionBlock, VideoBlock,
};
pub use objects::{ConfirmationDialog, OptionGroupObject, OptionObject, TextObject};
