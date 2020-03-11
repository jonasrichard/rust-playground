#[macro_use]
extern crate json;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    let data = object!{
        "id" => 3,
        "name" => "John Smith",
        "lang" => array!["en", "es"]
    };

    println!("{:#}", data);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
