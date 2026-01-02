# slacko

A comprehensive Rust SDK for the Slack API with support for both standard OAuth tokens and stealth mode authentication.

[![Crates.io](https://img.shields.io/crates/v/slacko.svg)](https://crates.io/crates/slacko)
[![Documentation](https://docs.rs/slacko/badge.svg)](https://docs.rs/slacko)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Multiple authentication methods: OAuth tokens (xoxp, xoxb) and stealth mode (xoxc/xoxd)
- Complete API coverage: 24 API modules covering all major Slack functionality
- Real-time messaging via WebSocket (RTM API)
- Block Kit builders for rich message layouts
- Strongly typed requests and responses
- Built on tokio for async/await support
- Automatic rate limit detection

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
slacko = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### OAuth Authentication

```rust
use slacko::{SlackClient, AuthConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using a bot token
    let client = SlackClient::new(AuthConfig::bot("xoxb-your-token"))?;

    // Post a message
    client.chat().post_message("#general", "Hello from Rust!").await?;

    // List channels
    let channels = client.conversations().list().await?;
    for channel in channels.channels {
        println!("#{}", channel.name.unwrap_or_default());
    }

    Ok(())
}
```

### Stealth Mode Authentication

```rust
use slacko::{SlackClient, AuthConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SlackClient::new(
        AuthConfig::stealth("xoxc-your-token", "xoxd-your-cookie")
    )?;

    let user = client.auth().test().await?;
    println!("Authenticated as {}", user.user);

    Ok(())
}
```

### Environment Variables

```bash
# OAuth mode
export SLACK_BOT_TOKEN=xoxb-...
# or
export SLACK_TOKEN=xoxp-...

# Stealth mode
export SLACK_XOXC_TOKEN=xoxc-...
export SLACK_XOXD_COOKIE=xoxd-...
```

```rust
let client = SlackClient::new(AuthConfig::from_env()?)?;
```

## API Examples

### Chat

```rust
// Post message
client.chat().post_message("#general", "Hello!").await?;

// Reply in thread
client.chat().post_reply("#general", "1234567890.123456", "Reply").await?;

// Update message
client.chat().update_message("#general", "1234567890.123456", "Updated").await?;

// Delete message
client.chat().delete_message("#general", "1234567890.123456").await?;
```

### Conversations

```rust
// List channels
let channels = client.conversations().list().await?;

// Create channel
client.conversations().create("new-channel", false).await?;

// Get history
let history = client.conversations().history("C12345678").await?;

// Invite users
client.conversations().invite("C12345678", &["U12345678"]).await?;
```

### Users

```rust
// List users
let users = client.users().list().await?;

// Get user info
let user = client.users().info("U12345678").await?;

// Set presence
client.users().set_presence("away").await?;
```

### Files

```rust
// Upload file
client.files().upload(content, "file.pdf", Some(&["C12345678"])).await?;

// List files
let files = client.files().list().await?;
```

### Reactions

```rust
// Add reaction
client.reactions().add("C12345678", "1234567890.123456", "thumbsup").await?;

// Remove reaction
client.reactions().remove("C12345678", "1234567890.123456", "thumbsup").await?;
```

### Search

```rust
// Search messages
let results = client.search().messages("query").await?;

// Search files
let files = client.search().files("filename").await?;
```

### Real-Time Messaging

```rust
// Connect and listen for messages
client.rtm().start(|message| {
    println!("Received: {:?}", message.text);
}).await?;
```

### Block Kit

```rust
use slacko::{MessageBuilder, SectionBlock, TextObject, ActionsBlock, ButtonElement};

let blocks = MessageBuilder::new()
    .add_header(HeaderBlock::new("Status Update"))
    .add_section(
        SectionBlock::new(TextObject::markdown("*Build succeeded*"))
    )
    .add_divider()
    .add_actions(
        ActionsBlock::new()
            .add_button(ButtonElement::new("view", "View Details"))
    )
    .build();

client.chat().post_message_with_blocks("#general", "Build update", blocks).await?;
```

## API Modules

| Module | Description |
|--------|-------------|
| `admin` | Enterprise Grid administration |
| `apps` | App management and permissions |
| `auth` | Authentication verification |
| `bookmarks` | Channel bookmarks |
| `calls` | Slack Calls integration |
| `chat` | Messages and threads |
| `conversations` | Channels, DMs, and groups |
| `dialog` | Legacy dialogs |
| `dnd` | Do Not Disturb settings |
| `emoji` | Custom emoji |
| `files` | File uploads and management |
| `oauth` | OAuth token exchange |
| `openid` | OpenID Connect authentication |
| `pins` | Pinned messages |
| `reactions` | Emoji reactions |
| `reminders` | Reminders |
| `rtm` | Real-time messaging |
| `search` | Message and file search |
| `stars` | Starred items |
| `team` | Team information |
| `usergroups` | User groups |
| `users` | User information and presence |
| `views` | Modals and App Home |
| `workflows` | Workflow Builder |

## Error Handling

```rust
use slacko::SlackError;

match client.chat().post_message("#general", "Hello").await {
    Ok(response) => println!("Sent: {}", response.ts),
    Err(SlackError::RateLimitExceeded { retry_after }) => {
        println!("Rate limited, retry after {} seconds", retry_after);
    }
    Err(SlackError::ApiError { method, message }) => {
        println!("API error in {}: {}", method, message);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Obtaining Tokens

### OAuth Tokens (Recommended)

1. Create a Slack App at https://api.slack.com/apps
2. Add required OAuth scopes under "OAuth & Permissions"
3. Install the app to your workspace
4. Copy the Bot Token (xoxb-...) or User Token (xoxp-...)

### Stealth Mode Tokens

Use the included browser extension for easy token extraction:

1. Load the extension from `browser-extension/` in Chrome (chrome://extensions > Load unpacked)
2. Navigate to your Slack workspace in the browser
3. Click the extension icon and copy the tokens

Or extract manually:

1. Open Slack in your browser
2. Open Developer Tools (F12)
3. Go to Application > Cookies, find the `d` cookie (xoxd value)
4. Go to Network tab, find any API request with `token` parameter (xoxc value)

## Running Tests

```bash
# Set up credentials
cp .env.example .env
# Edit .env with your tokens

# Run tests
cargo test --tests
```

## License

MIT License. See [LICENSE](LICENSE) for details.
