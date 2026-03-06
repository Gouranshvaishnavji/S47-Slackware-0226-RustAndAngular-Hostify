# API Explaination — Rust Backend: Route → Handler → DB → Response

## API Explanation (Route → Handler → Logic → Response)

- **Route maps to handler:** In Rust web frameworks (e.g., Actix Web, Axum) each HTTP route is registered with a handler function. The router matches the incoming method and path and dispatches the request to that handler. Example registrations:

- Actix Web: `app.route("/items", web::post().to(create_item))`
- Axum: `route("/items", post(create_item))`

- **How the handler receives and processes the request:** Handlers accept typed extractors for request parts. For JSON bodies you typically use `Json<MyInput>` where `MyInput: Deserialize`. The handler flow is:
  - Extract and parse input into a typed struct.
  - Validate fields (via `validator` or custom checks).
  - Perform business logic (services, computations, side effects).
  - Call database functions (repository layer) to read/write data.
  - Construct a typed response struct and return it (often as `Json<MyResponse>`).

- **How typed structs ensure safe input/output:** Request/response shapes are modeled as Rust structs with `serde::Deserialize` / `serde::Serialize`. This provides:
  - Compile-time guarantees of field names and types.
  - Clear contracts between frontend and backend.
  - Safer refactors because mismatches are compile-time errors, not runtime surprises.

- **How the API returns JSON responses:** Handlers return values that the framework serializes to JSON (e.g., `Ok(Json(my_response))` in Actix or `Json(my_response)` in Axum). Error cases are mapped to HTTP status codes and structured JSON error bodies so frontends can reliably parse and react.

- **SQLx / SeaORM and PostgreSQL:**
  - SQLx: an async, zero-cost SQL crate that supports compile-time checked queries (with `macros` feature). It runs parameterized queries and can deserialize rows directly into typed structs.
  - SeaORM: an ORM that maps entities to tables and provides active models and query builders. It returns typed entity structs from queries.
  - Best practice: isolate DB access in a repository/data layer so handlers call simple, typed functions like `db.create_item(&pool, input).await` and receive `Result<Item, DbError>`.

## End-to-End API Request Flow (textual diagram)

Angular Service (HTTP Request)
  ↓
Rust Route
  ↓
Rust Handler (Validation + Business Logic)
  ↓
PostgreSQL Query (via SQLx or SeaORM)
  ↓
Rust JSON Response
  ↓
Angular UI Update

Notes on each step:
- Angular Service: issues an HTTP request (method, URL, JSON body). Uses typed TypeScript interfaces for request/response shapes where possible.
- Rust Route: router matches path and method and extracts path/query params.
- Rust Handler: uses typed `Deserialize` structs, runs validation, applies business logic, and calls repository functions.
- PostgreSQL Query: parameterized queries or ORM calls run against the DB, returning typed rows or entity structs.
- Rust JSON Response: handler maps domain data into `Serialize` structs and returns JSON with the appropriate status code.
- Angular UI Update: receives JSON, updates state/UI via component services.

## Example minimal handler wiring (conceptual)

Actix Web (conceptual):

```rust
#[derive(Deserialize)]
struct CreateItem { name: String, qty: i32 }

#[derive(Serialize)]
struct ItemResponse { id: i32, name: String, qty: i32 }

async fn create_item(Json(payload): Json<CreateItem>, pool: Data<PgPool>) -> impl Responder {
    // validation (lengths, ranges)
    // db call: let item = repository::insert_item(&pool, payload).await?;
    // return Json(ItemResponse { ... })
}
```

## Reflection

Type-safe, strongly-validated Rust APIs are important because they move many classes of errors to compile time, improving reliability and safety. Strong typing and explicit validation create predictable APIs, reduce runtime crashes, and make refactors safer — essential qualities when building maintainable, robust backend services.

---

This file consolidates the previous PR content; the separate SVG diagram file was removed per request and replaced by the textual flow above.
