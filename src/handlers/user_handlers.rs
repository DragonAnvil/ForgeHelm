// Import Actix Web Types
// web          : used for extracting data from requests (Path, Query, JSON)
// HttpResponse : used to build HTTP Responses
// Result       : used for Handler Retur Types, allows Error Handling
use actix_web::{web, HttpResponse, Result};
// Import Uuid Type from the uuid crate
// Need this for handling user Ids
use uuid::Uuid;

// Import the database connection pool type
// DbPool is unique to this project and is the database connection manager
// Enables Handlers to access the database
use crate::database::DbPool;
// Import User-Related Data Structures:
use crate::models::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
// Import the User Service:
// UserService : Contains Business Logic for User Operations (Create, Read, Update, Delete)
use crate::services::user_service::UserService;
// Import Project Specific Custom Error Type:
// AppError: used for consistent error handling and response formatting across handlers
use crate::utils::errors::AppError;


//! Create a New User
//! 
//! Purpose: 
//!     Recieve JSON payload from Client
//!     Validate and Process Data
//!     Create New User in Database
//!     Return Response with Created Users Data
//! This handler demonstrates the usual flow of an HTTP Request in Actix Web
//! 1. Extract & Validate the Request Data
//! 2. Pass Data to the Service Layer for Business Logic
//! 3. Convert Result into an appropriate HTTP Response
//! 
//! 
pub async fn create_user(
    // "web::Data" is a wrapper for thread-safe shared state
    // Actix Web injects a shared database connection pool into this handler
    pool: web::Data<DbPool>,            // Dependency Injection of Database Pool
    // Actix Web auto deserializes incoming JSON body into a CreateUserRequest Struct
    // This ensures Type Safety and Validation immediately
    json: web::Json<CreateUserRequest>, // Auto JSON Deserialization

    // On Success, Return an HTTP Response
    // On Failure, Return a custom AppError
) -> Result<HttpResponse, AppError> {
    
    // Create the Service Instance
    // This creates a new instance of the User Service which encapsulates all busness logic
    // Passes a clone of the database pool reference to the service which allows interaction with database
    // "pool" is Type of web::Data<DbPool> which is Actix Web for shared state
    // ".get_ref()" Returns a reference to the DbPool
    // ".clone()" Clones the database pool ref, think of this as a handle or pointer to the actual pool
    // "UserService::new(...)" Calls the constructor for UserService, passing in the cloned pool
    let user_service = UserService::new(pool.get_ref().clone());

    // Call the Business Logic "create_user" from "user_service" to create a new user
    // json is pulled from above parameter, already deserialized
    // ".into_inner()" extracts the actual CreateUserRequest struct from the wrapper "json" giving direct access to the suer creation data
    // "json.into_inner()" = CreateUserRequest Struct data
    // "create_user" contains all the business logic for creating a user
    let user = user_service.create_user(json.into_inner()).await?;

    // Convert the User into a UserResponse and Return as JSON
    // "?" Operator here handles any JSON Serialization Errors
    let response = UserResponse::from(user);

    // Return "201 Created" for successful resource creation
    // REST Conventin and feeds clients useful info
    Ok(HttpResponse::Created().json(response))
}

// Get a User by ID
// 
// This handlers shows how to extract Path Parameters and handle errors
// 
pub async fn get_user(
    // This injects a wrapped connection to a database pool
    // "web::Data" This allows Actix Web to manage the data
    // This enables thread-safety = allows other multiple threads to use at same time
    // Shared State allows coherence between multiple threads
    pool: web::Data<DbPool>,
    // Tells Actix Web to extract the Rust Type "Uuid" from the URL
    path: web::Path<Uuid>,      // Extract UUID from URL Path

    // On Success, Return an HTTP Response
    // On Failure, Return a custom AppError
) -> Result<HttpResponse, AppError> {
    // Extracts the Uuid value from the Actix Web Path extractor and assigns it to "user_id"
    let user_id = path.into_inner();
    
    // Create the Service Instance
    // This creates a new instance of the User Service which encapsulates all busness logic
    // Passes a clone of the database pool reference to the service which allows interaction with database
    // "pool" is Type of web::Data<DbPool> which is Actix Web for shared state
    // ".get_ref()" Returns a reference to the DbPool
    // ".clone()" Clones the database pool ref, think of this as a handle or pointer to the actual pool
    // "UserService::new(...)" Calls the constructor for UserService, passing in the cloned pool
    let user_service = UserService::new(pool.get_ref().clone());

    // Call the business logic "get_user_by_id" asynchronously on the instance "user_service"
    // using "user_id" parameter from above
    // ".await?" propogates any errors
    match user_service.get_user_by_id(user_id).await? {
        // "Option:Some" = If a user is found, convert the internal user model to a UserResponse DTO
        // DTO = Data Transfer Object
        // Return a "200 OK HTTP Response" with User Data as JSON
        Some(user) => {
            let response = UserResponse::from(user);
            Ok(HttpResponse::Ok().json(response))
        }
        // "Option:None" = If no user is found, create and return a 404 Status Code Error using custom AppError Type
        None => {
            // Return "404 Error" using custom error type
            Err(AppError::user_not_found(user_id))
        }
    }
}


// Query Parameters for Endpoint: get_users
// 
// This Struct demonstrates how to handle Optional query parameters in a Type-Safe way
// Ths Struct defines the possible Query PArameters for the "get_users" endpoint
// Each field corresponds to a query parameter in the URL
// All feilds are optional, allowing felixible requests
// Actix Web auto deserializes query strings into this struct, ensuring Type-Safety
// This is much safer and cleaner than manually parsing query parameters
#[derive(serde::Deserialize)]
pub struct GetUsersQuery {
    // Maximum number of Users to Return (Pagination)
    pub limit: Option<i64>,
    // Number of Users to Skip (Pagination)
    pub offset: Option<i64>,
    // Optional Search Term to filter Users by name or other critria
    pub search: Option<String>,
}

// Get All Users with Optional Pagination & Search
// 
// This Handler shows how to handle complex Query Parameters
// Demonstrates how different business logic paths can share the same HTTP Endpoint
pub async fn get_users(
    // This injects a wrapped connection to a database pool
    // "web::Data" This allows Actix Web to manage the data
    // This enables thread-safety = allows other multiple threads to use at same time
    // Shared State allows coherence between multiple threads
    pool: web::Data<DbPool>,
    // This tells Actix Web to extract query parameters from the URL and auto deserialize into Rust Struct "GetUserQuery"
    // "web::Query<T>" = Actix Web Extractor for Query Parameters
    //      Takes the Query String from URL and tries to convert it into the type specified using Serde for Deserialization
    // "GetUsersQuery" is the Rust Struct defined just above
    query: web::Query<GetUsersQuery>,

    // If Success, Return an HTTP Response
    // If FAilure, Return a custom AppError
) -> Result<HttpResponse, AppError> {

    // Create the Service Instance
    // This creates a new instance of the User Service which encapsulates all busness logic
    // Passes a clone of the database pool reference to the service which allows interaction with database
    // "pool" is Type of web::Data<DbPool> which is Actix Web for shared state
    // ".get_ref()" Returns a reference to the DbPool
    // ".clone()" Clones the database pool ref, think of this as a handle or pointer to the actual pool
    // "UserService::new(...)" Calls the constructor for UserService, passing in the cloned pool 
    let user_service = UserService::new(pool.get_ref().clone());

    // If the Client provides a search term, it performs a search
    // Otherwise, return a paginated list of users
    // "if let Some(ref search_term"
    //      Checks if the "search" field in "GetUsersQuery" Struct is Some (Client provided search term in query string)
    //      If so, bind "search_terms" to a ref to the string
    let users = if let Some(ref search_term) = query.search {
        // If a search term is provided
        //      Call "search_users_by_name" through instance "user_service" using "search_term"
        user_service.search_users_by_name(search_term).await?
    } else {
        // Otherwise,
        //      Call "get_users" through instance "user_service"
        //      using "limit" and "offset" from "query" parameters to control pagination
        user_service.get_users(query.limit, query.offset).await?
    };

    // Converts a list of internal user models "User" into a list of response DTOs "UserResponse"
    // Safe and Formatted for API
    let response : Vec<UserResponse> = users
        // Takes ownership of the users vector and creates an iterator over its elements
        .into_iter()
        // For each User in the iterator, call "UserResponse::from(user)" to convert it into a UserResponse
        .map(UserResponse::from)
        // Collects the mapped results into a new Vec<UserResponse>
        // This Result is a vector of DTOs ready for serialization
        .collect();

    // Serializes the response vector to JSON and sends it in the HTTP response body
    // Returns HTTP status code 200 OK
    Ok(HttpResponse::Ok().json(responses))
}

// Update an Existing User
// 
// This Handler combines Path Parameters (for User ID) with a JSON body (for Update Data)
// Demonstrates how to handle partial updates and return appropriate status codes
pub async fn update_user(
    // This injects a wrapped connection to a database pool
    // "web::Data" This allows Actix Web to manage the data
    // This enables thread-safety = allows other multiple threads to use at same time
    // Shared State allows coherence between multiple threads
    pool: web::Data<DbPool>,
    // Tells Actix Web to extract the Rust Type "Uuid" from the URL
    path: web::Path<Uuid>,
    // Actix Web auto deserializes incoming JSON body into a UpdateUserRequest Struct
    // This ensures Type Safety and Validation immediately
    json: web::Json<UpdateUserRequest>,

    // If Success, Return an HTTP Response
    // If Failure, Return a custom AppError
) -> Result<HttpResponse, AppError> {
    // Extracts the Uuid value from the Actix Web Path extractor and assigns it to "user_id"
    let user_id = path.into_inner();

    // Create the Service Instance
    // This creates a new instance of the User Service which encapsulates all busness logic
    // Passes a clone of the database pool reference to the service which allows interaction with database
    // "pool" is Type of web::Data<DbPool> which is Actix Web for shared state
    // ".get_ref()" Returns a reference to the DbPool
    // ".clone()" Clones the database pool ref, think of this as a handle or pointer to the actual pool
    // "UserService::new(...)" Calls the constructor for UserService, passing in the cloned pool 
    let user_service = UserService::new(pool.get_ref().clone());

// Call the "update_user" business logic asynchronously on the "user_service" instance.
// Pass the "user_id" (extracted from the URL path as a Uuid) and the deserialized update data from the JSON body.
// The ".await?" waits for the async operation and propagates any errors.
// The result is an Option<User>, which is matched below to handle success or not-found.
    match user_service.update_user(user_id, json.into_inner()).await? {
        // If the update operation returns Some(user), it means the user was found and updated.
        // Convert the internal User model to a UserResponse DTO and return a 200 OK HTTP response with the updated user as JSON.
        Some(user) => {
            let response = UserResponse::from(user);
            Ok(HttpResponse::Ok().json(response))
        }
        None => {
            // If the update operation returns None, it means no user was found for the given ID.
            // Return a 404 Not Found error using the custom AppError type for consistent error handling.
            Err(AppError::user_not_found(user_id))
        }
    }
}

// Delete a User
// 
// This Handler demonstrates how to Return a different status code based on
// whether the resource exists or not. Some APIs return 404 if trying to delete a non-existent resource,
// others return 204. We can be explicit about whether or not the resource exists.
pub async fn delete_user(
    // 
    pool: web::Data<DbPool>,
    // 
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // 
    let user_id = path.into_inner();
    // 
    let user_service = UserService::new(pool.get_ref().clone());

    // 
    let was_deleted = user_service.delete_user(user_id).await?;

    // 
    if was_deleted {
        // "204 No Content" means "Successfully Deleted, no body in Response"
        Ok(HttpResponse::NoContent().finish())
    } else {
        // If no User found, Return user not found
        Err(AppError::user_not_found(user_id))
    }
}

// Endpoint Health Check
// 
// This is an Endpoint for other services to check if App is running and can connect to Database
// This Endpoint is vital for Load Balancers, Container Orchestration Systems, and Monitoring Tooling
pub async fn health_check(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    // Verify Database Connectivity by attempting to get a connection from the pool
    let _client = pool.get().await.map_err(AppError::DatabaseConnection)?;

    // 
    let health_response = serde_json::json!({
        // 
        "status": "healthy",
        // 
        "timestamp": chrono::Utc::now(),
        // 
        "service": "user-api"
    });

    // 
    Ok(HttpResponse::Ok().json(health_response))
}