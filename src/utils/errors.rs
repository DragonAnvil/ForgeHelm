// 
use actix_web::{HttpResponse, ResponseError};
// 
use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    // Database Connection Errors
    // Pool Ehxausted, Network Issues
    #[error("Database Connection Error: {0}")]
    DatabaseConnection(#[from] deadpool_postgres::PoolError),

    // Database Query Errors
    // SQL Syntax, Constraint Violations
    #[error(Database Error: {0})]
    Database(#[from] tokio_postgres::Error),

    // Validation Errors for User Input
    #[error("Validation Error: {0}")]
    ValidationError(String),

    // Requested Resource Not Found
    #[error("Resource Not Found: {0}")]
    NotFound(String),

    // Internal Server Errors
    // Unexpected Conditions
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    // JSON Parsing Errors
    #[error("JSON Parsing Error: {0}")]
    JSONError(#[from] serde_json::Error),
}

// Implementation of ResponseError allows AppError to be auto converted into HTTP Response by Actix Web
// This Integrates Business Logic and HTTP Layer
// When Result<T, AppError> is returned, Actix Web auto convers AppError into HTTP Response
// This eliminates boilerplater error handling in request handlers
impl ResponseError for AppError {
    // Determine which status code to return for each error type
    // This method maps internal error types to HTTP status codes
    fn status_code(&self) -> actix_web::http::StatusCode {
        // 
        use actix_web::http::StatusCode;

        // Match situation for each error type
        match self {
            // Validation errors are the client's fault
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,

            // Not Found Errors, Return 404
            AppError::NotFound(_) => StatusCode::NOT_FOUND,

            // Database and Connection Errors are Server Problems
            AppError::Database(_)
            | AppError::DatabaseConnection(_)
            | AppError::InternalServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            // JSON parsing errors are typically client errors (malformed JSON)
            AppError::JsonError(_) => StatusCode::BAD_REQUEST,
        }
    }

    // Create the HTTP Response Body for Each Error Type
    //
    // This method creates the actual JSON response which will be sent to client
    // This method does not expose internal details like database connection strings or internal file paths for security
    fn error_response(&self) -> HttpResponse {
        // Create consistent error response format
        // This will enable frontend to handle errors easily with a standard format
        let error_response = serde_json::json!({
            "error": {
                "message": self.user_friendly_message(),
                "type":self.error_type(),
            }
        });

        // 
        HttpResponse::build(self.status_code()).jso(error_response)
    }
}

// 
impl AppError {
    // Get User-Friendly Error Message
    // 
    // 
    fn user_friendly_message(&self) -> String {
        // 
        match self {
            // 
            AppError::ValidationError(msg) => msg.clone(),
            // 
            AppError::NotFound(msg) => msg.clone(),
            // 
            AppError::JsonError(_) => "Invalid JSON Format".to_string(),

            // 
            AppError::Database(_) => "A Database Error Occured".to_string(),
            // 
            AppError::DatabaseConnection(_) => "Unable to Connect to Database".to_string(),
            // 
            AppError::InternalServerError(msg) => {
                // Log Actual Error for Internal Debugging, but Return Generic Message
                log::error!("Internal Server Error: {}", msg);
                "An Internal Server Error Occured".to_string()
            }   
        }
    }

    // Get Machine-Readable Error Type
    // 
    // Allows Frontend to handle different error types
    // programmatically rather than parsing error messages
    fn error_type(&self) -> &'static str {
        // 
        match self {
            // 
            AppError::ValidationError(_) => "validation_error",
            AppError::NotFound(_) => "not_found",
            AppError::Database(_) => "database_error",
            AppError::DatabaseConnection(_) => "database_connection_error",
            AppError::InternalServerError(_) => "internal_server_error",
            AppError::JsonError(_) => "json_error",
        }
    }
}

// Helper Function to Create NotFound Errors
// 
// Makes creating not found errors easier with a standard format throughout App
impl AppError {
    // 
    pub fn not_found(resource: &str, id: &str) -> Self {
        // 
        AppError::NotFound(format!("{} with id '{}' not found", resource, id))
    }

    // 
    pub fn user_not_found(user_id: uuid::Uuid) -> Self {
        // 
        AppError::not_found("User", &user_id.to_string())
    }
}