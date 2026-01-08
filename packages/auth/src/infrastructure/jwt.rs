use crate::domain::entity::User;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rust_reborn_contracts::{AppError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
    pub refresh_expiration_days: i64,
}

pub struct JwtService {
    config: JwtConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.secret.as_bytes());

        Self {
            config,
            encoding_key,
            decoding_key,
        }
    }

    pub fn generate_token(&self, user: &User) -> Result<String> {
        let user_id = user.id.ok_or_else(|| {
            AppError::internal("cannot generate token: user.id is None")
        })?;
        
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::hours(self.config.expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email: user.email.value().to_string(),
            username: user.username.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::internal(format!("failed to generate token: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> Result<i64> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| AppError::unauthorized(format!("invalid token: {}", e)))?;

        let user_id = token_data.claims.sub.parse::<i64>()
            .map_err(|e| AppError::internal(format!("invalid user ID in token: {}", e)))?;

        Ok(user_id)
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| AppError::unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }
}
