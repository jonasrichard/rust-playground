extern crate chrono;
extern crate tokio_postgres;

use chrono::{DateTime, Utc};
use tokio_postgres::{Error, NoTls};

use crate::user;

/// Represent a chat channel, it has a name and members.
/// It also caches the last message and the timestamp when it was sent.
pub struct Channel {
    id: i32,
    name: String,
    //members: Vec<user::User>,
    //last_message: String,
    //last_modified: DateTime<Utc>,
}

pub async fn create_channel(client: &mut tokio_postgres::Client, name: String, _members: Vec<i32>) -> Result<Channel, String> {
    match client.execute("INSERT INTO channel (name) values ($1)", &[&name]).await {
        Ok(result) => {
            println!("result {}", result);
            ()
        },
        Err(_e) =>
            ()
    }
    Err("x".to_string())
}

pub async fn find_channels() -> Vec<Channel> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=chat dbname=chat password=postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let rows = client.query("select id, name from channel", &[]).await.unwrap();
    let mut result = Vec::new();

    for row in rows {
        result.push(Channel{
            id: row.get(0),
            name: row.get(1)
        });
    }

    result
}

//pub fn find_channels(client: &mut tokio_postgres::Client) -> Result<Vec<Channel>, String> {
//    Err("no channels found".to_string())
//}
