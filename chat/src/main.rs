extern crate chrono;
#[macro_use]
extern crate json;
extern crate tokio_postgres;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio_postgres::{Error, NoTls};

pub mod channel {

    use chrono::{DateTime, Utc};

    pub struct Channel {
        id: i32,
        name: String,
        members: Vec<User>,
        last_message: String,
        last_modified: DateTime<Utc>,
    }

    pub struct User {
        id: i32,
        name: String,
    }

    // TODO type alias i32 to user id
//    pub fn create_channel(client: &mut postgres::Client, name: String, _members: Vec<i32>) -> Result<Channel, String> {
//        match client.execute("INSERT INTO channel (name) values ($1)", &[&name]) {
//            Ok(result) => {
//                println!("result {}", result);
//                ()
//            },
//            Err(e) =>
//                ()
//        }
//        Err("x".to_string())
//    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let data = object!{
        "id" => 3,
        "name" => "John Smith",
        "lang" => array!["en", "es"]
    };

    println!("{:#}", data);

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=chat dbname=chat password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT id, name FROM channel", &[])
        .await?;

    println!("{:?}", rows);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = make_service_fn(|_client| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
