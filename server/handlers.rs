// 
use actix_web::{web, HttpResponse, Responder};

// Call local crate "db.rs" and Pub Struct "PgPool" for use
use crate::database::db::PgPool;
// Use local crate "models.rs", use Struct "Item"
use crate::models::{
    Item, 
    CreateItem, 
    UpdateItem, 
    DeleteItem,
    Workspace,
    CreateWorkspace,
    UpdateWorkspace,
    DeleteWorkspace,
    WorkspaceTools,
    CreateWorkspaceTool,
    UpdateWorkspaceTool,
    DeleteWorkspaceTool,
    Spreadsheets,
    CreateSpreadsheet,
    UpdateSpreadsheet,
    DeleteSpreadsheet,
};
// Call local crate "database/queries.rs" and Public Functions for CRUD DB Queries
use crate::database::queries::{
    list_items,
    create_item,
    update_item,
    delete_item,
    get_workspace_column_names,
    list_workspaces,
    create_workspace,
    update_workspace,
    delete_workspace,
};

// Handler - GET All Items
// pool is shared ref to database connection pool
// web::Data<PgPool> Actix Web wraps Pool in a special type to allow safe sharing between requests
pub async fn list_items_handler(pool: web::Data<PgPool>) -> impl Responder {
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
pub async fn create_item_handler(pool: web::Data<PgPool>, payload: web::Json<CreateItem>) -> impl Responder {
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
pub async fn update_item_handler(pool: web::Data<PgPool>, payload: web::Json<UpdateItem>) -> impl Responder {
    // 
    match update_item(pool.get_ref(), &payload.into_inner()).await {
        // 
        Ok(item) => HttpResponse::Ok().json(item),
        // 
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler - DELETE - Delete Item
pub async fn delete_item_handler(pool: web::Data<PgPool>, payload: web::Json<DeleteItem>) -> impl Responder {
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

// Handler - Get Workspace Column Names
pub async fn get_workspace_column_name_handler(pool: web::Data<PgPool>) -> HttpResponse {
    // 
    match get_workspace_column_names(&pool).await {
        Ok(columns) => HttpResponse::Ok().json(columns),
        Err(e) => {
            eprintln!("Error fetching workspace columns: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch columns")
        }
    }
}

// Handler - GET All Workspaces
// pool is shared ref to database connection pool
// web::Data<PgPool> Actix Web wraps Pool in a special type to allow safe sharing between requests
pub async fn list_workspaces_handler(pool: web::Data<PgPool>) -> impl Responder {
    // DB Function to get all items...
    // Rust Match statement, like switch/case
    // list_items: Database Query Function
    // pool.get_ref() Gets a reference to the actual connection pool from Actix Web Wrapper
    // await? becuase this is an async operation
    match list_workspaces(pool.get_ref()).await {
        // Ok(item): If DB Query is success, return result called items
        // HttpResponse::Ok() creates an HTTP response with status code 200 (OK)
        // .json(items) converts list of items into JSON format and places inside response body to be sent back to client
        Ok(workspaces) => HttpResponse::Ok().json(workspaces),
        // Err(e) If error returned, send back "e"
        // HttpResponse::InternalServerError() Creates an HTTP response with status code 500 (Internal Service Error)
        // .body(format!("Error: {}", e)) Places message in the response body describing the error
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler - POST - Create Workspace
pub async fn create_workspace_handler(pool: web::Data<PgPool>, payload: web::Json<CreateWorkspace>) -> impl Responder {
    // Validate input and call DB function to create item
    // pass reference to pool and payload(CreateItem) to "create_item" DB Query Function
    // .into_inner() is needed to extract the struct from the Actix Web wrapper that is payload
    match create_workspace(pool.get_ref(), &payload.into_inner()).await {
        // 
        Ok(workspace) => HttpResponse::Ok().json(workspace),
        // 
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler - PUT - Update Workspace
pub async fn update_workspace_handler(pool: web::Data<PgPool>, payload: web::Json<UpdateWorkspace>) -> impl Responder {
    // 
    match update_workspace(pool.get_ref(), &payload.into_inner()).await {
        // 
        Ok(workspace) => HttpResponse::Ok().json(workspace),
        // 
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler - DELETE - Delete Workspace
pub async fn delete_workspace_handler(pool: web::Data<PgPool>, payload: web::Json<DeleteWorkspace>) -> impl Responder {
    // 
    match delete_workspace(pool.get_ref(), &payload.into_inner()).await {
        // 
        Ok(rows_deleted) => {
            if rows_deleted == 1 {
                HttpResponse::Ok().body("Workspace deleted successfully")
            } else {
                HttpResponse::NotFound().body("Workspace not found")
            }
        }
        // 
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}