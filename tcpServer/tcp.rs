// importing neccessary modules form the Rust lberaaries
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // the line reads data from the stream and stored it in the buffer
    stream
        .read(&mut buffer)
        .expect("Failed to ready from client");
    // this line converts the data in the buffer into a utf encoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

// entry poinst
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listining on 127.0.0.1:8080");
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e)=>{
                println!("Failed to establish connection: {}",e);
                // stderror -standard error stream
            }
        }
    }
}
