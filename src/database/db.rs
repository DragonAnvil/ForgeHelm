// 
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

// Calls local Crate "config", imports Config Public Struct
use crate::config::Config;

// Public Alias Type "PgPool" for deadpool_postgres{Pool}
// This makes calling Pool easier throughout App
pub type PgPool = Pool;

// Public Function to Create a Pool using deadpool_postgres
// Pulling config info from config.rs
pub fn create_pool(cfg: &Config) -> PgPool {
    // Configuring ManagerConfig Struct
    // RecyclingMethod to be set to Fast
    let mgr_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };

    // Configure Manager Struct
    let mgr = Manager::from_config(
        // cfg is Type Config, pull database_url from Struct
        cfg.database_url.parse().unwrap(), 
        // This sets up a new empty config object from tokio
        tokio_postgres::Config::new(),
        // Specifies the connection should use no TLS encryption
        NoTls,
        // This is the pool behavior configuration from above
        mgr_config
    );
    
    // Start building a pool using "mgr" struct
    Pool::builder(mgr)
    // Max number of conncetions in the pool
    .max_size(16)
    // Finalize builder and create actual pool
    .build()
    // Extracts the Pool from Result immediately, will crash if invalid config
    .unwrap()
}