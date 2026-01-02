use slacko::api::conversations::ListConversationsRequest;
use slacko::{AuthConfig, SlackClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SlackClient::new(AuthConfig::from_env()?)?;

    // List DM conversations
    let params = ListConversationsRequest {
        exclude_archived: Some(false),
        types: Some("im,mpim".to_string()), // im = DMs, mpim = group DMs
        limit: Some(100),
        cursor: None,
    };
    let response = client.conversations().list_with_options(params).await?;

    println!("Your DM conversations:");
    for channel in response.channels {
        let name = channel.name.unwrap_or_else(|| "DM".to_string());
        let id = channel.id;
        println!("  {}: {}", name, id);
    }

    Ok(())
}
