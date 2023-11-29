
use std::net::*; 
use tokio::*;
use std::io;
use local_ip_address::list_afinet_netifas;
use socket2::{Domain, Socket, Type};


//nc -u 127.0.0.1 34254
//224.0.0.0 /8
//225.0.0.0 
//224.0.0.0 /4 -> 224 até 240


#[tokio::main]
async fn main() {

    let recebe_unicast_handle = tokio::spawn(recebe_unicast());
    let recebe_multicast_handle = tokio::spawn(recebe_multicast());


    println!("Pegando todas as interfaces disponíveis na máquina");
    println!("-----------------------------------------------");


    let network_interfaces = list_afinet_netifas().unwrap();

    for (name, ip) in network_interfaces.iter() {
        println!("{}:\t{:?}", name, ip);
    }

    println!("-----------------------------------------------");


    loop{

        let mut input = String::new();

        println!("Enviar mensagem?");
    
        // Read user input into the 'input' variable
        io::stdin().read_line(&mut input).expect("Failed to read line");


        //Tirando a droga do \n
        let input = input.trim();


        if input.eq("y") {  envia_msg("Ola","127.0.0.1",34254); }

        else if input.eq("q") {break;}

    }


  


   return;


   





}


fn envia_msg(msg : &str , ip : &str , port: u16 )
{   
    //Converte antes do envio
    let msg_bytes = msg.as_bytes();


    //TODO: Atualmente manda do localhost, ver como mudaremos isso
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");    
    

    println!("to: {}:{} -> {} {:?} ",ip,port,msg,msg_bytes);
    
    //TODO: Aprender a checar por erros e coisas assim
    socket.send_to(msg_bytes, address(ip, port));
}

async fn recebe_unicast()
{
    let mut buf = [0;150];
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:0").expect("Não foi possível criar o socket");

    loop{

        let (number_bytes, src) = socket.recv_from(&mut buf).expect("Erro"); 

        let data : &mut [u8] = &mut buf[..number_bytes];
        let string_data = String::from_utf8_lossy(data).to_string();
    
        println!("from: {} -> {} {:?} ", src ,string_data , data);

    }
}

async fn recebe_multicast()
{
    let mut buf = [0;150];

    //Dar uma olhada 
    let multicast_socket = UdpSocket::bind("0.0.0.0:34254").expect("Não foi possível criar o socket");

    // Sintonizando na rádio
    let multicast_group = Ipv4Addr::new(224, 0, 0, 1);

    //Segundo argumento especifica qual rede do meu computador eu vou querer usar
    multicast_socket.join_multicast_v4(&multicast_group, &Ipv4Addr::UNSPECIFIED).expect("erro");


    loop {
        let (number_bytes, src) = multicast_socket.recv_from(&mut buf).expect("erro");
        let data : &mut [u8] = &mut buf[..number_bytes];
        let string_data = String::from_utf8_lossy(data).to_string();
    
        println!("from: {} -> {} {:?} ", src ,string_data , data);
    }

}

fn address( ip : &str , port: u16) -> String
{
    return format!("{}:{}", ip, port); 
}
