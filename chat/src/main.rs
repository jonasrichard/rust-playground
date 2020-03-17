extern crate chrono;
//#[macro_use]
//extern crate json;
extern crate tokio_postgres;

mod channel;
mod user;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use tokio_postgres::{Error, NoTls};

// Functions, data structures for handling chat channels

async fn route(req: Request<Body>, _client: &mut tokio_postgres::Client) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") =>
            Ok(Response::new("ping".into())),
        (&Method::GET, "/channel") => {
            channel::find_channels();
            Ok(Response::new("to be implemented".into()))
        },
        (_, _) =>
            Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap())
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

    let (mut client, connection) =
        tokio_postgres::connect("host=localhost user=chat dbname=chat password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

//    let rows = client
//        .query("SELECT id, name FROM channel", &[])
//        .await?;
//
//    println!("{:?}", rows);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = make_service_fn(|_c| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
            route(req, &mut client).await
        }))
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
