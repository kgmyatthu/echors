extern crate rand;

fn handle_client(mut stream: std::net::TcpStream) -> Result<(), std::io::Error> {
    use std::io::{Read, Write};
    

    let mut buf = [0; 1024];

    loop {
        let length = stream.read(&mut buf).expect("Couldn't read from stream");
        if length == 0 { return Ok(()) }
        stream.write(&buf[..length]).expect("Couldn't write to stream");
    }
}

fn main() {
    let listener = std::net::TcpListener::bind("0.0.0.0:1337")
                    .expect("Could not bind to address");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
