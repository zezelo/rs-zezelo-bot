use crate::ColumnRef::Column;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Discord::Table)
                    .if_not_exists()
                    .col(string(Discord::Id).primary_key().unique_key().not_null())
                    .col(string(Discord::Name))
                    .col(ColumnDef::new(Discord::GlobalName).string())
                    .col(ColumnDef::new(Discord::Email).string())
                    .col(ColumnDef::new(Discord::Discriminator).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .string()
                            .primary_key()
                            .unique_key()
                            .not_null(),
                    )
                    .col(string(User::Name))
                    .foreign_key(
                        ForeignKey::create()
                            .from(User::Table, User::Id)
                            .to(Discord::Table, Discord::Id),
                    )
                    .col(boolean(User::Administrator))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .string()
                            .primary_key()
                            .unique_key()
                            .not_null(),
                    )
                    .col(string(User::Name))
                    .foreign_key(
                        ForeignKey::create()
                            .from(User::Table, User::Id)
                            .to(Discord::Table, Discord::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PlayerEvaluation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlayerEvaluation::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlayerEvaluation::EvaluatorId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlayerEvaluation::PlayerId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlayerEvaluation::Communication)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlayerEvaluation::Teamplay)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlayerEvaluation::UtilityUsage)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlayerEvaluation::Behavior)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlayerEvaluation::Comment).text().null())
                    .col(
                        ColumnDef::new(PlayerEvaluation::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_evaluator_id")
                            .from(PlayerEvaluation::Table, PlayerEvaluation::EvaluatorId)
                            .to(Discord::Table, Discord::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_id")
                            .from(PlayerEvaluation::Table, PlayerEvaluation::PlayerId)
                            .to(Discord::Table, Discord::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Discord::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PlayerEvaluation::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Administrator,
}
#[derive(DeriveIden)]
enum Discord {
    Table,
    Id,
    Name,
    GlobalName,
    Email,
    Discriminator,
}

#[derive(DeriveIden)]
enum PlayerEvaluation {
    Table,
    Id,
    EvaluatorId,
    PlayerId,
    Communication,
    Teamplay,
    UtilityUsage,
    Behavior,
    Comment,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    Name,
    Description,
}
#[derive(DeriveIden)]
enum GuildUser {
    Table,
    GuildId,
    UserId,
    CreatedAt,
}
