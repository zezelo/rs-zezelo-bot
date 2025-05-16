use crate::core::repository::user_repository::UserRepository;
use crate::core::structs::database::DatabaseInstance;
use sea_orm::DbErr;
use serenity::all::{Interaction, User, UserId};
use serenity::prelude::Context;

#[derive(Default)]
pub struct UserHandler {}

impl UserHandler {
    pub async fn create_user_if_not_exists(user: Option<&User>) -> Result<(), DbErr> {
        let instance = DatabaseInstance::new().db;

        if let Some(user) = user {
            let discord =
                UserRepository::get_by_discord_id(instance.as_ref(), user.id.to_string()).await?;

            if let None = discord {
                UserRepository::create(
                    instance.as_ref(),
                    user.id.to_string(),
                    user.name.clone(),
                    user.discriminator.map(|d| d.to_string()),
                    user.email.clone(),
                    user.global_name.clone(),
                )
                .await?;
            }
        }

        Ok(())
    }

    pub async fn preregister(interaction: &Interaction) -> Result<(), DbErr> {
        let user = match interaction {
            Interaction::Command(i) => Some(&i.user),
            Interaction::Component(i) => Some(&i.user),
            Interaction::Autocomplete(i) => Some(&i.user),
            Interaction::Modal(i) => Some(&i.user),
            _ => None,
        };

        Self::create_user_if_not_exists(user).await?;

        Ok(())
    }

    pub async fn preregister_target(ctx: &Context, target_discord_id: u64) -> Result<(), DbErr> {
        let user_id = UserId::new(target_discord_id);

        let user = ctx
            .http
            .get_user(user_id)
            .await
            .expect("Failed to get user");

        Self::create_user_if_not_exists(Some(&user)).await?;

        Ok(())
    }
}
