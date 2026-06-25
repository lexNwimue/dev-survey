use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::env;


#[tokio::main]
async fn main() {

    println!("Starting server...🕙");

    dotenvy::dotenv().expect("env file not found");
   
    let app = Router::new().route("/", get(|| async { "Hello world."}));

    let database_uri = env::var("DATABASE_URI").expect("Could not start server due to missing env");

    PgPoolOptions::new().max_connections(3).connect(&database_uri).await.expect("Could not start server");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
