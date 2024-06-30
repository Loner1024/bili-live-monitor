use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

#[tokio::main]
async fn main() {
    let mut rx = return_receiver();

    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }
}

fn return_receiver() -> Receiver<i32> {
    let (tx, rx) = mpsc::channel(1024);
    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = tx.send(i).await {
                println!("receiver dropped");
                return;
            }
        }
    });
    rx
}
