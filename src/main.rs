use socket::{Socket, TCPServer};
mod socket;

fn main() {

    let server = TCPServer::default();
    let _ = server.start_listening();

}
