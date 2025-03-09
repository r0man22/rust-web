use std::env;
use dotenv::dotenv;

pub fn get_db_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}
