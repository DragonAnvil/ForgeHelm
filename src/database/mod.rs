// Database Setup & Connection

//! Database module handles all database-related functionality
//! 
//! This module demonstrates Rust's module system
//! Each mod.rs acts as the public interface for the module
//! This is used to decide what gets exposed to the rest of the App

// Creates a Public subModule allowing access inside other modules
// Includes code from connection.rs
pub mod connection;

// Re-Export commonly used types so other modules can access them easily
// This creates a clean API
// Parameters are listed from connection.rs, this makes them accessible outside this module without needing "connection::"
pub use connection::{DbPool, create_pool, run_migrations};