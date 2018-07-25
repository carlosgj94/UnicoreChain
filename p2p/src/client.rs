pub struct Client {
    port: u16,
}

impl Client {
    pub fn new(port: u16) -> Client {
        Client {
            port: port
        }
    }

    pub fn connect (&self) {
        println!("Client connected on: {}", self.port);
    }
}
