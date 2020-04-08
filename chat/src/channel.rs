extern crate chrono;
extern crate tokio_postgres;

//use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use tokio_postgres::NoTls;

//use crate::user;

/// Represent a chat channel, it has a name and members.
/// It also caches the last message and the timestamp when it was sent.
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: i32,
    name: String,
    //members: Vec<user::User>,
    //last_message: String,
    //last_modified: DateTime<Utc>,
}

pub async fn create_channel(channel: Channel) -> Result<(), String> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=chat dbname=chat password=postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    match client.execute("INSERT INTO channel (name) values ($1)", &[&channel.name]).await {
        Ok(result) => {
            eprintln!("result {}", result);
            Ok(())
        },
        Err(e) => {
            eprintln!("error {}", e);
            Err(e.to_string())
        }
    }
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
