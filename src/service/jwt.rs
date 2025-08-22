use axum::http::StatusCode;
use std::sync::Arc;
use jsonwebtoken::{ encode, Header, EncodingKey, DecodingKey, Validation };
use crate::model::{ Claim };
pub async fn give_token(claims: Claim, jwt_secret: Arc<String>) -> String {
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes())
    ).unwrap();
    token
}

pub async fn verify_token(token: &str, jwt_secret: Arc<String>) -> Result<Claim, StatusCode> {
    let token_data = jsonwebtoken
        ::decode::<Claim>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default()
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(token_data.claims)
}
