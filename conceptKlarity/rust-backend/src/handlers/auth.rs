use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::config;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    let payload = req.into_inner();
    let expected_user = config::get_admin_user();
    let expected_pass = config::get_admin_password();

    if payload.username != expected_user || payload.password != expected_pass {
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    let exp = now + 60 * 60; // 1 hour expiration

    let claims = Claims { sub: payload.username, iat: now, exp };

    let secret = config::get_jwt_secret();
    match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
        Err(e) => {
            log::error!("JWT encode error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
