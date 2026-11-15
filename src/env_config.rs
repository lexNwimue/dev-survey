use std::env;

pub struct EnvConfig {
    pub database_url: String,
    pub port: u16,
    pub admin_token: String,
}

impl EnvConfig {
    pub fn parse() -> Self {
        let port: u16 = env::var("PORT")
            .unwrap_or(String::from("4000"))
            .parse()
            .expect("Invalid port provided");
        let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env");
        let admin_token = env::var("ADMIN_TOKEN").expect("Missing ADMIN_TOKEN env");

        Self {
            database_url,
            port,
            admin_token,
        }
    }
}
