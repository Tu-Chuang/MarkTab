use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use crate::{
    error::{AppError, AppResult},
    models::{user::User, token::Token},
    config::Config,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,  // user_id
    exp: i64,  // expiration time
}

pub struct AuthService;

impl AuthService {
    pub async fn login(
        pool: &MySqlPool,
        email: &str,
        password: &str,
        user_agent: &str,
        ip: &str,
    ) -> AppResult<Token> {
        // 查找用户
        let user = User::find_by_email(pool, email)
            .await?
            .ok_or_else(|| AppError::Auth("User not found".to_string()))?;

        // 验证密码
        if !bcrypt::verify(password, &user.password)? {
            return Err(AppError::Auth("Invalid password".to_string()));
        }

        // 生成token
        let config = Config::from_env()?;
        let exp = Utc::now() + Duration::days(7);
        let claims = Claims {
            sub: user.id,
            exp: exp.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )?;

        // 保存token记录
        Token::create(
            pool,
            user.id,
            &token,
            user_agent,
            ip,
            exp,
        ).await
    }

    pub async fn verify_token(
        pool: &MySqlPool,
        token: &str,
    ) -> AppResult<User> {
        // 验证token是否有效
        let token_record = Token::find_by_token(pool, token)
            .await?
            .ok_or_else(|| AppError::Auth("Invalid token".to_string()))?;

        // 获取用户信息
        let user = User::find_by_id(pool, token_record.user_id)
            .await?
            .ok_or_else(|| AppError::Auth("User not found".to_string()))?;

        Ok(user)
    }
} 