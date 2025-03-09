mod config;
mod db;
mod index;

use std::net::TcpListener;
use std::thread;
use std::time::Duration;


fn main () {
    match db::connect_to_db() {
       Ok(_client) => {
           println!("Database connecting successful.");
       },
       Err(err) => {
           eprintln!("An error occurred while connecting to the database: {}", err);
       },
    }


    let listener = TcpListener::bind("127.0.0.1:8080").expect("Unable to bind to address");
    println!("Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        thread::spawn(move || {
            index::handle_index(stream);

        });

        thread::sleep(Duration::from_secs(1));
    }
}
