use axum::{ routing::{ get, post }, Router };
use crate::{ handler::{ login_handler, register_handler, secret_handler } };
use crate::shared::AppState;

pub fn get_routers() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", get(register_handler))
        .route("/secret", get(secret_handler))
}
