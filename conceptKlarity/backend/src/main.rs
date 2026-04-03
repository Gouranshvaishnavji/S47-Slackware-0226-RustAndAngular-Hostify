use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, net::SocketAddr, sync::{Arc, Mutex}};

#[derive(Clone)]
struct AppState {
    products: Arc<Mutex<Vec<Product>>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Product {
    id: u64,
    name: String,
    price: u64,
    description: Option<String>,
}

#[derive(Deserialize)]
struct CreateProduct {
    name: String,
    price: u64,
    description: Option<String>,
}

/// API-level errors converted into structured JSON responses
enum ApiError {
    BadRequest(String),
    NotFound(String),
    Internal(anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest(msg) => {
                let body = Json(json!({ "error": msg }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            ApiError::NotFound(msg) => {
                let body = Json(json!({ "error": msg }));
                (StatusCode::NOT_FOUND, body).into_response()
            }
            ApiError::Internal(err) => {
                // Internal error: hide details but include a request id or message
                let body = Json(json!({ "error": "internal server error" }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn list_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, ApiError> {
    let guard = state.products.lock().map_err(|e| ApiError::Internal(anyhow::anyhow!("lock poisoned: {}", e)))?;
    Ok(Json(guard.clone()))
}

async fn create_product(State(state): State<AppState>, Json(payload): Json<CreateProduct>) -> Result<(StatusCode, Json<Product>), ApiError> {
    // Validate input using Result / Option instead of panics
    if payload.name.trim().is_empty() {
        return Err(ApiError::BadRequest("name is required".into()));
    }
    if payload.price == 0 {
        return Err(ApiError::BadRequest("price must be > 0".into()));
    }

    let mut guard = state.products.lock().map_err(|e| ApiError::Internal(anyhow::anyhow!("lock poisoned: {}", e)))?;
    let id = guard.last().map(|p| p.id + 1).unwrap_or(1);
    let new = Product { id, name: payload.name, price: payload.price, description: payload.description };
    guard.push(new.clone());
    Ok((StatusCode::CREATED, Json(new)))
}

fn read_port_from_env() -> Result<u16> {
    let default = "8080".to_string();
    let raw = env::var("PORT").unwrap_or(default);
    let port: u16 = raw.parse().context("failed to parse PORT env var as u16")?;
    Ok(port)
}

#[tokio::main]
async fn main() -> Result<()> {
    let port = read_port_from_env().context("reading port failed")?;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let state = AppState { products: Arc::new(Mutex::new(vec![
        Product { id: 1, name: "Wireless Mouse".into(), price: 899, description: Some("Ergonomic".into()) },
        Product { id: 2, name: "Mechanical Keyboard".into(), price: 3499, description: Some("RGB".into()) },
    ])) };

    let app = Router::new()
        .route("/health", get(health))
        .route("/products", get(list_products).post(create_product))
        .with_state(state);

    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.context("server failed")?;
    Ok(())
}
