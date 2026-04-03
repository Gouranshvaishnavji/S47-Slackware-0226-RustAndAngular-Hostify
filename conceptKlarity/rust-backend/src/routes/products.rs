use actix_web::web;
use crate::handlers::products::{create_product, list_products, get_product, update_product, delete_product};
use crate::middleware::auth::AuthMiddleware;
use crate::config;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Public read endpoints
    cfg.service(
        web::scope("/api/products")
            .route("", web::get().to(list_products))
            .route("/{id}", web::get().to(get_product))
    );

    // Protected write endpoints (POST/PUT/DELETE) wrapped with AuthMiddleware
    let secret = config::get_jwt_secret();
    cfg.service(
        web::scope("/api/products")
            .wrap(AuthMiddleware::new(secret))
            .route("", web::post().to(create_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product))
    );
}
