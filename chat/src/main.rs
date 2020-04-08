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
//use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
//use tokio_postgres::Error;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

// Functions, data structures for handling chat channels

async fn route(req: Request<Body>) -> Result<Response<Body>> {
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

async fn server() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
            println!("{:?}", req);
            route(req).await
        }))
    });

    Ok(Server::bind(&addr).serve(service).await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = server().await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn get_test() {
        let mut rt = Runtime::new().unwrap();
        rt.spawn(server());

        std::thread::sleep(std::time::Duration::from_millis(50));

        let body_fut = async {
            let req = reqwest::get("http://localhost:3000/channel").await;
            req.unwrap().text().await
        };

        let body = rt.block_on(body_fut).unwrap();

        println!("{}", body);
        // TODO unmarshal json and check it

        assert!(body.len() > 0)
    }
}
