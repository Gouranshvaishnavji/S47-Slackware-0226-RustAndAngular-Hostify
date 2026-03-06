# Rust Backend Architecture: Angular to PostgreSQL

## 1. Why Rust for our Backend?
Rust was chosen for this project to ensure high performance and memory safety. Key advantages include:
* **Memory Safety:** Elimination of null pointer crashes.
* **Speed:** Performance comparable to C++, ideal for real-time data processing.
* **Fearless Concurrency:** Safe multi-threading for handling high-traffic requests.

---

## 2. Scenario: Inventory Management System
We are building an endpoint to track warehouse stock.

### A. The Data Model (Structs)
```rust
#[derive(serde::Deserialize, serde::Serialize)]
struct InventoryItem {
    id: Option<i32>,
    name: String,
    sku: String,
    quantity: i32,
}
B. The Handler (Logic)
Rust
#[post("/inventory")]
async fn add_to_stock(item: web::Json<InventoryItem>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO inventory (name, sku, quantity) VALUES ($1, $2, $3) RETURNING id",
        item.name,
        item.sku,
        item.quantity
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
3. Architecture Flow Diagram
Angular Component: User enters "Mechanical Keyboard" and clicks "Add".

Angular Service: HttpClient sends a POST request with JSON payload to /api/inventory.

Rust Route: The /api/inventory scope identifies the add_to_stock handler.

Rust Handler: Deserializes JSON into the InventoryItem struct, validating data types.

SQLx Query: Executes an asynchronous INSERT into PostgreSQL.

Database: PostgreSQL confirms storage and returns the new ID.

JSON Response: Rust sends { "id": 101 } back to the frontend.

UI Update: Angular updates the table to show the new item.

4. Development Notes
Type Safety: By using shared structs, the API ensures that a string is never saved into an integer column in the DB.

Async Execution: The server remains responsive even during heavy database I/O operations.

Compile-time Checks: SQLx validates that our queries match the actual PostgreSQL schema before the code even runs.