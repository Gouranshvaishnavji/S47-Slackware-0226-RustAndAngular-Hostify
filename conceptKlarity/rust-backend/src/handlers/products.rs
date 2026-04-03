use actix_web::{web, HttpResponse, Responder};
use crate::models::product::{CreateProductRequest, ProductResponse, ProductStatus};

pub async fn create_product(req: web::Json<CreateProductRequest>) -> impl Responder {
    // The web::Json extractor uses Serde to deserialize JSON into CreateProductRequest.
    // If the payload is invalid JSON or missing required fields, Actix will return a 400 Bad Request automatically.

    let payload = req.into_inner();

    // In a real app you would persist and generate an ID. Here we simulate creating an ID.
    let response = ProductResponse {
        id: 100, // simulated id
        name: payload.name,
        price: payload.price,
        description: payload.description,
        status: ProductStatus::Available,
    };

    HttpResponse::Created().json(response)
}
