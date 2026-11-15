use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

pub async fn run_migrations(db: &DatabaseConnection) -> anyhow::Result<()> {
    Migrator::up(db, None).await?;
    Ok(())
}
