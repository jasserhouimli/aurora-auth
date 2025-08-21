use axum::http::StatusCode;
use jsonwebtoken::{ decode , DecodingKey };
pub async fn secret_handler() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::UNAUTHORIZED)
}
