extern crate chrono;
extern crate tokio_postgres;

mod channel;
mod user;

use crate::channel::Channel;

use std::convert::Infallible;
use std::net::SocketAddr;

//use futures::TryStreamExt;
use hyper::body;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use tokio_postgres::Error;

// Functions, data structures for handling chat channels

async fn route(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") =>
            Ok(Response::new("ping".into())),
        (&Method::GET, "/channel") => {
            let channels = channel::find_channels().await;
            let channels_json = serde_json::to_string(&channels).unwrap();
            Ok(Response::new(channels_json.into()))
        },
        (&Method::POST, "/channel") => {
            let bytes = body::to_bytes(req.into_body()).await.unwrap();
            let json: Channel = serde_json::from_slice(&bytes).unwrap();
            match channel::create_channel(json).await {
                Ok(_) =>
                    Ok(Response::new("Success".into())),
                Err(e) =>
                    Ok(Response::builder().status(StatusCode::NOT_FOUND).body(e.into()).unwrap())
            }
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
