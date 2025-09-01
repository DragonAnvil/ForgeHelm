// 
use actix_web::web;

// Import local crate "handlers.rs" and all public functions for use here
use crate::handlers::{list_items_handler, create_item_handler, update_item_handler, delete_item_handler};

// Configuration for routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/items")
                .route(web::get().to(list_items_handler))       // GET      /items
                .route(web::post().to(create_item_handler))     // POST     /items
                .route(web::put().to(update_item_handler))     // PUT      /items
                .route(web::delete().to(delete_item_handler))     // DELETE   /items
        );
}