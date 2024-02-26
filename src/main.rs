use server::Server;

mod server;

fn main() {
    let mut server = Server::new();
    server.run();
}
