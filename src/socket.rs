use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};
use std::sync::{Arc, Mutex};

use std::time::Duration;

pub struct TCPServer {
    address: String,
    port: u16
}

impl Default for TCPServer {
    fn default() -> Self {
        Self { address: "0.0.0.0".to_string(), port: 8080 }
    }
}

fn handle_read(mut stream: TcpStream) {
    let mut buffer = [0;4096];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let received = String::from_utf8_lossy(&buffer[..size]);
            println!("Received: {}", received);
        }
        Err(e) => {
            eprintln!("Failed to read from client {}", e);
        }
    }
}

fn handle_write(mut stream: TcpStream, message: String){
    if let Err(err) = stream.write_all(message.as_bytes()) {
        println!("{0}", err);
    };
}

impl TCPServer {
    pub fn initialize_socket(&mut self, address: String, port: u16) {
        self.set_address(address);
        self.set_port(port);
    }
    fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    fn set_address(&mut self, address: String) {
        self.address = address;
    }
    pub fn get_port(&mut self) -> u16 {
        self.port
    }
    pub fn get_address(&mut self) -> String {
        self.address.clone()
    }

    pub fn start_listening(self) {
        let shared_self = Arc::new(self);
        let full_address = shared_self.address.clone() + ":" + &shared_self.port.to_string();
        let listener = TcpListener::bind(full_address).unwrap();
        let active_threads = Arc::new(Mutex::new(0));
        for stream in listener.incoming() {
            let active_requests=Arc::clone(&active_threads);
            let stream=stream.unwrap();
            thread::spawn(move || {
            {
                let mut connection=active_requests.lock().unwrap();
                *connection+=1;
                if *connection>=5{
                    thread::sleep(Duration::from_secs(2));
                }
            }
            handle_read(stream);
            let mut connection=active_requests.lock().unwrap();
            *connection-=1;
            });
        }
    }
}