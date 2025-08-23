// Imports neccessary types for PostgreSQL connection pooling
use deadpool_postgres::{Config as PoolConfig, Pool, Runtime};
// Imports the type for non-TLS database connections
use tokio_postgres::NoTls;
// Imports a generic error-handling result Type
use anyhow::Result;

// Database Connection Pool Type Alias
// This improves readability
pub type DbPool = Pool;

// Create a connection pool to the PostgreSQL Database
// Connection Pooling improves Performance for the following reasons:
// 1. Creating database connections is expensive (network handshake, authentication)
// 2. We want to reuse connections across multiple requests
// 3. We need to limit the total number of connections to avoid overwhelming the database

// Parameter        = database_url
// Type             = &str (borrowed string slice)
// Result<DbPool>   = On Success, Return DbPool Type
pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    // Parse the Database URL into the connection configuration
    // This function can fail if the URL is malformed
    // PoolConfig = imported Type from deadpool_postgres = Config
    // "from_connection_string" parses database_url into a configuration object
    let mut cfg = PoolConfig::from_connection_string(database_url)?;

    // Configure the connection pool parameters
    // These numbers can be tuned in the future based on usage of API
    // manager is a field of Config struct from deadpool_postgreSQL Crate used to access connection pool manager settings
    // This essentially sets the config to recycle connections quickly for reuse without extra checks or resets
    cfg.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    // Maximum number of connections in the pool
    cfg.max_size = 16;
    // Keep at least 4 idle connections ready
    cfg.min_idle = Some(4);

    // Create the Connection Pool
    // Use NoTls for simplicity, return and improve this in production
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    // Test the connection by trying to retrieve one from the pool
    {
        let _client = pool.get().await?;
        log::info!("Successfully connected to PostgreSQL Database!")
    }

    // 
    Ok(pool)
}

// Run Database Migrations
// Simplified Migration Runner
// pool     = Parameter name
// &DbPool  = Type
// Migration is the same thing as Updating
// Migration is used on initial setup of backend server and anytime the Database Schema Structure has been changed
pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    // Gets Database Client from pool
    let client = pool.get().await?;

    // Read and Execute the migration file
    // In Production, you should track which migrations have been run
    // "include_str" is macro that includes contents of the file as a string at compile time
    let migration_sql = include_str("../../migrations/initial.sql");

    // "batch_execute" Executes SQL statements from var above
    client.batch_execute(migration_sql).await?;

    // 
    log::info!("Database migrations completed successfully!");
    Ok(());
}