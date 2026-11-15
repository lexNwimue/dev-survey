use sea_orm_migration::prelude::*;

use crate::helpers::add_updated_at_trigger;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261025_010200_create_questions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("questions")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new("survey_id").uuid().not_null())
                    .col(ColumnDef::new("section_id").uuid().not_null())
                    .col(ColumnDef::new("text").text().not_null())
                    .col(
                        ColumnDef::new("type")
                            .text()
                            .not_null()
                            .check(Expr::col("type").is_in([
                                "single_choice",
                                "multi_choice",
                                "text",
                                "scale",
                            ])),
                    )
                    .col(ColumnDef::new("options").json_binary())
                    .col(
                        ColumnDef::new("required")
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new("order_index").integer().not_null())
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
                            .name("fk_questions_survey_id")
                            .from("questions", "survey_id")
                            .to("surveys", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_questions_section_id")
                            .from("questions", "section_id")
                            .to("sections", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // order_index is unique across the whole survey, not per section
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE questions ADD CONSTRAINT uq_questions_survey_id_order_index \
                UNIQUE (survey_id, order_index) DEFERRABLE INITIALLY DEFERRED;",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_questions_survey_id")
                    .table("questions")
                    .col("survey_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_questions_section_id")
                    .table("questions")
                    .col("section_id")
                    .to_owned(),
            )
            .await?;

        add_updated_at_trigger(manager, "questions").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("questions").if_exists().to_owned())
            .await
    }
}
