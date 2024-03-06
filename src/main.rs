use tokio::*;
use tokio::time::{sleep, Duration};
use std::env;

mod NetworkAbs;
use NetworkAbs::{NetworkLayer, Orquestrator};

#[tokio::main]
async fn main() {


    let args: Vec<String> = env::args().collect();

    let port = &args[1].parse::<u16>().unwrap();

 



    let network = NetworkLayer::config("239.0.0.125",12345,"127.0.0.1",*port).await;

    let mut orq = network.run().await;

    loop {
        
        if args.len() > 2 {

            let dst = &args[2].parse::<u16>().unwrap();

            network.send("Uni port","127.0.0.1",*dst).await;
   
        
            network.send("multicast Port","239.0.0.125",12345).await;

            sleep(Duration::from_secs(3)).await;

        }

    

        //match orq.receiver_uni.try_recv() {
        //    Ok(received_message) => {
        //        println!("Uni Msg: {}", received_message);
        //    }
        //    Err(error) => {
        //        //println!("Erro Uni msg: {}", error);
        //    }
        //}
    
        match orq.receiver.recv() {
            Ok(received_message) => {
                println!("Mult Msg: {}", received_message);
            }
            Err(error) => {
                //println!("Erro Mult msg: {}", error);
            }
        }


        //sleep(Duration::from_secs(3)).await;

    }



}

