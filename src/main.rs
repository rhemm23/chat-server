use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::thread;

fn handle_client(mut stream: TcpStream) {
  stream.write(b"Hello!").unwrap();
}

fn main() {
  match TcpListener::bind("0.0.0.0:3333") {
    Ok(listener) => {
      for stream in listener.incoming() {
        match stream {
          Ok(stream) => {
            thread::spawn(move || {
              handle_client(stream)
            });
          }
          Err(e) => {
            println!("Error: {}", e);
          }
        }
      }
      drop(listener);
    }
    Err(e) => {
      println!("Unable to start listener socket: {}", e);
    }
  }
}
