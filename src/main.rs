mod config;
mod db;
mod index;
mod routes;
mod handlers;
mod models;

use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let client = match db::connect_to_db() {
       Ok(client) => {
           println!("Database connecting successful.");
           Arc::new(Mutex::new(client)) // ✅ `client` burada atanıyor
       },
       Err(err) => {
           eprintln!("An error occurred while connecting to the database: {}", err);
           return; // ❗ Programı durduruyoruz ki `client` olmadan ilerlemesin
       },
    };

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Unable to bind to address");
    println!("Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        let stream = Arc::new(Mutex::new(stream));
        let client = Arc::clone(&client); // `client`'i paylaşmaya devam ediyoruz

        thread::spawn(move || {
            // `stream`'i önce `lock` ile alıyoruz
            let stream = stream.lock().unwrap();
            // `stream` artık `TcpStream` türünde
            index::handle_index(stream.try_clone().unwrap()); // `try_clone` ile yeni bir kopya alıyoruz
            routes::user::user_routes(stream.try_clone().unwrap(), client); // Aynı şekilde burada da `try_clone` kullanıyoruz
        });

        thread::sleep(Duration::from_secs(1));
    }
}
