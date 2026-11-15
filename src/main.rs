use std::sync::Arc;

use dev_survey::{db::config::connect_db, env_config, handlers, state::AppState};

#[tokio::main]
async fn main() {
    println!("Starting server...🕙");
    dotenvy::dotenv().expect("env file not found");

    let config = env_config::EnvConfig::parse();

    let db = connect_db(&config.database_url)
        .await
        .expect("unable to connect to db");

    let state = Arc::new(AppState { config, db });

    let app = handlers::all_routes();

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
