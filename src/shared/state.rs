use std::{ env, sync::Arc };

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: Arc<String>,
}

pub fn load_state() -> AppState {
    let jwt_secret = Arc::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set"));
    AppState { jwt_secret }
}
