use axum::{ http::StatusCode, Json };
use crate::model::jsonrep::{ LoginRequest, LoginResponse };
use std::env;
use chrono::{ Utc };

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Claim {
    sub: String,
    exp: i64,
}
use jsonwebtoken::{ encode, Header, EncodingKey, DecodingKey };
pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<
    Json<LoginResponse>,
    StatusCode
> {
    let jwt_secret = String::from("secret");
    let claims = Claim {
        sub: payload.username.clone(),
        /// 3600 = 1 hour
        exp: Utc::now().timestamp() + 3600,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref())
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(
        Json(LoginResponse {
            status: true,
            message: format!("Hello {}", payload.username),
            token,
        })
    )
}
