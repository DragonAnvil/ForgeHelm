// 
use actix_web::{web, HttpResponse, Responder};

// Call local crate "db.rs" and Pub Struct "PgPool" for use
use crate::db::PgPool;
// Call local crate "models.rs" and Pub Structs for use
use crate::models::{CreateItem, UpdateItem};
// Call local crate "database/queries.rs" and Public Functions for CRUD DB Queries
use crate::database::queries::{list_items, create_item, update_item, delete_item};

// Handler - GET All Items
// pool is shared ref to database connection pool
// web::Data<PgPool> Actix Web wraps Pool in a special type to allow safe sharing between requests
pub async fn list_items(pool: web::Data<PgPool>) -> impl Responder {
    // DB Function to get all items...
    // Rust Match statement, like switch/case
    // list_items: Database Query Function
    // pool.get_ref() Gets a reference to the actual connection pool from Actix Web Wrapper
    // await? becuase this is an async operation
    match list_items(pool.get_ref()).await {
        // Ok(item): If DB Query is success, return result called items
        // HttpResponse::Ok() creates an HTTP response with status code 200 (OK)
        // .json(items) converts list of items into JSON format and places inside response body to be sent back to client
        Ok(items) => HttpResponse::Ok().json(items),
        // Err(e) If error returned, send back "e"
        // HttpResponse::InternalServerError() Creates an HTTP response with status code 500 (Internal Service Error)
        // .body(format!("Error: {}", e)) Places message in the response body describing the error
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler - GET - 
// future...

// Handler - POST - Create Item 
pub async fn create_item(pool: web::Data<PgPool>, payload: web::Json<CreateItem>) -> impl Responder {
    // Validate input and call DB function to create item
    // pass reference to pool and payload(CreateItem) to "create_item" DB Query Function
    // .into_inner() is needed to extract the struct from the Actix Web wrapper that is payload
    match create_item(pool.get_ref(), &payload.into_inner()).await {
        // 
        Ok(item) => HttpResponse::Ok().json(item),
        // 
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler - PUT - Update Item
pub async fn update_item(pool: web::Data<PgPool>, payload: web::Json<UpdateItem>) -> impl Responder {
    // 
    match update_item(pool.get_ref(), &payload.into_inner()).await {
        // 
        Ok(item) => HttpResponse::Ok().json(item),
        // 
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler - DELETE - Delete Item
pub async fn delete_item(pool: web::Data<PgPool>, payload: web::Json<DeleteItem>) -> impl Responder {
    // 
    match delete_item(pool.get_ref(), &payload.into_inner()).await {
        // 
        Ok(rows_deleted) => {
            if rows_deleted == 1 {
                HttpResponse::Ok().body("Item deleted successfully")
            } else {
                HttpResponse::NotFound().body("Item not found")
            }
        }
        // 
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
