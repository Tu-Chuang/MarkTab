use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Serialize)]
pub struct JsonResponse<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T: Serialize> JsonResponse<T> {
    pub fn success(data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            code: 1,
            msg: "success".to_string(),
            data: Some(data),
        })
    }

    pub fn error(msg: &str) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            code: 0,
            msg: msg.to_string(),
            data: None::<T>,
        })
    }
}

pub trait Controller {
    fn config(cfg: &mut web::ServiceConfig);
} 