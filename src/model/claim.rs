use serde::{ Serialize, Deserialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: i64,
}
