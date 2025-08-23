// Data Structure & Database Models

// Contains all data structures and domain objects
//!
//! This module acts as Central Registry for all Data Models
//! Room to add more model files for different table schemas

// Makes a Public subModule accessible to external modules
pub mod user;

// Export commonly used Types from User to external modules to access directly 
pub use user::{
    User, 
    CreateUserRequest,
    UpdateUserRequest,
    UserResponse
};