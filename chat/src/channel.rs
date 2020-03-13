pub mod channel;

extern crate chrono;

use chrono::{DateTime};

struct Channel {
    id: i32,
    name: string,
    members: Vec<User>,
    last_message: string,
    last_modified: DateTime,
}
