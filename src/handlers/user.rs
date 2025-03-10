//use std::sync::Arc;
use postgres::{Client, Error};
use crate::models::user::RegisterUser;

pub fn register_user(client: &mut Client, user_data: &RegisterUser) -> Result<(), Error> {
    client.execute(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        &[&user_data.name, &user_data.email, &user_data.password],
    )?;
    Ok(())
}
