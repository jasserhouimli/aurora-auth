use axum::{ http::StatusCode, Json };
use crate::model::jsonrep::{ LoginRequest, LoginResponse };

use jsonwebtoken::{ encode, Header, EncodingKey, DecodingKey };
pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<
    Json<LoginResponse>,
    StatusCode
> {
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret("secret".as_ref())
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(
        Json(LoginResponse {
            status: true,
            message: format!("Cc {}", payload.username),
            token,
        })
    )

}
