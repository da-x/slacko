use slacko::{AuthConfig, SlackClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SlackClient::new(AuthConfig::from_env()?)?;

    // Test auth to get user info
    let response = client.auth().test().await?;

    println!("Auth test:");
    println!("  User: {}", response.user);
    println!("  User ID: {}", response.user_id);
    println!("  Team: {}", response.team);
    println!("  Team ID: {}", response.team_id);

    Ok(())
}
