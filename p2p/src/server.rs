extern crate actix_web;
use self::actix_web::http::Method;
use self::actix_web::{
    error, server, App, Error, HttpMessage, HttpRequest, HttpResponse, Json, Result,
};
extern crate json;
extern crate serde_json;
use std::collections::HashMap;

#[derive(Serialize)]
struct Connection {
    address: String,
    alive: bool,
}

#[derive(Serialize, Deserialize)]
struct Version {
    version: String,
}

#[derive(Fail, Debug)]
#[fail(display = "Version error")]
struct VersionError {
    error: &'static str,
}
// Use default implementation for `error_response()` method
impl error::ResponseError for VersionError {}

pub struct Server {
    port: u16,
    connections: HashMap<String, Connection>,
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port: port,
            connections: HashMap::new(),
        }
    }

    pub fn start(&self) {
        println!("Server connected on: {}", self.port);
        server::new(|| {
            App::new()
                .resource("/", |r| r.method(Method::GET).with(version))
                .resource("/get-nodes", |r| r.method(Method::GET).f(get_nodes))
        }).bind("127.0.0.1:8088")
            .unwrap()
            .run();
    }
}

fn version(version: Json<Version>) -> Result<HttpResponse, VersionError> {
    if version.version == env!("CARGO_PKG_VERSION").to_string() {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(
                serde_json::to_string(&Version {
                    version: env!("CARGO_PKG_VERSION").to_string(),
                }).unwrap(),
            )
            .into())
    } else {
        Err(VersionError {
            error: "Version not compatible",
        })
    }
}

fn get_nodes(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let info = _req.connection_info();

    match info.remote() {
        Some(ref p) => println!("has value {}", p),
        None => println!("has no value"),
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(
            serde_json::to_string(&Connection {
                address: "127.0.0.1:8088".to_string(),
                alive: true,
            }).unwrap(),
        )
        .into())
}
