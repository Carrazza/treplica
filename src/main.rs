use tokio::*;
use tokio::time::{sleep, Duration};



mod NetworkAbs;
use NetworkAbs::{NetworkLayer, Orquestrator};

#[tokio::main]
async fn main() {


    let network = NetworkLayer::config("239.0.0.125",12345,"127.0.0.1",15001).await;

    let mut orq = network.run().await;

    tokio::spawn(async move {
        while let Some(message) = orq.receiver_uni.recv().await {
            println!("Received: {}", message);
        }
    });


    loop {
        
        network.send("Uni port","127.0.0.1",15000).await;
        //network.send("multicast Port","239.0.0.125",12345).await;
        sleep(Duration::from_secs(5)).await;

      

    }



}

