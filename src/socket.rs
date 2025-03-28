use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

pub struct Socket {
    address: String,
    port: u16
    
}

impl Default for Socket {
    fn default() -> Self {
        Self { address: "localhost".to_string(), port: 8080 }
    }
}

impl Socket {
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
}

pub struct TCPServer {
    address: String,
    port: u16
}

impl Default for TCPServer {
    fn default() -> Self {
        Self { address: "0.0.0.0".to_string(), port: 8080 }
    }
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

    fn handle_client(&self, mut stream: TcpStream) {
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

    fn write_to_client(&self, mut stream: TcpStream, message: String){
        if let Err(err) = stream.write_all(message.as_bytes()) {
            println!("{0}", err);
        };
    }

    pub fn start_listening(self)-> std::io::Result<()> {
        let full_address = self.address.clone() + ":" + &self.port.to_string();
        println!("{full_address}");
        let listener = TcpListener::bind(full_address)?;
        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }
}