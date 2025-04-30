use crate::core::structs::environment::*;
use crate::infrastructure::interaction::Handler;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn configure_discord() {
    let environment = Environment::new(EnvType::DiscordToken).expect("Failed to get token");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILDS;

    let handler = Handler {
        state: Arc::new(Mutex::new(HashMap::new())),
    };

    let mut client = Client::builder(environment.value, intents)
        .event_handler(handler)
        .await
        .expect("Error while creating client");

    client
        .start()
        .await
        .expect("Client failed to start, aborting...");
}
