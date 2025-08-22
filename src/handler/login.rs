use axum::{ extract::State, http::StatusCode, Json };
use crate::{ model::{ jsonrep::{ LoginRequest, LoginResponse }, Claim } };
use crate::shared::AppState;
use chrono::{ Utc, Duration };
use axum_extra::extract::cookie::{ Cookie, CookieJar, SameSite };
use crate::service::{ jwt, auth };
use axum::response::IntoResponse;

pub async fn login_handler(
    jar: CookieJar,
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>
) -> Result<impl IntoResponse, StatusCode> {
    let token = jwt::give_token(
        Claim {
            sub: payload.username.clone(),
            exp: (Utc::now() + Duration::hours(1)).timestamp(),
        },
        app_state.jwt_secret.clone()
    ).await;

    let _jar = auth::set_auth_cookie("auth_token".into(), jar, token.clone());

    Ok((
        _jar,
        Json(LoginResponse {
            status: true,
            message: format!("Hello {}", payload.username),
            token,
        }),
    ))
}
