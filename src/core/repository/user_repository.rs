use crate::core::entities::user::DiscordUser;
use crate::database::entities;
use crate::infrastructure::database::entities::prelude::Discord;
use crate::infrastructure::database::entities::prelude::User;
use sea_orm::ActiveValue;
use sea_orm::prelude::*;
pub struct UserRepository {}

impl UserRepository {
    pub async fn get_by_discord_id<C: ConnectionTrait>(
        db: &C,
        id: String,
    ) -> Result<Option<DiscordUser>, DbErr> {
        let discord_with_user = Discord::find()
            .filter(entities::discord::Column::Id.eq(id))
            .find_also_related(User)
            .one(db)
            .await?;

        if let Some((discord, Some(user))) = discord_with_user {
            return Ok(Some(DiscordUser { discord, user }));
        }

        Ok(None)
    }

    pub async fn create<C: ConnectionTrait>(
        db: &C,
        id: String,
        name: String,
        discriminator: Option<String>,
        email: Option<String>,
        global_name: Option<String>,
    ) -> Result<DiscordUser, DbErr> {
        let discord = entities::discord::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            name: ActiveValue::Set(name.clone()),
            discriminator: ActiveValue::Set(discriminator),
            email: ActiveValue::Set(email),
            global_name: ActiveValue::Set(global_name),
            ..Default::default()
        }
        .insert(db)
        .await?;

        let user = entities::user::ActiveModel {
            id: ActiveValue::Set(id),
            name: ActiveValue::Set(name),
        }
        .insert(db)
        .await?;

        let user = DiscordUser { discord, user };

        Ok(user)
    }
}
