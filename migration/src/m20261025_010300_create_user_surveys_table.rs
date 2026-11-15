use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261025_010300_create_user_surveys_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // No updated_at on this table, so no trigger
        manager
            .create_table(
                Table::create()
                    .table("user_surveys")
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
                    .col(
                        ColumnDef::new("status")
                            .text()
                            .not_null()
                            .default("in_progress")
                            .check(Expr::col("status").is_in(["in_progress", "complete"])),
                    )
                    .col(
                        ColumnDef::new("current_question_order")
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new("started_at")
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new("completed_at").timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_surveys_user_id")
                            .from("user_surveys", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_surveys_survey_id")
                            .from("user_surveys", "survey_id")
                            .to("surveys", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Upsert target: one participation record per user per survey
        manager
            .create_index(
                Index::create()
                    .name("uq_user_surveys_user_id_survey_id")
                    .table("user_surveys")
                    .col("user_id")
                    .col("survey_id")
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_surveys_user_id")
                    .table("user_surveys")
                    .col("user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_surveys_survey_id")
                    .table("user_surveys")
                    .col("survey_id")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("user_surveys").if_exists().to_owned())
            .await
    }
}
