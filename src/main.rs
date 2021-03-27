use std::fs;
use std::io::prelude::*;
use std::net::*;
use std::string::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]).to_string();

    stream.write(get_response(request).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_response(request: String) -> String {
    // println!("Request: {}", request);

    let content = fs::read_to_string("response.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    response
}
