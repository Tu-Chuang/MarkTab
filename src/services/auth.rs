use crate::{models::{user::User, token::Token}, error::AppError};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,  // user_id
    pub exp: i64,  // expiration time
}

pub struct AuthService;

impl AuthService {
    pub async fn login(
        pool: &MySqlPool,
        email: &str,
        password: &str,
        user_agent: &str,
        ip: &str,
    ) -> Result<Token, AppError> {
        let user = User::find_by_email(pool, email)
            .await?
            .ok_or_else(|| AppError::Auth("Invalid credentials".into()))?;

        if !bcrypt::verify(password, &user.password)? {
            return Err(AppError::Auth("Invalid credentials".into()));
        }

        let claims = Claims {
            sub: user.id,
            exp: (Utc::now() + Duration::days(7)).timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes()),
        )?;

        Token::create(pool, user.id, &token, user_agent, ip).await
            .map_err(AppError::Database)
    }

    pub async fn verify_token(
        pool: &MySqlPool,
        token: &str,
    ) -> Result<User, AppError> {
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes()),
            &Validation::default(),
        )?;

        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = ?",
            claims.claims.sub
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Auth("User not found".into()))?;

        Ok(user)
    }
} 