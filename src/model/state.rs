use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: Arc<String>,
}
