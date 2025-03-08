use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_index(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Gelen HTTP isteğini okumak için buffer
    stream.read(&mut buffer).unwrap(); // Veriyi okuyalım

    // Burada, gelen isteğin başını alıyoruz. Örneğin: "GET / HTTP/1.1"
    let request = String::from_utf8_lossy(&buffer[..]);
    let first_line = request.lines().next().unwrap_or("");

    // Eğer gelen istek GET / ise, index sayfası dönecek.
    if first_line.starts_with("GET / HTTP/1.1") {
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
        stream.write(response.as_bytes()).unwrap(); // Yanıtı gönderiyoruz.
        stream.flush().unwrap(); // Veriyi temizliyoruz.
    } else {
        // Eğer gelen istek farklı bir şeyse, 404 Not Found dönecek.
        let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\n\r\n404 Not Found";
        stream.write(response.as_bytes()).unwrap(); // Yanıtı gönderiyoruz.
        stream.flush().unwrap(); // Veriyi temizliyoruz.
    }
}
