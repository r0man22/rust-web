use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn main () {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Unable to bind to address");
    println!("Server running on http://127.0.0.1:8080");

    for _stream in listener.incoming() {
        let _stream = _stream.expect("Failed to accept connection");

        thread::spawn(move || {

        });

        thread::sleep(Duration::from_secs(1));
    }
}
