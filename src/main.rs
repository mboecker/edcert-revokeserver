use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    // create the listener
    let listener = TcpListener::bind("0.0.0.0:41001").expect("Failed to create TCP Listener");
    println!("Waiting for connections...");

    // accept connections
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        thread::spawn(|| handle_stream(stream));
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 32];

    // read public key
    stream.read_exact(&mut buf).expect("Failed to read public key");
    println!("{:?}", buf);

    // write response
    stream.write(b"ok").expect("Failed to write response");
}
