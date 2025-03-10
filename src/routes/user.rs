use std::net::TcpStream;
use std::io::prelude::*;
use crate::handlers::user::register_user;
use std::sync::Arc;
use postgres::Client;
use std::sync::Mutex;
use crate::models::user::RegisterUser;
//use serde::{Deserialize, Serialize};
use serde_json;


pub fn user_routes(mut stream: TcpStream, client: Arc<Mutex<Client>>) {
    let mut buffer = [0; 512];
    let read_bytes =  stream.read(&mut buffer).expect("Failed to read from stream");
    if read_bytes == 0 {
        println!("No data read from the stream");
        return;
    }
    let response = if buffer.starts_with(b"POST /register HTTP/1.1") {
        let body = String::from_utf8_lossy(&buffer[..]);

        let user_data: RegisterUser = serde_json::from_str(&body).expect("JSON parsing failed");
        println!("{:?}", user_data); 
        let mut client = client.lock().expect("Failed to lock the database client");

        match register_user(&mut *client, &user_data) {
            Ok(_) => "HTTP/1.1 200 OK\r\n\r\nUser registered successfully".to_string(),
            Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n".to_string(),
        }
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string()
    };

    stream.write(response.as_bytes()).expect("Failed to write to stream");
    stream.flush().expect("Failed to flush the stream");
}
