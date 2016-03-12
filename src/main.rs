use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // create the listener
    let listener = TcpListener::bind("0.0.0.0:41001").expect("Failed to create TCP Listener");
    println!("Waiting for connections...");

    // accept connections
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        // spawn thread for each connection
        thread::spawn(|| handle_stream(stream));
    }
}

fn handle_stream(mut stream: TcpStream) {
    // set a timeout duration
    stream.set_read_timeout(Some(Duration::from_secs(2))).expect("Failed to set read timeout");
    stream.set_write_timeout(Some(Duration::from_secs(2))).expect("Failed to set write timeout");

    // allocate buffer
    let mut buf = [0; 32];

    // read public key
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            println!("{:?}", buf);

            // write response
            stream.write(b"ok").expect("Failed to write response");
        },
        _ => {
            stream.write(b"timeout").expect("Failed to write response");
        }
    }
}
