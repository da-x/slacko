use slacko::{AuthConfig, SlackClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SlackClient::new(AuthConfig::from_env()?)?;

    // Get our user ID
    let auth = client.auth().test().await?;
    println!("Logged in as: {} ({})", auth.user, auth.user_id);

    // Send message to self-DM (channel ID found via conversations.open)
    // Your self-DM channel: DPDKJAZT6
    let response = client
        .chat()
        .post_message("DPDKJAZT6", "Hello from the Slack SDK! 🚀 (sent via Rust)")
        .await?;

    println!("Message sent! ts: {}", response.ts);
    Ok(())
}
