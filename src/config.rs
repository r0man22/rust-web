use std::env;
use dotenv::dotenv;

pub fn get_db_url() -> String {
    if  dotenv().is_err() {
        println!("Error loading .env file");
    }
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}
