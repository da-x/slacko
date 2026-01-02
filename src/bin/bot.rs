//! Example Slack bot using the SDK
//!
//! This bot joins a channel and responds to messages.

use slacko::{AuthConfig, SlackClient};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    info!("Starting Slack Bot using SDK");

    // Load authentication from environment
    let auth = AuthConfig::from_env()?;
    info!("Authenticated using {} mode", auth.auth_type_str());

    // Create Slack client
    let client = SlackClient::new(auth)?;

    // Get the channel to join from environment
    let channel = env::var("SLACK_CHANNEL").unwrap_or_else(|_| {
        eprintln!("SLACK_CHANNEL environment variable not set");
        std::process::exit(1);
    });

    info!("Joining channel: {}", channel);

    // Join the channel
    match client.conversations().join(&channel).await {
        Ok(_) => info!("Successfully joined channel"),
        Err(e) => {
            if e.to_string().contains("already_in_channel") {
                info!("Already in channel");
            } else {
                return Err(e.into());
            }
        }
    }

    // Start RTM and listen for messages in the channel
    info!("Connecting to Slack RTM API");

    let client_clone = client.clone();
    let channel_clone = channel.clone();

    client
        .rtm()
        .start_with_channel(&channel, move |event| {
            let message_text = match &event.text {
                Some(text) => text,
                None => return,
            };

            info!("Received message: {}", message_text);

            // Generate response
            let response = format!("I received your message: \"{}\"", message_text);

            // Post response (in thread if applicable)
            let thread_ts = event.thread_ts.or(event.ts);

            let client = client_clone.clone();
            let channel = channel_clone.clone();

            tokio::spawn(async move {
                let mut request =
                    slacko::api::chat::PostMessageRequest::new(&channel).text(&response);
                request.thread_ts = thread_ts;

                if let Err(e) = client.chat().post_message_with_options(request).await {
                    eprintln!("Failed to post message: {}", e);
                }
            });
        })
        .await?;

    Ok(())
}
