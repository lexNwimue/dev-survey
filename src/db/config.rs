use std::time::Duration;

use anyhow::Ok;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::db::{migration::run_migrations, seeders::seed_db};

pub async fn connect_db(db_url: &str) -> anyhow::Result<DatabaseConnection> {
    let mut db_connection_config = ConnectOptions::new(db_url);
    db_connection_config
        .max_connections(20)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(10));

    let db: DatabaseConnection = Database::connect(db_connection_config).await?;

    run_migrations(&db).await?;
    seed_db(&db).await?;

    Ok(db)
}
