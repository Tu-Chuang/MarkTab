pub mod crypto;
pub mod jwt;
pub mod validator;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 1,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            code: 0,
            msg: msg.into(),
            data: None,
        }
    }
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
} 