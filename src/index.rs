use std::io::{Read, Write};
use std::net::TcpStream;

const BUFFER_SIZE: usize = 1024;

pub fn handle_index(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            let first_line = request.lines().next().unwrap_or("");
            println!("Incoming request: {}", first_line);
          
            let response = if first_line.contains("GET / HTTP/1.1") {
                "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!"
            } else if first_line.contains("HEAD / HTTP/1.1") {
                "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\n"
            } else {
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\n\r\n404 Not Found"
            };
            
            if let Err(e) = stream.write(response.as_bytes()) {
                eprintln!("Error sending reply: {}", e);
            }
            
            if let Err(e) = stream.flush() {
                eprintln!("Error clearing data: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading data: {}", e);
        }
    }
}
