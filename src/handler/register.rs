use axum::{ http::StatusCode, Json };
use crate::model::jsonrep::{ RegisterResponse };
pub async fn register_handler() -> Result<Json<RegisterResponse>, StatusCode> {
    Ok(
        Json(RegisterResponse {
            status: true,
            message: "Register ".to_string(),
        })
    )
}
