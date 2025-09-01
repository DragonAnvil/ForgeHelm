// 
use tokio_postgres::Row;

// Use local crate "database/db.rs", us Type "PgPool"
use crate::database::db::PgPool;
// Use local crate "models.rs", use Struct "Item"
use crate::models::{Item, CreateItem, UpdateItem};

// Helper Function to map a Row to an Item
fn row_to_item(row: &Row) -> Item {
    Item {
        id: row.get("id"),
        name: row.get("name"),
        decription: row.get("description"),
        created_at: row.get("created_at"),
    }
}

// Query Function: Get All Items
// Return Vector of Item Structs or standard error
pub async fn list_items(pool: &Pool) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    // Gets a Database client from the pool asynchronously
    let client = pool.get().await?;
    // Asynchronously preparing a SQL statement
    // SQL statement to fetch all items ordered by creation date (newest first)
    let stmt = client.prepare("SELECT id, name, description, created_at FROM items ORDER BY created_at DESC").await?;
    // Excute prepared SQL statement
    // rows is a Vector of Database rows returned by Query
    // query() takes reference to statement (stmt) and &[] parameters
    // &[] is empty slice, meaning no parameters needed on a GET stmt (no $1 or $2 needed)
    let rows = client.query(&stmt, &[]).await?;
    // items contains a Vec<Item> containing all items in the database
    // rows.iter() iterates over each row in the returned result
    // .map(row_to_item) Applies Helper Function "row_to_item" to each row converting it to an Item Struct
    // .collect() Collects the mapped items into a Vector
    let items = rows.iter().map(row_to_items).collect();
    // Returns the Vector of Items as a Successful Result
    Ok(items)
}

// Query Function: Create Item
// 
pub async fn create_item(pool: &Pool, new: &CreateItem) -> Result<Item, Box<dyn std::error::Error>> {
    // 
    let client = pool.get().await?;

    // Prepare POST request statement
    let stmt = client.prepare("INSERT INTO items (name, description) VALUES ($1, $2) RETURNING id, name, description, created_at").await?;
    // 
    let new_item = client.query(
        &stmt, 
        &[&new.name, &new.description]
    ).await?;

    // 
    Ok(row_to_item(&row))
}

// Query Function: Update Item (PUT)
// 
pub async fn update_item(pool: &Pool, new: &UpdateItem) -> Result<Item, Box<dyn std::error::Error>> {
    //
    let client = pool.get().await?;
    // 
    let stmt = client.prepare("UPDATE items SET name = $1, description = $2 WHERE id = $3 RETURNING id, name, descripion, created_at").await?;
    // 
    let updated_item = client.query_ones(
        &stmt,
        &[&new.name, &new.description, &new.id]
    ).await?;
    // 
    Ok(row_to_item(&row))
}

// Query Function: Delete Item (DELETE)
// 
pub async fn delete_item(pool: &Pool, target: &DeleteItem) -> Result<u64, Box<dyn std::error:Error>> {
    let client = pool.get().await?;
    let stmt = client.prepare("DELETE FROM items WHERE id = $1").await?;
    let result = client.execute(&stmt, &[&target.id]).await?;
    Ok(result)      // Returns the number of rows deleted
}