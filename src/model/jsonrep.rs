use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub status: bool,
    pub message: String,
}
