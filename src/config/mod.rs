// Configuration Management

// Declare which crates to call for use
use std::env;           // Allows reading of environment variables
use anyhow::Results;    // Imports a type alias/identifier from anyhow for error handling

// Application Config Structure
// This Struct ensures all required config is present
// and properly typed before the Application starts

#[derive(Debug, Clone)]         // Auto implement Debug (for formatted printing) and Clone (to duplicate Config values)
pub struct Config {             // Public Struct "Config" 
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub frontend_url: String,
}

// impl = Implementation Block
// Implementation = Methods (Member Functions) for a specified Struct
impl Config {
    // Load Configuration from ENVironment variables
    // The ? operator propogates errors up the call stack automatically
    // "Result<Self>" Result is Return type, Self refers to Type being implemented, in this case Self = Config Struct
    // If Passing, Returns Ok(Config), if Fail, Return Err(anyhow::Error)
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        // 
        Ok(Config {
            // env::var returns Result<String, VarError>
            // We use ? to handle missing variables gracefully
            // Searches .env for each and assigns to Config Struct member if found
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST")
                // unwrap_or_else provides a standard value if missing
                // "|_|" is a closure argument to ignore error for missing value
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?, // parse() converts String to u16, which could fail which is why ? is used
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        })
    }

    // Get the complete Server Address as a String
    // This is computed combining Host and Port
    // Borrows Self to access Host and Port members
    pub fn  server_address(&self) -> String {
        // "format!" is macro for string formatting
        // {}:{} is the format string with 2 placeholders
        format!("{}:{}", self.server_host, self.server_port)
    }
}