use crate::core::structs::environment::*;
use serenity::prelude::*;

pub async fn configure_discord() {
    let environment = Environment::new(EnvType::DiscordToken).expect("Failed to get token");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS;

    let mut client = Client::builder(environment.value, intents)
        .await
        .expect("Error while creating client");

    client
        .start()
        .await
        .expect("CLient failed to start, aborting...");
}
