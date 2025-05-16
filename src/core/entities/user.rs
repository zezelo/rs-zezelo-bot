use crate::infrastructure::database::entities::{discord, user};

pub struct DiscordUser {
    pub discord: discord::Model,
    pub user: user::Model
}