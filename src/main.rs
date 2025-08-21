use axum::{ Router };
mod route;
mod handler;
mod model;
use model::state::AppState;
use std::{ env, sync::Arc };
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().nest("/api/v1", Router::new().merge(route::get_routers()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is ready at localhost/3000");
    axum::serve(listener, app).await.unwrap();
}

fn load_state() -> AppState {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    AppState {
        jwt_secret: Arc::new(jwt_secret),
    }
}
