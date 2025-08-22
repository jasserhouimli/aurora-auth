use axum::{ extract::{ State }, http::StatusCode, response::IntoResponse, Json };
use axum_extra::extract::TypedHeader;
use headers::{ Authorization, authorization::Bearer };
use crate::shared::AppState;
use crate::service::jwt;

pub async fn secret_handler(
    State(app_state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse {
    let jwt_secret = app_state.jwt_secret.clone();
    let token = bearer.token();

    match jwt::verify_token(token, jwt_secret).await {
        Ok(claims) => (StatusCode::OK, Json(claims)).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
