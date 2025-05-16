use crate::core::entities::evaluation::CreateUserEvaluation;
use crate::infrastructure::database::entities::player_evaluation;
use crate::infrastructure::database::entities::player_evaluation::Model;
use chrono::Utc;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DbErr};
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{PaginatorTrait, QueryFilter};
pub struct EvaluateRepository {}

impl EvaluateRepository {
    pub async fn create<C: ConnectionTrait>(
        db: &C,
        create_user_evaluation: CreateUserEvaluation,
    ) -> Result<Model, DbErr> {
        let player_evaluation = player_evaluation::ActiveModel {
            player_id: ActiveValue::Set(create_user_evaluation.player_id),
            evaluator_id: ActiveValue::Set(create_user_evaluation.evaluator_id),
            comment: ActiveValue::Set(create_user_evaluation.comment),
            behavior: ActiveValue::Set(create_user_evaluation.behavior),
            teamplay: ActiveValue::Set(create_user_evaluation.teamplay),
            communication: ActiveValue::Set(create_user_evaluation.communication),
            utility_usage: ActiveValue::Set(create_user_evaluation.utility_usage),
            created_at: ActiveValue::Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(player_evaluation)
    }

    pub async fn fetch_paginated<C: ConnectionTrait>(
        db: &C,
        player_id: u64,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = player_evaluation::Entity::find()
            .filter(player_evaluation::Column::PlayerId.eq(player_id))
            .paginate(db, page_size);

        let total_pages = paginator.num_pages().await?;
        let records = paginator.fetch_page(page - 1).await?;

        Ok((records, total_pages))
    }
}
