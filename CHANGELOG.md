# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-02

### Added

- Initial release of slacko with complete Slack API coverage
- Authentication support:
  - OAuth tokens (xoxp, xoxb)
  - Stealth mode (xoxc/xoxd browser session tokens)
  - Environment variable auto-detection
- 24 API modules:
  - admin - Enterprise Grid administration
  - apps - App management and permissions
  - auth - Authentication verification
  - bookmarks - Channel bookmarks
  - calls - Slack Calls integration
  - chat - Messages, threads, scheduled messages
  - conversations - Channels, DMs, groups, history
  - dialog - Legacy dialogs
  - dnd - Do Not Disturb settings
  - emoji - Custom emoji listing
  - files - File upload, download, management
  - oauth - OAuth v2 token exchange
  - openid - OpenID Connect authentication
  - pins - Pinned messages
  - reactions - Emoji reactions
  - reminders - Reminder management
  - rtm - Real-time messaging via WebSocket
  - search - Message and file search
  - stars - Starred items
  - team - Team information
  - usergroups - User group management
  - users - User info, presence, profiles
  - views - Modals and App Home
  - workflows - Workflow Builder integration
- Block Kit builders for rich message layouts:
  - Section, Header, Divider, Image, Context blocks
  - Button, Select, DatePicker elements
  - Confirmation dialogs
  - Message builder with fluent API
- Error handling with typed errors:
  - API errors with method context
  - Rate limit detection with retry-after
  - Network and parsing errors
- Browser extension for stealth token extraction
- Integration tests for all API modules
