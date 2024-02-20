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
use tokio::sync::mpsc;


pub struct NetworkLayer {

    mult_group: Ipv4Addr,
    mult_port: u16,
    mult_address: SocketAddr,


    local_ip: Ipv4Addr,
    local_port: u16,
    local_address: SocketAddr,

    sender : tokio::sync::mpsc::Sender<String>,
    receiver : tokio::sync::mpsc::Receiver<String>


}


fn address( ip : &str , port: u16) -> String
{
    return format!("{}:{}", ip, port); 
}

fn ip_from_str(ip: &str) -> Ipv4Addr {

    return Ipv4Addr::from_str(ip).unwrap()

}

async fn recebe_multicast(mult_address : SocketAddr, mult_group: Ipv4Addr)
{
//let teste: SocketAddr = "239.0.0.125:12345".parse().unwrap();


    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    socket.set_reuse_address(true).unwrap();
    socket.bind(&mult_address.into()).unwrap();

    socket.join_multicast_v4(&mult_group,&Ipv4Addr::UNSPECIFIED).unwrap();

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








impl NetworkLayer {

    pub async fn config (mult_group: &str , mult_port : u16, local_ip : &str , local_port:u16) -> Self
    {

        let (sender, mut receiver) = mpsc::channel::<String>(32);


        return NetworkLayer {

            mult_group : ip_from_str(mult_group),
            mult_address : address(mult_group, mult_port).parse().unwrap(),
            mult_port : mult_port,

            local_ip : ip_from_str(local_ip),
            local_port : local_port,
            local_address : address(local_ip, local_port).parse().unwrap(),
            receiver : receiver ,
            sender : sender
            

        }
       
    }
    pub async fn recebe_unicast(local_address : SocketAddr )
    {
        //let local_sock: SocketAddr = "127.0.0.1:30000".parse().unwrap();
    
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        socket.set_reuse_address(true).unwrap();
        socket.bind(&local_address.into()).unwrap();
    
    
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
  
    pub async fn run(&self)
    {
        let recebe_multicast_handle = tokio::spawn(recebe_multicast(self.mult_address , self.mult_group));
        let recebe_unicast_handle = tokio::spawn(NetworkLayer::recebe_unicast(self.local_address));

    
    }


    pub async fn send(&self,msg : &str , ip : &str , port: u16 )
    {   
        //Converte antes do envio
        let msg_bytes = msg.as_bytes();

        let bind_ip = address(self.local_ip.to_string().as_str(), 0);


        //TODO: Atualmente manda do localhost, ver como mudaremos isso
        let socket = UdpSocket::bind(bind_ip).await.expect("couldn't bind to address");    
        

        println!("to: {}:{} -> {} {:?} ",ip,port,msg,msg_bytes);
        
        //TODO: Aprender a checar por erros e coisas assim
        socket.send_to(msg_bytes, address(ip, port)).await.unwrap();
    }
}

