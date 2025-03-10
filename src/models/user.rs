use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

