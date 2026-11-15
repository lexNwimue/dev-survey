use sea_orm_migration::prelude::*;

use crate::helpers::add_updated_at_trigger;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261025_010100_create_sections_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("sections")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new("survey_id").uuid().not_null())
                    .col(ColumnDef::new("title").string_len(200).not_null())
                    .col(ColumnDef::new("description").text())
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
                            .name("fk_sections_survey_id")
                            .from("sections", "survey_id")
                            .to("surveys", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // The query builder can't express DEFERRABLE, so this constraint is raw SQL
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE sections ADD CONSTRAINT uq_sections_survey_id_order_index \
                UNIQUE (survey_id, order_index) DEFERRABLE INITIALLY DEFERRED;",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sections_survey_id")
                    .table("sections")
                    .col("survey_id")
                    .to_owned(),
            )
            .await?;

        add_updated_at_trigger(manager, "sections").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("sections").if_exists().to_owned())
            .await
    }
}
