extern crate chrono;
//#[macro_use]
//extern crate json;
extern crate tokio_postgres;

mod channel;
mod user;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use tokio_postgres::{Error, NoTls};

// Functions, data structures for handling chat channels

async fn route(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") =>
            Ok(Response::new("ping".into())),
        (&Method::GET, "/channel") => {
            let channels = channel::find_channels().await;
            Ok(Response::new(channels.len().to_string().into()))
        },
        (_, _) => {
            println!("{}", req.uri().path());
            println!("{}", req.method());
            Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
//    let data = object!{
//        "id" => 3,
//        "name" => "John Smith",
//        "lang" => array!["en", "es"]
//    };
//
//    println!("{:#}", data);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
            println!("{:?}", req);
            route(req).await
        }))
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
