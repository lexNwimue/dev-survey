use sea_orm_migration::prelude::*;

use crate::helpers::add_updated_at_trigger;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261025_010000_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new("email")
                            .string_len(254)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new("first_name").string_len(100).not_null())
                    .col(ColumnDef::new("last_name").string_len(100).not_null())
                    // Profile fields (keyed enums — short code stored)
                    .col(ColumnDef::new("job_role").string_len(50))
                    .col(ColumnDef::new("state").string_len(50))
                    .col(ColumnDef::new("primary_language").string_len(50))
                    // Profile fields (direct enums — snake_case value stored)
                    .col(ColumnDef::new("age_range").string_len(50))
                    .col(ColumnDef::new("experience_range").string_len(50))
                    .col(ColumnDef::new("gender").string_len(50))
                    .col(ColumnDef::new("education_level").string_len(50))
                    .col(ColumnDef::new("employment_type").string_len(50))
                    .col(ColumnDef::new("work_arrangement").string_len(50))
                    .col(ColumnDef::new("nysc_status").string_len(50))
                    .col(ColumnDef::new("earning_currency").string_len(50))
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

        add_updated_at_trigger(manager, "users").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Dropping the table also drops its indexes and trigger
        manager
            .drop_table(Table::drop().table("users").if_exists().to_owned())
            .await
    }
}
