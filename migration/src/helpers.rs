use sea_orm_migration::prelude::ConnectionTrait;
use sea_orm_migration::{DbErr, SchemaManager};

/// Attaches the shared `set_updated_at()` trigger function to `table`.
/// Table names are hardcoded by migrations, never user input.
pub async fn add_updated_at_trigger(manager: &SchemaManager<'_>, table: &str) -> Result<(), DbErr> {
    let sql = format!(
        "CREATE TRIGGER {table}_set_updated_at BEFORE UPDATE ON {table} \
        FOR EACH ROW EXECUTE FUNCTION set_updated_at();"
    );
    manager.get_connection().execute_unprepared(&sql).await?;
    Ok(())
}
