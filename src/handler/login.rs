use axum::{ extract::State, http::StatusCode, Json };
use crate::model::{ jsonrep::{ LoginRequest, LoginResponse } };
use crate::shared::AppState;
use chrono::{ Utc };
use crate::model::{ Claim };

use crate::service::jwt;

pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    let token = jwt::give_token(
        Claim {
            sub: payload.username.clone(),
            exp: Utc::now().timestamp() + 3600,
        },
        app_state.jwt_secret.clone()
    ).await;
    Ok(
        Json(LoginResponse {
            status: true,
            message: format!("Hello {}", payload.username),
            token,
        })
    )
}
