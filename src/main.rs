use socket::TCPServer;
mod socket;

fn main() {

    let mut server = TCPServer::default();
    server.initialize_socket("127.0.0.1".to_string(), 5001);
    let _ = server.start_listening();

}
