use rustache::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::*;
use std::string::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let s = stream.unwrap();

        pool.execute(|| {
            handle_connection(s);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let http_get = b"GET / HTTP/1.1\r\n";
    let request = String::from_utf8_lossy(&buffer[..]).to_string();
    println!("Request Length: {}", request.len());

    if buffer.starts_with(http_get) {}

    let response = match buffer.starts_with(http_get) {
        true => make_get_response(request),
        _ => make_post_response(request),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn make_post_response(request: String) -> String {
    let content = request;
    let response = format!(
        "HTTP/1.1 201 OK-POST\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    println!("POST Response: {}", response);
    response
}

fn make_get_response(request: String) -> String {
    println!("Request: {}", request);

    let content = fs::read_to_string("response.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK-GET\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    response
}
