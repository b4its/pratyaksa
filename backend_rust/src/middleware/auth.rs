use crate::config::AppConfig;
use crate::errors::AppError;
use crate::models::auth::Claims;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                if v.starts_with("Bearer ") {
                    Some(v.trim_start_matches("Bearer ").to_string())
                } else {
                    None
                }
            });

        let token = match auth_header {
            Some(t) => t,
            None => {
                let err = AppError::Unauthorized("Token tidak ditemukan".to_string());
                return Box::pin(ready(Err(actix_web::Error::from(err))));
            }
        };

        // Get JWT secret from app data
        let config = req
            .app_data::<actix_web::web::Data<AppConfig>>()
            .cloned()
            .expect("AppConfig not found");

        let secret = config.jwt_secret.clone();

        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => {
                req.extensions_mut().insert(token_data.claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(_) => {
                let err = AppError::Unauthorized("Token tidak valid atau sudah kadaluarsa".to_string());
                Box::pin(ready(Err(actix_web::Error::from(err))))
            }
        }
    }
}

/// Helper to extract claims from request extensions
pub fn extract_claims(req: &actix_web::HttpRequest) -> Result<Claims, AppError> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("Claims tidak ditemukan".to_string()))
}
