mod connection_pool;

use std::fmt;

struct Connection {
    id: i32,
    log: Vec<String>
}

#[derive(Debug)]
struct Pool {
    conns: Vec<Connection>
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
         .field("id", &self.id)
         .field("log", &self.log)
         .finish()
    }
}

impl Pool {
    fn new() -> Self {
        let mut cs = Vec::new();

        for i in 0..4 {
            cs.push(Connection{id: i, log: Vec::new()});
        }

        Pool {
            conns: cs
        }
    }

    fn get(&mut self) -> Option<Connection> {
        self.conns.drain(0..1).take(1).next()
    }

    fn release(&mut self, conn: Connection) {
        self.conns.push(conn);
    }

    fn is_conn_alive(&self, id: i32) -> bool {
        self.conns.iter().any(|conn| conn.id == id)
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first() {
        let mut pool = Pool::new();
        let conn = pool.get().unwrap();

        assert_eq!(0, conn.id);
    }

    #[test]
    fn put_back() {
        let mut pool = Pool::new();
        let conn = pool.get().unwrap();
        let id = conn.id;

        pool.release(conn);
        assert!(pool.is_conn_alive(id));
    }
}
