extern crate actix_web;
use self::actix_web::{server, App, HttpRequest, HttpResponse, Result, Error};
use self::actix_web::http::{Method};
extern crate json;
extern crate serde_json;

#[derive(Serialize)]
struct Connections {
    address: String,
    alive: bool,
}

#[derive(Serialize)]
struct Version {
    version: String
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
            .resource("/", |r| r.method(Method::GET).f(version))
            .resource("/get-nodes", |r| r.method(Method::GET).f(get_nodes)))
            .bind("127.0.0.1:8088")
            .unwrap()
            .run();
    }
}

fn version(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&Version {
            version: env!("CARGO_PKG_VERSION").to_string()
        }).unwrap()).into())
}

fn get_nodes(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&Connections {
            address: "127.0.0.1:8088".to_string(),
            alive: true,
        }).unwrap()).into())
}
