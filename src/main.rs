use tokio::*;
use tokio::time::{sleep, Duration};



mod NetworkAbs;
use NetworkAbs::NetworkLayer;

#[tokio::main]
async fn main() {


    let network = NetworkLayer::config("239.0.0.125",12345,"127.0.0.1",15000).await;

    network.run().await;

    loop {
        
        network.send("ola","127.0.0.1",15000).await;
        sleep(Duration::from_secs(5)).await;

    }



}

