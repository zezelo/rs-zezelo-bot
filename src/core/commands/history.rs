use crate::core::repository::evaluate_repository::EvaluateRepository;
use crate::core::structs::database::DatabaseInstance;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateMessage,
};
use serenity::prelude::Context;

pub async fn get_evaluation_history(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    match interaction.data.options.get(0) {
        Some(data_option) => match &data_option.value {
            CommandDataOptionValue::User(user_id) => {
                let db = DatabaseInstance::new().db;

                let (evaluations, size) =
                    EvaluateRepository::fetch_paginated(db.as_ref(), user_id.get(), 1, 10)
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
                    interaction
                        .channel_id
                        .send_message(
                            &ctx.http,
                            CreateMessage::new().content(format!("Avaliação: {:?}", evaluation)),
                        )
                        .await?;
                }

                interaction
                    .create_followup(
                        &ctx.http,
                        CreateInteractionResponseFollowup::new().content("Conteúdo Processado"),
                    )
                    .await?;

                Ok(())
            }
            _ => {
                interaction
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Esse tipo de comando não existe"),
                        ),
                    )
                    .await?;

                Ok(())
            }
        },
        _ => {
            interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Falha ao obter parâmetro de usuário"),
                    ),
                )
                .await?;

            Ok(())
        }
    }
}
