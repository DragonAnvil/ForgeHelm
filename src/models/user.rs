// 
use serde:: {Deserialize, Serialize};
// 
use uuid::Uuid;
// 
use chrono::{DataTime, Utc};


// The User Struct represents a user record from our database
// 
//! The derives here are doing Work:
//! - Serialize/Deserialize: Automatically convert to/from JSON for API resources
//! - Debug: Allows us to print the struct for debugging (but respects privacy)
//! - Clone: Allows us to make copies of User instances when needed
//! - PartialEq: Allows us to compare User instances for equality

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

// CreateUserRequest represents the data we expect to see when creating a new user
//!
//! We define this separate from the User Struct to control what can be created/editable by the frontend
//! User Struct has "id" and timestamps that should not be editable by frontend
//! We call this pattern "Representation Independence"
//! The Internal Struct "User" is separate from the External API Contract "CreateUserRequest"
// derive is a Rust macro that allows implementing traits for the struct
// Debug allows printing the struct using println!()
// Deserialize Enables the struct to be created from sata formats like JSON using serde library
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

// UpdateUserRequest represents the data we expect to see when updating a user
//!
//! We use "Option<Type>" here to allow partial edits to the struct
//! Without it you would need to recreate the entire record
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
}

// UserResponse is what we expect to see when sending Response back to API Frontend
//!
//! This might seem redundant, but defining specific responses allows you to control
//! what specific info is sent back to frontend, without changing the internal structure of the Struct
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Conversion from User to UserResponse
// Used to prepare data to send to Frontend
//! 
//! The "From" trait is Rust's standard method of defining conversions betwee Types
//! This allows calling user_response = UserResponse::from(user)
//! 
// Implement the "From" trait for the Type "UserResponse"
// Where the source Type is "User"
impl From<User> for UserResponse {
    // "from" is required Method for "From" Trait
    fn from(user: User) -> Self {
        // Creates a new UserResponse by pulling fields from "User" Struct
        UserResponse {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// Validation Logic for User Data
// Implementation for CreateUserRequest Struct
impl CreateUserRequest {
    // Validate that the user creation request is valid
    //!
    //! Returns a Result - either Ok(()) if Valid, or Err with
    //! a description. This explicit error handling is much better
    //! than silently accepting invalid data or throwing runtime exceptions
    pub fn validate(&self) -> Result<(), String> {
        let trimmed_name = self.name.trim();

        // 
        if trimmed_name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        // 
        if trimmed_name.len() > 255 {
            return Err("Name cannot be longer than 255 characters".to_string());
        }

        // Room for more validations
        //! Examples:
        //! Check for allowed characters
        //! Check for profanity
        //! Ensure Uniqueness (but usually done at Service Layer) 
        
        Ok(())
    }

    // Get the Cleaned Name (trimmed of whitespace)
    // This is a computed property that ensures we always store clean data
    pub fn cleaned_name(&self) -> String {
        self.name.trim().to_string()
    }
}

// Implementation for UpdateUserRequest Struct
impl UpdateUserRequest {
    // Validate the Update Request 
    pub fn validate(&self) -> Result<(), String> {
        // "if let Some(ref name)" is used because name is Option Type in the UpdateUserRequest Struct
        if let Some(ref name) = self.name {
            // 
            let trimmed_name = name.trim();

            // 
            if trimmed_name.is_empty() {
                return Err("Name cannot be empty".to_string());
            }

            // 
            if trimmed_name.len() > 255 {
                return Err("Name cannot be longer than 255 characters".to_string());
            }
        }
        
        // Room to add more validations

        Ok(())
    }

    // Get the Cleaned Name if one was provided
    // "Option<String>" is used to ensure we only validate if a name is provided in the update request
    pub fn cleaned_name(&self) -> Option<String> {
        self.name.as_ref().map(|name| name.trim().to_string());
    }
}