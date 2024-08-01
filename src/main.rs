use mastodon_async::prelude::*;
use mastodon_async::helpers::toml;
use mastodon_async::{helpers::cli, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register().await?
    };

    let you = mastodon.verify_credentials().await?;
    println!("{:?}", you);

    Ok(())
}

async fn register() -> Result<Mastodon> {
    let registration = Registration::new("https://botsin.space")
        .client_name("mastodon-async-examples")
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
}
