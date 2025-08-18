use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::{ApiError, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub role: String,
}

pub struct TokenValidator {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl TokenValidator {
    pub fn new(secret: String) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn create_token(&self, user_id: &str, role: &str) -> Result<String, ApiError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
            role: role.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ApiError::Internal(format!("Token creation failed: {}", e)))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, ApiError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| ApiError::Unauthorized)
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Skip auth for health and docs endpoints
    let path = request.uri().path();
    if path == "/health" || path.starts_with("/swagger-ui") || path.starts_with("/api-docs") {
        return Ok(next.run(request).await);
    }

    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized)?;

    // Validate token
    let claims = state.token_validator.validate_token(token)?;

    // Insert claims into request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation_and_validation() {
        let validator = TokenValidator::new("test-secret".to_string());
        
        let token = validator.create_token("user123", "admin").unwrap();
        let claims = validator.validate_token(&token).unwrap();
        
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, "admin");
    }
}