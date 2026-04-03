use actix_service::Service;
use actix_web::dev::{forward_ready, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::body::{EitherBody, MessageBody};
use futures_util::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

/// JWT-based auth middleware. Validates `Authorization: Bearer <jwt>` tokens
/// using the provided secret and rejects unauthorized requests with `401`.
#[derive(Clone)]
pub struct AuthMiddleware {
    secret: String,
}

impl AuthMiddleware {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

pub struct AuthMiddlewareService<S> {
    service: Arc<S>,
    secret: String,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: Send + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Arc::new(service),
            secret: self.secret.clone(),
        }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: Send + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone();
        let srv = self.service.clone();

        Box::pin(async move {
            // Allow preflight
            if req.method() == actix_web::http::Method::OPTIONS {
                let resp = HttpResponse::Ok().finish().map_into_right_body();
                return Ok(req.into_response(resp));
            }

            // Check Authorization header (case-insensitive "Bearer" scheme)
            if let Some(header_val) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
                if let Ok(s) = header_val.to_str() {
                    let mut parts = s.splitn(2, ' ');
                    if let Some(scheme) = parts.next() {
                        if scheme.eq_ignore_ascii_case("Bearer") {
                            if let Some(token_raw) = parts.next() {
                                let token = token_raw.trim();
                                let validation = Validation::new(Algorithm::HS256);
                                match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
                                    Ok(data) => {
                                        // attach claims to request extensions for handlers
                                        let mut req = req;
                                        req.extensions_mut().insert(data.claims);
                                        let res = srv.call(req).await?;
                                        return Ok(res.map_into_left_body());
                                    }
                                    Err(e) => {
                                        log::debug!("JWT decode error: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let er = ErrorResponse { error: "Unauthorized".to_string() };
            let resp = HttpResponse::Unauthorized().json(er).map_into_right_body();
            Ok(req.into_response(resp))
        })
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
