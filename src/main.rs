extern crate unicore_p2p as p2p;

use p2p::{Client, Server};

fn main() {
    println!("Hello, world!");
    let _client = Client::new(80);
    let _server = Server::new(80);
    _server.start();
    _client.connect();
}
