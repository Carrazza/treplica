use std::mem::MaybeUninit;
use std::net::{SocketAddr, IpAddr,Ipv4Addr};
use std::str::FromStr;
use socket2::*;
use std::io::{self, ErrorKind};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use std::net::*; 
use tokio::*;
use local_ip_address::list_afinet_netifas;
use socket2::{Domain, Socket, Type};

const MULTICAST_GROUP: &str = "239.0.0.125";
const PORT: u16 = 12345;

fn ip_from_str(ip: &str) -> Ipv4Addr {

    return Ipv4Addr::from_str(ip).unwrap()

}

fn address( ip : &str , port: u16) -> String
{
    return format!("{}:{}", ip, port); 
}


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


async fn recebe_multicast()
{
    let teste: SocketAddr = "239.0.0.125:12345".parse().unwrap();

   
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    //socket.set_reuse_address(true)?;
    socket.bind(&teste.into()).unwrap();

    socket.join_multicast_v4(&ip_from_str(MULTICAST_GROUP),&Ipv4Addr::UNSPECIFIED).unwrap();

    loop {
        let mut buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };

    

        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                
                let u8_buf: &mut [u8];

                unsafe {
                    u8_buf = &mut *(buf.as_mut_ptr() as *mut [u8; 1024]);
                    
                }

                let message = String::from_utf8_lossy(&u8_buf[..size]);
                println!("Received message from {:?}: {}", src, message);
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => eprintln!("Error receiving message: {}", e),
        }


    }
}

async fn recebe_unicast()
{
    let local_sock: SocketAddr = "127.0.0.1:30000".parse().unwrap();

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    //socket.set_reuse_address(true)?;
    socket.bind(&local_sock.into()).unwrap();

    socket.join_multicast_v4(&ip_from_str(MULTICAST_GROUP),&Ipv4Addr::UNSPECIFIED).unwrap();

    loop {
        let mut buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };

    

        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                
                let u8_buf: &mut [u8];

                unsafe {
                    u8_buf = &mut *(buf.as_mut_ptr() as *mut [u8; 1024]);
                    
                }

                let message = String::from_utf8_lossy(&u8_buf[..size]);
                println!("Received message from {:?}: {}", src, message);
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => eprintln!("Error receiving message: {}", e),
        }


    }


}