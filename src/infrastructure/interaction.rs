use crate::core::commands::evaluate::evaluate;
use crate::core::commands::*;
use crate::core::enums::discord::{DiscordCommand, DiscordCustomId};
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
        match interaction {
            Interaction::Command(command) => {
                if command.data.name == DiscordCommand::Evaluate.as_str() {
                    evaluate(&ctx, &command).await.expect("Failed to evaluate")
                }
            }
            Interaction::Modal(modal) => {
                if modal.data.custom_id == DiscordCustomId::CreateEvaluateModal.as_str() {
                    evaluate::publish_at_evaluation_channel(&ctx, &modal)
                        .await
                        .expect("Failed to send evaluation command");
                }
            }
            _ => {}
        }
    }
}
