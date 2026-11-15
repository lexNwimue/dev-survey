use sea_orm_migration::prelude::*;

use crate::helpers::add_updated_at_trigger;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261025_010400_create_responses_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("responses")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new("user_id").uuid().not_null())
                    .col(ColumnDef::new("survey_id").uuid().not_null())
                    .col(ColumnDef::new("question_id").uuid().not_null())
                    .col(ColumnDef::new("answer").json_binary().not_null())
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new("updated_at")
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_responses_user_id")
                            .from("responses", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_responses_survey_id")
                            .from("responses", "survey_id")
                            .to("surveys", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_responses_question_id")
                            .from("responses", "question_id")
                            .to("questions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Upsert target: one answer per user per question
        manager
            .create_index(
                Index::create()
                    .name("uq_responses_user_id_question_id")
                    .table("responses")
                    .col("user_id")
                    .col("question_id")
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_responses_survey_id")
                    .table("responses")
                    .col("survey_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_responses_question_id")
                    .table("responses")
                    .col("question_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_responses_user_id")
                    .table("responses")
                    .col("user_id")
                    .to_owned(),
            )
            .await?;

        add_updated_at_trigger(manager, "responses").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("responses").if_exists().to_owned())
            .await
    }
}
