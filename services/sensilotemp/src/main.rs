extern crate actix_web;

use actix_web::{server, HttpRequest}



fn index(_req: &HttpRequest) -> &'static str {
    "Hello World with Rust and Actix-Web"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
