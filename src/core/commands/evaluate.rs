use crate::core::entities::evaluation::CreateUserEvaluation;
use crate::core::enums::discord::DiscordCustomId;
use crate::core::repository::evaluate_repository::EvaluateRepository;
use crate::core::structs::database::DatabaseInstance;
use crate::core::structs::environment::{EnvType, Environment};
use serenity::all::*;
use std::default::Default;
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
    match interaction.data.options.get(0) {
        Some(data_option) => {
            if let CommandDataOptionValue::User(user_id) = &data_option.value {
                interaction
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Modal(
                            CreateModal::new(
                                format!(
                                    "{}|{}",
                                    DiscordCustomId::Evaluate.as_str(),
                                    user_id.to_string()
                                ),
                                "Avaliação de jogador",
                            )
                            .components(create_action_rows()),
                        ),
                    )
                    .await?;
            }
        }
        _ => {
            println!("Param option not found");
        }
    }

    Ok(())
}

pub async fn publish_at_evaluation_channel(
    ctx: &Context,
    interaction: &ModalInteraction,
    custom_id: &str,
) -> Result<(), Box<dyn Error>> {
    let env = Environment::new(EnvType::EvaluationChannel)?;

    let channel_id = ChannelId::new(env.value.parse::<u64>()?);

    let mut evaluation_text = format!(
        "**Avaliação de <@{}> para <@{}>**\n",
        interaction.user.id, custom_id
    );

    let instance = DatabaseInstance::new().db;

    let mut user_evaluation = CreateUserEvaluation::default();

    user_evaluation.evaluator_id = interaction.user.id.to_string();
    user_evaluation.player_id = custom_id.to_string();

    for action_row in interaction.data.components.iter() {
        for component in &action_row.components {
            if let ActionRowComponent::InputText(input_text) = component {
                if let Some(value) = &input_text.value {
                    if let Some(custom_id) = DiscordCustomId::new(input_text.custom_id.as_str()) {
                        let question_title = match custom_id {
                            DiscordCustomId::EvaluateCommunication => {
                                user_evaluation.communication = value.to_string();
                                "Comunicação do jogador"
                            }
                            DiscordCustomId::EvaluateTeamWork => {
                                user_evaluation.teamplay = value.to_string();
                                "Trabalho em equipe do jogador"
                            }
                            DiscordCustomId::EvaluateBehaviour => {
                                user_evaluation.behavior = value.to_string();
                                "Comportamento do jogador"
                            }
                            DiscordCustomId::EvaluateGrenade => {
                                user_evaluation.utility_usage = value.to_string();
                                "Uso de Utilitárias"
                            }
                            _ => {
                                user_evaluation.comment = Some(value.to_string());
                                "Comentário"
                            }
                        };

                        evaluation_text
                            .push_str(format!("\n**{}:**\n{}", question_title, value).as_str())
                    }
                }
            }
        }
    }

    EvaluateRepository::create(instance.as_ref(), user_evaluation)
        .await
        .expect("Failed to create evaluation");

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new("Zezelo"))
        .description(evaluation_text);

    let message = CreateMessage::new().add_embed(embed);

    channel_id
        .send_message(&ctx.http, message)
        .await
        .expect("Failed to reply message");

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Sua avaliação foi realizada, obrigado!")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
