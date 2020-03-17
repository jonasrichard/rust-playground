extern crate chrono;
extern crate tokio_postgres;

use chrono::{DateTime, Utc};
use crate::user;

/// Represent a chat channel, it has a name and members.
/// It also caches the last message and the timestamp when it was sent.
pub struct Channel {
    id: i32,
    name: String,
    members: Vec<user::User>,
    last_message: String,
    last_modified: DateTime<Utc>,
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

pub fn find_channels() {
}

//pub fn find_channels(client: &mut tokio_postgres::Client) -> Result<Vec<Channel>, String> {
//    Err("no channels found".to_string())
//}
