extern crate threadpool;
extern crate num_cpus;

use std::io::Write;
use std::io::Read;
use std::net::TcpListener;

use threadpool::ThreadPool;

fn main() {

    // create new threadpool
    let pool = ThreadPool::new(num_cpus::get());
    println!("Creating {} threads...", pool.max_count());

    // create the listener
    let listener = TcpListener::bind("0.0.0.0:41001").expect("Failed to create TCP Listener");
    println!("Waiting for connections...");

    // accept connections
    for stream in listener.incoming() {

        // start a process in the thread
        pool.execute(|| {
            let mut stream = stream.expect("Failed to accept Connection");
            let mut buf = [0; 32];

            // read public key
            stream.read_exact(&mut buf).expect("Failed to read public key");
            println!("{:?}", buf);

            // write response
            stream.write(b"ok").expect("Failed to write response");
        });
    }
}
