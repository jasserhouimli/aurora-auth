use axum::{ Router };
mod route;
mod handler;
mod model;
mod service;
use model::{ AppState };
use std::{ env, sync::Arc };
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app_state = AppState {
        jwt_secret: Arc::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set")),
    };
    let app = Router::new().nest("/api/v1", route::get_routers().with_state(app_state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is ready at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
