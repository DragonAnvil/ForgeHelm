// 
use dotenvy::dotenv;
use std::env;

// Structure (Public) for Configuration Data
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

// Implementation for Config Struct
impl Config {
    pub fn from_env() -> Self {
        // Read and Access data from .env file
        dotenv().ok();
        // Create var "database_url" from .env variable named "DATABASE_URL", with expectation set
        let databse_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // Create host variable from .env var named "HOST"
        // env::Var("HOST") retrieves and returns Type "Result<T, Error>"
        // If there is an Error, then .unwrap_or_else is used to ignore the error
        // |_| is a closure statement in Rust, |...| is used to define parameters of a closure
        // _ is used as a placeholder used to ignore the input
        // "|_| "127.0.0.1" is used if an error is returned, ignore it and substitute "127.0.0.1"
        // .into() is used to convert "127.0.0.1" into a String
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        // Create port variable from .env "PORT" variable with error handling
        let port = env::var("PORT").unwrap_or_else(|_| "8080".into()).parse().expect("PORT must be a number");
        
        // This Implementation Returns Self, with the following vars
        Self { database_url, host, port }

    }
}