use std::future::{self, Future};
use std::pin::Pin;
use tokio::sync::mpsc;

pub type Result<T> = std::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

type Callback = dyn Fn(u32) + Send + Sync;
//type AsyncCallback = (dyn Fn(String) -> impl Future<Output=()>) + Send + Sync;
type AsyncCallback = dyn Fn(u32) -> Pin<Box<dyn Future<Output=u32> + Send>> + Send + Sync;

enum Message {
    Empty,
    SyncCallback(u32, Box<Callback>),
    AsyncCallback(u32, Box<AsyncCallback>)
}

async fn handle_messages(messages: &mut mpsc::Receiver<Message>) -> Result<()> {
    while let Some(message) = messages.recv().await {
        match message {
            Message::Empty =>
                println!("Empty message"),
            Message::SyncCallback(arg, cb) => {
                cb(arg)
            },
            Message::AsyncCallback(arg, cb) => {
                println!("Here is an async passing");
                let i: u32 = cb(arg).await;
                println!("Return is {}", i);
                ()
            }
        }
    }

    Ok(())
}

fn callback2(arg: u32) -> impl Future<Output=u32> {
    future::ready(arg + 4)
}

fn callback3(arg: u32) -> Pin<Box<dyn Future<Output=u32> + Send>> {
    Box::pin(future::ready(arg * 2))
}

async fn exec(i: u32, callback: Box<dyn Fn(u32) -> u32>) -> Result<()> {
    if callback(i) > 5 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "small".to_string())))
    }
}

type Fut<Out> = Pin<Box<dyn Future<Output=Result<Out>>>>;

fn ecb(n: u32) -> Fut<()> {
    //future::ready(Ok(()))
    Box::pin(future::ready(Ok(())))
}

async fn exec2<'a>(a: &'a str,
               b: &'a str,
               callback: Box<dyn Fn(u32) -> Fut<()>>
               ) -> Result<()> {
    callback(3).await?;
    Ok(())
}

//async fn callback4(arg: String) -> usize {
//    arg.len() * 3
//}

#[tokio::main]
pub async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        handle_messages(&mut rx).await;
    });

    println!("Callback 2 {}", callback2(4).await);
    println!("Callback 3 {}", callback3(5).await);

    let cases = vec![
        Message::Empty,

        Message::SyncCallback(7, Box::new(|n| println!("sync callback {}", n))),

        Message::AsyncCallback(8, Box::new(callback3))
    ];

    for case in cases {
        tx.send(case).await;
    }

    let r = exec(3, Box::new(|n| n * n)).await;
    println!("exec {:?}", r);

    let r2 = exec2("a", "b", Box::new(ecb)).await;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(())
}
