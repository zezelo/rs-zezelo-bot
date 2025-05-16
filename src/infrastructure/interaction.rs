use crate::core::commands::evaluate::evaluate;
use crate::core::commands::history::get_evaluation_history;
use crate::core::commands::*;
use crate::core::enums::discord::{DiscordCommand, DiscordCustomId};
use crate::core::handlers::user_handler::UserHandler;
use serenity::all::{EventHandler, Interaction, Ready};
use serenity::async_trait;
use serenity::prelude::*;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct Handler {
    pub state: Arc<Mutex<HashMap<String, Box<dyn Any + Send + Sync>>>>,
}

impl Handler {
    async fn add_state(&mut self, custom_id: String, value: Box<dyn Any + Send + Sync>) {
        let state_clone = Arc::clone(&self.state);
        let custom_id_clone = custom_id.clone();

        tokio::spawn(async move {
            sleep(Duration::from_secs(30)).await;

            let mut state = state_clone.lock().await;
            state.remove(&custom_id_clone);
        });

        let mut state = self.state.lock().await;
        state.insert(custom_id, value);
    }

    async fn remove(&mut self, key: String) {
        let mut state = &mut self.state.lock().await;

        state.remove(&key);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        register_commands::register_commands(&ctx, &ready)
            .await
            .expect("Failed to register commands");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        UserHandler::preregister(&interaction)
            .await
            .expect("Failed to verify user");

        match interaction {
            Interaction::Command(command) => {
                if command.data.name == DiscordCommand::Evaluate.as_str() {
                    evaluate(&ctx, &command).await.expect("Failed to evaluate")
                }
                if command.data.name == DiscordCommand::History.as_str() {
                    get_evaluation_history(&ctx, &command)
                        .await
                        .expect("Failed to get history")
                }
            }
            Interaction::Modal(modal) => match modal.data.custom_id.split_once("|") {
                Some((kind, id)) => {
                    if kind == DiscordCustomId::Evaluate.as_str() {
                        let parsed_id = id.parse::<u64>().expect("Failed to parse id");

                        UserHandler::preregister_target(&ctx, parsed_id)
                            .await
                            .expect("Failed to verify user");

                        evaluate::publish_at_evaluation_channel(&ctx, &modal, id)
                            .await
                            .expect("Failed to send evaluation command");
                    }
                }
                _ => {
                    println!("Comando não identificado!")
                }
            },
            _ => {
                println!("Tipo de comando não delimitado");
            }
        }
    }
}
