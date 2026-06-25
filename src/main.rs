use axum::{Router, routing::get};
use dev_survey::env_config::{self, EnvConfig};
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() {

    println!("Starting server...🕙");

    dotenvy::dotenv().expect("env file not found");
   
    let app = Router::new().route("/", get(|| async { "Hello world."}));

    let EnvConfig {database_uri, port}  = env_config::EnvConfig::parse();

    PgPoolOptions::new().max_connections(3).connect(&database_uri).await.expect("Could not start server");

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
