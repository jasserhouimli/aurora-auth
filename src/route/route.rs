use axum::{ http::StatusCode, response::IntoResponse, routing::{ get, post }, Router };
use crate::handler::{ login_handler, register_handler, secret_handler };

pub fn get_routers() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", get(register_handler))
        .route("/secret", get(secret_handler))
}
