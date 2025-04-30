use crate::core::enums::discord::DiscordCustomId;
use crate::core::structs::environment::{EnvType, Environment};
use serenity::all::*;
use std::error::Error;

fn create_action_rows() -> Vec<CreateActionRow> {
    let labels = [
        "Comunicação do jogador",
        "Trabalho em equipe do jogador",
        "Comportamento do jogador",
        "Utilitária do jogador",
    ];

    let enums = [
        DiscordCustomId::EvaluateCommunication,
        DiscordCustomId::EvaluateTeamWork,
        DiscordCustomId::EvaluateBehaviour,
        DiscordCustomId::EvaluateGrenade,
    ];

    let commentary = CreateActionRow::InputText(CreateInputText::new(
        InputTextStyle::Paragraph,
        "Comentário",
        DiscordCustomId::EvaluateCommentary.as_str(),
    ));

    let mut action_rows: Vec<CreateActionRow> = enums
        .iter()
        .enumerate()
        .map(|(i, custom_id)| {
            CreateActionRow::InputText(CreateInputText::new(
                InputTextStyle::Short,
                labels[i],
                custom_id.as_str(),
            ))
        })
        .collect();

    action_rows.extend([commentary]);

    action_rows
}

pub async fn evaluate(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn Error>> {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Modal(
                CreateModal::new(
                    DiscordCustomId::CreateEvaluateModal.as_str(),
                    "Avaliação de jogador",
                )
                .components(create_action_rows()),
            ),
        )
        .await?;

    Ok(())
}

pub async fn publish_at_evaluation_channel(
    ctx: &Context,
    interaction: &ModalInteraction,
) -> Result<(), Box<dyn Error>> {
    let env = Environment::new(EnvType::EvaluationChannel)?;

    let channel_id = ChannelId::new(env.value.parse::<u64>()?);

    let evaluation_text = format!(
        "**Avaliação de <@{}> para ** \n\n Text Here!",
        interaction.user.id
    );

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new("Zezelo"))
        .description(evaluation_text);

    let message = CreateMessage::new().add_embed(embed);

    channel_id
        .send_message(&ctx.http, message)
        .await
        .expect("Failed to reply message");

    Ok(())
}
