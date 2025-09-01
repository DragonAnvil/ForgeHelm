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
    pub id: i32,                        // Unique Identifier for Item, used to find correct Item for Update Query
    pub name: Option<String>,           // Option is used here for both
    pub description: Option<String>,    // because the fields could be empty with an update
}

// Delete Item Struct
#[derive(Deserialize)]
pub struct DeleteItem {
    pub id: i32,                        // Needed to find correct Item to Delete in Query
}

// Workspaces Table Structs ------------------------------------

// Public Struct - Workspaces
#[derive(Serialize)]
pub struct Workspace {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// Create Workspace
#[derive(Deserialize)]
pub struct CreateWorkspace {
    pub name: String,
    pub description: String,
    pub owner: String,
}

// Update Workspace
#[derive(Deserialize)]
pub struct UpdateWorkspace {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
}

// Delete Workspace
#[derive(Deserialize)]
pub struct DeleteWorkspace {
    pub id: i32,
}


// Workspace Tools Structs ---------------------------------

// Workspace Tools
#[derive(Serialize)]
pub struct WorkspaceTools {
    pub id: i32,
    pub workspace_id: i32,
    pub name: String,
    pub description: String,
    pub doctype: String,            // SQL table schema is "type", might need to revisit
    pub owner: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// Create WorkspaceTool
#[derive(Deserialize)]
pub struct CreateWorkspaceTool {
    pub name: String,
    pub description: String,
    pub doctype: String,
    pub owner: String,
}

// Update WorkspaceTool
#[derive(Deserialize)]
pub struct UpdateWorkspaceTool {
    pub id: i32,
    pub name: Option<String>,           // doctype is left off, cannot be changed after creation
    pub description: Option<String>,    // might be an option if can convert to different views
    pub owner: Option<String>,
}

// Delete WorkspaceTool
#[derive(Deserialize)]
pub struct DeleteWorkspaceTool {
    pub id: i32,
}


// Spreadsheets Structs ------------------------------------------

// Spreadsheets
#[derive(Serialize)]
pub struct Spreadsheets {
    pub id: i32,
    pub user_id: i32,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create Spreadsheet
#[derive(Deserialize)]
pub struct CreateSpreadsheet {
    pub owner: String,
    pub name: String,
    pub description: String,
}

// Update Spreadsheet
#[derive(Deserialize)]
pub struct UpdateSpreadsheet {
    pub id: i32,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

// Delete Spreadsheet
#[derive(Deserialize)]
pub struct DeleteSpreadsheet {
    pub id: i32,
}