use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let msg = "Hello, server!";
    stream.write(msg.as_bytes()).unwrap();

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let received = String::from_utf8_lossy(&buffer[..]);
    println!("Received: {}", received);
}
