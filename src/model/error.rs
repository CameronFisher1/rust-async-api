use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorRes {
    pub error: String,
}