extern crate actix_web;
use self::actix_web::{server, App, HttpRequest, HttpResponse, Result, Error};
use self::actix_web::http::{Method};
extern crate json;
extern crate serde_json;

#[derive(Serialize)]
struct Connections {
    address: String,
    port: u16,
}

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port: port
        }
    }

    pub fn start(&self) {
        println!("Server connected on: {}", self.port);
        server::new(|| App::new()
            .resource("/", |r| r.method(Method::GET).f(running)))
            .bind("127.0.0.1:8088")
            .unwrap()
            .run();
    }
}

fn running(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&Connections {
            address: "127.0.0.1".to_string(),
            port: 8088,
        }).unwrap()).into())
}
