use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use crate::{services::auth::AuthService, error::AppError};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
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
        let pool = req.app_data::<web::Data<MySqlPool>>().unwrap().clone();
        
        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                let token = auth_header.to_str()
                    .map_err(|_| AppError::Auth("Invalid token format".into()))?
                    .replace("Bearer ", "");
                
                let user = AuthService::verify_token(&pool, &token).await?;
                req.extensions_mut().insert(user);
                
                Ok(self.service.call(req).await?)
            } else {
                Err(AppError::Auth("No token provided".into()).into())
            }
        })
    }
} 