
use std::net::*; 
use tokio::*;
//nc -u 127.0.0.1 34254

fn main() {

    println!("Hello, world!");

   


    recebe_msg();
    envia_msg("Ola","127.0.0.1",50000)

   





}


fn envia_msg(msg : &str , ip : &str , port: u16 )
{   
    //Converte antes do envio
    let msg = msg.as_bytes();


    //TODO: Atualmente manda do localhost, ver como mudaremos isso
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");    
    
    
    //TODO: Aprender a checar por erros e coisas assim
    socket.send_to(msg, address(ip, port));
}

fn recebe_msg()
{
    let mut buf = [0;150];
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:34254").expect("Não foi possível criar o socket");

    loop{

        let (number_bytes, src) = socket.recv_from(&mut buf).expect("Erro"); 

        let data : &mut [u8] = &mut buf[..number_bytes];
        let string_data = String::from_utf8_lossy(data).to_string();
    
        println!("from: {} -> {:?} \n{}", src , data, string_data);

    }
}


fn address( ip : &str , port: u16) -> String
{
    return format!("{}:{}", ip, port); 
}
