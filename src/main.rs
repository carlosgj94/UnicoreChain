extern crate unicore_p2p as p2p;
extern crate unicore_cli as cli;

use p2p::{Client, Server};
use cli::{cli as Cli};

fn main() {
    println!("Hello, world!");
    let _client = Client::new(80);
    let _server = Server::new(80);
    Cli::cli();
    _server.start();
    _client.connect();
}
