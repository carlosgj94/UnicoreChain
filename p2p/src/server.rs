pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port: port
        }
    }

    pub fn connect (&self) {
        println!("Server connected on: {}", self.port);
    }
}
