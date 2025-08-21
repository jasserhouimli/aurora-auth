use axum::{ extract::Path, http::StatusCode };
use jsonwebtoken::{ decode, DecodingKey };

pub async fn secret_handler(Path(id): Path<String>) -> Result<String, StatusCode> {
    Ok(id.to_string())
}
