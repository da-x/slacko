use slacko::{AuthConfig, SlackClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SlackClient::new(AuthConfig::from_env()?)?;

    // Get our user ID
    let auth = client.auth().test().await?;
    println!("Logged in as: {} ({})", auth.user, auth.user_id);

    // Try to open self-DM
    match client.conversations().open(&[&auth.user_id]).await {
        Ok(response) => {
            println!("Open DM success: channel_id = {}", response.channel.id);
        }
        Err(e) => {
            println!("Open DM failed: {:?}", e);
        }
    }

    Ok(())
}
