extern crate tokio_postgres;

mod connection_pool {
    use tokio_postgres::{Client, NoTls};

    struct Pool {
        conns: Vec<Client>
    }

    impl Pool {
        async fn new(params: &str, init_size: usize) -> Self {
            let mut cs = Vec::new();

            for _i in 0..init_size {
                let (client, conn) = tokio_postgres::connect(params, NoTls).await.unwrap();

                conn.await.unwrap();
                cs.push(client);
            }

            Pool {
                conns: cs
            }
        }
    }
}
