use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261025_004250_add_updated_at_trigger"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION set_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN 
                NEW.updated_at = now();
                RETURN NEW;
            END;
            $$ LANGUAGE plpgsql;
            "#,
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER surveys_set_updated_at
            BEFORE UPDATE ON surveys
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at();
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP TRIGGER IF EXISTS surveys_set_updated_at ON surveys;")
            .await?;

        db.execute_unprepared("DROP FUNCTION IF EXISTS set_updated_at();")
            .await?;

        Ok(())
    }
}
