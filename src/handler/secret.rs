use axum::{ extract::{ State }, http::StatusCode, response::IntoResponse, Json };
// use axum_extra::extract::TypedHeader;
// use headers::{ Authorization, authorization::Bearer };
use crate::shared::AppState;
use crate::service::jwt;
use axum_extra::extract::cookie::CookieJar;

pub async fn secret_handler(
    jar: CookieJar,
    State(app_state): State<AppState>
) -> impl IntoResponse {
    let jwt_secret = app_state.jwt_secret.clone();
    if let Some(cookie) = jar.get("auth_token") {
        let token = cookie.value();
        match jwt::verify_token(token, jwt_secret).await {
            Ok(claims) => {
                return (StatusCode::OK, Json(claims)).into_response();
            }
            Err(_) => {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        }
    }

    StatusCode::UNAUTHORIZED.into_response()
}
