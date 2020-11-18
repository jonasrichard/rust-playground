//use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

struct SyncCall {
    id: u32,
    data: u8,
    caller: oneshot::Sender<u32>
}

async fn server(mut receiver: mpsc::Receiver<SyncCall>) {
    loop {
        println!("[server] Waiting for messages...");

        tokio::select! {
            Some(call) = receiver.recv() => {
                println!("[server] Get id: {} data: {}", call.id, call.data);
                sleep(Duration::from_millis(500)).await;

                if let Err(e) = call.caller.send(call.id) {
                    println!("[server] Send error {:?}", e);

                    return
                }

                if call.data == 88 {
                    return
                }
                ()
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");

    let (sender, receiver) = mpsc::channel(4);

    tokio::spawn(async move {
        server(receiver).await
    });

    sleep(Duration::from_millis(1000)).await;

    println!("[client] Sending numer");

    let (call_tx, call_rx) = oneshot::channel();

    let call = SyncCall {
        id: 1000,
        data: 88,
        caller: call_tx
    };

    if let Err(e) = sender.send(call).await {
        eprintln!("[client] Error during sending {}", e);

        return Ok(())
    }

    println!("[client] Sync wait");
    call_rx.await?;


    Ok(())
}
