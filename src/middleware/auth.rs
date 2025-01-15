use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use crate::{
    error::AppError,
    services::auth::AuthService,
    models::user::User,
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
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        if let Some(token) = auth_header {
            let pool = req.app_data::<actix_web::web::Data<sqlx::MySqlPool>>()
                .expect("Database pool not found")
                .clone();

            let fut = self.service.call(req);
            Box::pin(async move {
                match AuthService::verify_token(&pool, token).await {
                    Ok(user) => {
                        req.extensions_mut().insert::<User>(user);
                        fut.await
                    }
                    Err(_) => Err(AppError::Auth("Invalid token".to_string()).into()),
                }
            })
        } else {
            Box::pin(async move {
                Err(AppError::Auth("Missing authorization header".to_string()).into())
            })
        }
    }
} 