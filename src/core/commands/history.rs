use crate::core::repository::evaluate_repository::EvaluateRepository;
use crate::core::structs::database::DatabaseInstance;
use crate::infrastructure::database::entities::player_evaluation::Model;
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandInteraction, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
    CreateMessage, UserId,
};
use serenity::prelude::Context;

pub async fn send_evaluation_by_model(
    ctx: &Context,
    interaction: &CommandInteraction,
    model: &Model,
) -> Result<(), serenity::Error> {
    let mut evaluation_text = format!(
        "**Avaliação de <@{}> para <@{}>**\n",
        model.evaluator_id, model.player_id
    );

    evaluation_text.push_str(
        format!(
            "\n**{}:**\n{}",
            "Comunicação do jogador", model.communication
        )
        .as_str(),
    );
    evaluation_text.push_str(
        format!(
            "\n**{}:**\n{}",
            "Trabalho em equipe do jogador", model.teamplay
        )
        .as_str(),
    );
    evaluation_text
        .push_str(format!("\n**{}:**\n{}", "Comportamento do jogador", model.behavior).as_str());

    evaluation_text
        .push_str(format!("\n**{}:**\n{}", "Uso de Utilitárias", model.utility_usage).as_str());

    if let Some(comment) = &model.comment {
        evaluation_text.push_str(format!("\n**{}:**\n{}", "Comentário", comment).as_str());
    }

    interaction
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embeds(vec![CreateEmbed::new().description(evaluation_text)]),
        )
        .await?;

    Ok(())
}
fn extract_options(options: &[CommandDataOption]) -> Result<(UserId, i64), String> {
    let user_option = options.get(0).ok_or("Missing user option")?;
    let int_option = options.get(1).ok_or("Missing integer option")?;

    let user_id = match user_option.value {
        CommandDataOptionValue::User(id) => id,
        _ => return Err("First option is not a user".to_string()),
    };

    let integer = match int_option.value {
        CommandDataOptionValue::Integer(value) => value,
        _ => return Err("Second option is not an integer".to_string()),
    };

    Ok((user_id, integer))
}
pub async fn get_evaluation_history(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let (user_id, page) = extract_options(interaction.data.options.as_ref())?;

    let db = DatabaseInstance::new().db;

    let (evaluations, size) =
        EvaluateRepository::fetch_paginated(db.as_ref(), user_id.get(), page as u64, 20)
            .await
            .expect("Failed to fetch evaluations");

    if (size == 0) {
        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Não foi encontrado nenhum registro"),
                ),
            )
            .await
            .expect("Failed to respond to interaction");

        return Ok(());
    }

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new()
                    .content("Essas são as respostas!")
                    .ephemeral(true),
            ),
        )
        .await?;

    for evaluation in evaluations {
        send_evaluation_by_model(&ctx, &interaction, &evaluation).await?;
    }

    interaction
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new().content(format!(
                "Exibindo página 1 de **{}** do jogador <@{}>",
                size,
                user_id.get()
            )),
        )
        .await?;

    Ok(())
}
