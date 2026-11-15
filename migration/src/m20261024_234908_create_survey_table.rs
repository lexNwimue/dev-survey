use sea_orm_migration::{
    prelude::*,
    sea_query::value::prelude::{Utc, chrono::Datelike},
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261024_234908_create_survey_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("surveys")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new("title").string_len(200).not_null())
                    .col(
                        ColumnDef::new("slug")
                            .string_len(100)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new("status")
                            .text()
                            .not_null()
                            .default("draft")
                            .check(Expr::col("status").is_in(["draft", "active", "closed"])),
                    )
                    .col(
                        ColumnDef::new("results_published")
                            .boolean()
                            .not_null()
                            .default(Expr::Value(Value::Bool(Some(false)))),
                    )
                    .col(
                        ColumnDef::new("year")
                            .integer()
                            .not_null()
                            .default(Utc::now().year()),
                    )
                    .col(ColumnDef::new("opens_at").timestamp_with_time_zone())
                    .col(ColumnDef::new("closes_at").timestamp_with_time_zone())
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
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_surveys_slug")
                    .table("surveys")
                    .col("slug")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_surveys_status")
                    .table("surveys")
                    .col("status")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_surveys_slug")
                    .table("surveys")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_surveys_status")
                    .table("surveys")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("surveys").to_owned())
            .await?;

        Ok(())
    }
}
