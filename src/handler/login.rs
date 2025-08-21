use axum::{ http::StatusCode, Json };
use crate::model::jsonrep::{ LoginRequest, LoginResponse };
pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<
    Json<LoginResponse>,
    StatusCode
> {
    Ok(Json(LoginResponse { status: true, message: format!("Welcome {}", payload.username) }))
}
