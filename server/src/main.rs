use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 { return; }  // Connection closed
         // Print the received message
        let received_msg = String::from_utf8_lossy(&buffer[0..bytes_read]);
        println!("Received: {}", received_msg);
         // Send a response to the client
         let response = "Message received";
         stream.write(response.as_bytes()).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_client(stream);
        });
    }
}
