use std::env;

pub struct EnvConfig {
    pub database_uri: String,
    pub port: u16
}


impl EnvConfig {
    pub fn parse () -> Self {
        let port: u16 = env::var("PORT").unwrap_or(String::from("4000")).parse().expect("Invalid port provided");
        let database_uri = env::var("DATABASE_URI").expect("Missing DATABASE_URI env");

        Self {
            database_uri,
            port
        }
    }
}