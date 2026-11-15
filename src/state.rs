use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::env_config::EnvConfig;

pub struct AppState {
    pub db: DatabaseConnection,
    pub config: EnvConfig,
}

pub type SharedState = Arc<AppState>;
