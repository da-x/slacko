use slacko::{AuthConfig, SlackClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load from environment
    let client = SlackClient::new(AuthConfig::from_env()?)?;

    // Post message to DM
    let response = client
        .chat()
        .post_message("DG4599EJ3", "Hello from the Slack SDK! 🚀")
        .await?;

    println!("Message sent! ts: {}", response.ts);
    Ok(())
}
