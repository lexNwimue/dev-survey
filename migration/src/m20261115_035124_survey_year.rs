use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261115_035124_survey_year"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_surveys_year")
                    .table("surveys")
                    .col("year")
                    .unique()
                    .to_owned(),
            )
            .await?;

        let connection = manager.get_connection();
        connection
            .execute_unprepared(
                r#"ALTER TABLE surveys
            ALTER COLUMN year DROP DEFAULT;"#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_surveys_year")
                    .table("surveys")
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
