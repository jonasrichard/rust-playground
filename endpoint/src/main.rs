use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
struct Context {
    ch: mpsc::Sender<String>,
}

async fn route(req: Request<Body>, context: Context) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hey".into()))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let http_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let (tx, _) = mpsc::channel(1);
    let context = Context {
        ch: tx,
    };

    let make_svc = make_service_fn(move |_conn| {
        let context = context.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |req| route(req, context.clone())))
        }
    });

    let server = Server::bind(&http_addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("HTTP error {}", e);
    }

    Ok(())
}
