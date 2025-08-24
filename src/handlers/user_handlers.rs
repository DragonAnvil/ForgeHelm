// 
use actix_web::{web, HttpResponse, Result};
// 
use uuid::Uuid;

// 
use crate::database::DbPool;
// 
use crate::models::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
// 
use crate::services::user_service::UserService;
// 
use crate::utils::errors::AppError;


//! Create a New User
//! 
//! This handler demonstrates the usual flow of an HTTP Request in Actix Web
//! 1. Extract & Validate the Request Data
//! 2. Pass Data to the Service Layer for Business Logic
//! 3. Convert Result into an appropriate HTTP Response
//! 
//! 
pub async fn create_user(
    pool: web::Data<DbPool>,            // Dependency Injection of Database Pool
    json: web::Json<CreateUserRequest>, // Auto JSON Deserialization
) -> Result<HttpResponse, AppError> {
    
    // Create the Service Instance
    let user_service = UserService::new(pool.get_ref().clone());

    // Call the Business Logic
    // "into_inner()" method extracts the CreateUserRequest from web::Json wrapper
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
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,      // Extract UUID from URL Path
) -> Result<HttpResponse, AppError> {
    // 
    let user_id = path.into_inner();
    // 
    let user_service = UserService::new(pool.get_ref().clone());

    // 
    match user_service.get_user_by_id(user_id).await? {
        // 
        Some(user) => {
            // 
            let response = UserResponse::from(user);
            // 
            Ok(HttpResponse::Ok().json(response))
        }
        // 
        None => {
            // Return "404 Error" using custom error type
            Err(AppError::user_not_found(user_id))
        }
    }
}


// Query Parameters for Endpoint: get_users
// 
// This Struct demonstrates how to handle Optional query parameters in a Type-Safe way
// Actix Web can auto Deserialize Query Strings into this Struct
// Much safer than manually parsing Query Parameters
#[derive(serde::Deserialize)]
pub struct GetUsersQuery {
    // 
    pub limit: Option<i64>,
    // 
    pub offset: Option<i64>,
    // 
    pub search: Option<String>,
}

// Get All Users with Optional Pagination & Search
// 
// This Handler shows how to handle complex Query Parameters
// Demonstrates how different business logic paths can share the same HTTP Endpoint
pub async fn get_users(
    // 
    pool: web::Data<DbPool>,
    // 
    query: web::Query<GetUsersQuery>,
) -> Result<HttpResponse, AppError> {
    // 
    let user_service = UserService::new(pool.get_ref().clone());

    // 
    let users = if let Some(ref search_term) = queary.search {
        // If a search term is provided, use the search functionality
        user_service.search_users_by_name(search_term).await?
    } else {
        // Otherwise, Return paginated results
        user_service.get_users(query.limit, query.offset).await?
    };

    // Convert Vec<User> to Vec<UserResponse>
    // "collect()" gathers iterator results into a Vec
    let response : Vec<UserResponse> = users
        .into_iter()
        .map(USerResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(responses))
}

// Update an Existing User
// 
// This Handler combines Path Parameters (for User ID) with a JSON body (for Update Data)
// Demonstrates how to handle partial updates and return appropriate status codes
pub async fn update_user(
    // 
    pool: web::Data<DbPool>,
    // 
    path: web::Path<Uuid>,
    // 
    json: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    // 
    let user_id = path.into_inner();
    // 
    let user_service = UserService::new(pool.get_ref().clone());

    // 
    match user_service.update_user(user_id, json.into_inner()).await? {
        // 
        Some(user) => {
            // 
            let response = UserResponse::from(user);
            // 
            Ok(HttpResponse::Ok().json(response))
        }
        None => {
            // If no User found for update, Return 404 Status Code
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