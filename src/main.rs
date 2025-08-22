use axum::{ Router };
mod route;
mod handler;
mod model;
mod service;
mod shared;
use std::{ env };
use crate::shared::state;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app_state = state::load_state();
    let api_path = format!("/api/{}", env::var("APIVERSION").unwrap_or("v1".to_string()));
    let app = Router::new().nest(&api_path, route::get_routers().with_state(app_state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is ready at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
