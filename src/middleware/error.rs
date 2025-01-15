use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use crate::{error::AppError, utils::Response};

pub struct ErrorMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ErrorMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorMiddlewareService { service }))
    }
}

pub struct ErrorMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorMiddlewareService<S>
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
        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => Ok(res),
                Err(err) => {
                    let app_error = match err.as_error::<AppError>() {
                        Some(app_err) => app_err.clone(),
                        None => AppError::Internal("Internal server error".to_string()),
                    };

                    let json_error = Response::error(app_error.to_string());
                    let res = HttpResponse::from_error(app_error).json(json_error);
                    
                    Ok(ServiceResponse::new(
                        req.into_parts().0,
                        res,
                    ))
                }
            }
        })
    }
} 