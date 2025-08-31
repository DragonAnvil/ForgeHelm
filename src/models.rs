// 
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Derives the serde::Serialize trait to enable converting struct fields into formats like JSON
#[derive(Serialize)]
// Public Struct "Item"
// Used to define the types for each member of the Item table
pub struct Item {
    pub id: i32,
    pub name: String,                   // Since Option is not used here, a String must be provided
    pub description: Option<String>,    // Option allows empty values, but if present, it must be a String
    pub created_at: DateTime<Utc>,
}

// Create Item Struct
#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
}

// Update Item Struct
#[derive(Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,           // Option is used here for both
    pub description: Option<String>,    // because the fields could be empty with an update
}
