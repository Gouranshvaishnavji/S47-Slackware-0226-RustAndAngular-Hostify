# Rust Data Models — Structs, Enums, and Integration

This document explains how we defined Rust data models (structs and enums) and integrated them into the backend scaffold for the project.

Overview

- Rust structs model domain entities (like `Product`).
- Enums model discrete state or typed variants (like `ProductStatus`).
- We use `serde` to serialize/deserialize models to/from JSON for HTTP handlers.

Files changed / created

- `conceptKlarity/rust-backend/src/models/product.rs` — contains the `Product` struct and `ProductStatus` enum, with `serde` derives.
- `conceptKlarity/rust-backend/src/handlers/items.rs` — constructs and returns `Product` JSON via Actix handlers.

Example: `Product` struct and `ProductStatus` enum

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    Available,
    OutOfStock,
    Discontinued,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub status: ProductStatus,
}
```

Why these derives and attributes

- `Serialize` / `Deserialize` (from `serde`) allow automatic JSON encoding and decoding in handlers.
- `Debug` and `Clone` are useful for development and testing.
- `#[serde(rename_all = "snake_case")]` makes enum variant names map to predictable JSON strings (e.g., `available`).

Integrating into Actix handlers

In `src/handlers/items.rs` we import the model types and return JSON responses directly:

```rust
use crate::models::product::{Product, ProductStatus};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_items() -> impl Responder {
    let items = vec![Product { id: 1, name: "Example".into(), price: 9.99, description: Some("Sample".into()), status: ProductStatus::Available }];
    HttpResponse::Ok().json(items)
}
```

Mapping to the frontend (TypeScript)

Frontend TypeScript interfaces should mirror the Rust model shape so serialised JSON maps cleanly.
Example TypeScript model:

```ts
export type ProductStatus = 'available' | 'out_of_stock' | 'discontinued';

export interface Product {
  id: number;
  name: string;
  price: number;
  description?: string;
  status: ProductStatus;
}
```

When we return `Product` JSON from Actix, the Angular `HttpClient` can deserialize into `Product` objects if the TypeScript interface matches the JSON shape.

Database & persistence notes

- If using SQLx or another ORM, annotate Rust models for DB mapping (example):

```rust
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ProductRow { /* fields matching DB columns */ }
```

- Convert between DB rows and API DTOs (separate `models::db` vs `models::api` modules) to avoid leaking DB-specific types into API contracts.

Testing and verification

- Start the backend and query the endpoint to verify JSON output:

```bash
cd conceptKlarity/rust-backend
cargo run

curl http://localhost:8080/api/items
```

- Expect JSON where `status` is a string like `"available"` and the structure matches the TypeScript `Product` interface.

Best practices and tips

- Keep API DTOs (what you send over HTTP) separate from internal DB models where useful.
- Use `serde` attributes to control JSON field names and enum representation.
- When updating models, update both the Rust DTO and the TypeScript interface so the front-end and back-end remain in sync.

Next steps (optional)

- Add `sqlx::FromRow` derives and implement DB migrations in `rust-backend/migrations/` when moving from in-memory examples to a persistent store.
- Add unit tests for (de)serialization to ensure enum variants and optional fields behave as expected.

---

## Typed Request/Response Models & Endpoint

We added a typed request and response model and a working POST endpoint to demonstrate Serde-based (de)serialization and typed APIs.

Models added (Rust)

```rust
// request
#[derive(Deserialize, Debug)]
pub struct CreateProductRequest {
        pub name: String,
        pub price: f64,
        pub description: Option<String>,
}

// response
#[derive(Serialize, Debug)]
pub struct ProductResponse {
        pub id: i32,
        pub name: String,
        pub price: f64,
        pub description: Option<String>,
        pub status: ProductStatus,
}
```

Endpoint implemented

- POST `/api/products` — accepts JSON matching `CreateProductRequest` and returns `201 Created` with JSON `ProductResponse`.
- The handler uses `web::Json<CreateProductRequest>` extractor; Actix + Serde automatically deserializes the request body into the typed struct. Invalid JSON or missing required fields result in a `400 Bad Request` automatically.

Example handler (simplified):

```rust
pub async fn create_product(req: web::Json<CreateProductRequest>) -> impl Responder {
        let payload = req.into_inner();
        let response = ProductResponse { id: 100, name: payload.name, price: payload.price, description: payload.description, status: ProductStatus::Available };
        HttpResponse::Created().json(response)
}
```

Testing with curl

Send a valid request:

```bash
curl -i -X POST http://localhost:8080/api/products \
    -H "Content-Type: application/json" \
    -d '{"name":"New Product","price":12.5,"description":"demo"}'
```

Expected result: HTTP/1.1 201 Created and JSON body matching `ProductResponse`.

Send invalid JSON (will produce 400):

```bash
curl -i -X POST http://localhost:8080/api/products \
    -H "Content-Type: application/json" \
    -d '{"name":"MissingPrice"}'
```

Why Serde

- `serde` provides safe, performant, and flexible (de)serialization between Rust types and JSON. Using typed structs avoids manual parsing and ensures compile-time guarantees about field types.

Notes on error handling

- The `web::Json` extractor rejects invalid JSON or mismatched types with a 400 response by default. For additional validation (e.g., non-negative price) you can add explicit checks in the handler and return `HttpResponse::BadRequest()` when needed.

