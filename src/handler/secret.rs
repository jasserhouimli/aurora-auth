use axum::http::StatusCode;

pub async fn secret_handler() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::UNAUTHORIZED)
}
