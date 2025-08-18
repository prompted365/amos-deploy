use axum::{
    extract::State,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use crate::{ApiResult, AppState, ApiError};
use utoipa::ToSchema;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: i64,
    pub user_id: String,
    pub role: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
    ),
    tag = "auth",
)]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    // In production, verify credentials against a user database
    // For now, accept a test user
    if request.username == "admin" && request.password == "amos123" {
        let token = state.token_validator.create_token("admin-user-id", "admin")?;
        
        Ok(Json(LoginResponse {
            token,
            expires_in: 86400, // 24 hours
            user_id: "admin-user-id".to_string(),
            role: "admin".to_string(),
        }))
    } else {
        Err(ApiError::Unauthorized)
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponse),
        (status = 401, description = "Invalid token"),
    ),
    tag = "auth",
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(request): Json<RefreshRequest>,
) -> ApiResult<Json<LoginResponse>> {
    // Validate the existing token
    let claims = state.token_validator.validate_token(&request.token)?;
    
    // Create a new token with the same claims
    let new_token = state.token_validator.create_token(&claims.sub, &claims.role)?;
    
    Ok(Json(LoginResponse {
        token: new_token,
        expires_in: 86400, // 24 hours
        user_id: claims.sub,
        role: claims.role,
    }))
}