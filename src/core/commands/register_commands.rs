use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, Ready,
};
use std::error::Error;

pub async fn register_commands(ctx: &Context, ready: &Ready) -> Result<(), Box<dyn Error>> {
    let available_guilds: Vec<GuildId> = ready.guilds.iter().map(|x| x.id).collect();
    for guild_id in available_guilds.iter() {
        guild_id.set_commands(&ctx.http, get_commands()).await?;
    }

    Ok(())
}

fn get_commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("avaliar")
            .description("Avaliar um jogador")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "player",
                    "Select player that will receive evaluation",
                )
                .required(true),
            ),
        CreateCommand::new("historico")
            .description("Acessar o histórico")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "player",
                    "O player no qual deseja ver o histórico",
                )
                .required(true),
            ),
    ]
}
