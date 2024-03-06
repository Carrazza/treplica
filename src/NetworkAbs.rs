use std::mem::MaybeUninit;
use std::net::{SocketAddr, IpAddr,Ipv4Addr};
use std::str::FromStr;
use socket2::*;
use tokio::task::JoinHandle;
use std::io::{self, ErrorKind};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use std::net::*; 
use tokio::*;
use local_ip_address::list_afinet_netifas;
use socket2::{Domain, Socket, Type};
use std::sync::mpsc;




#[derive(Clone)]
pub struct NetworkLayer {

    mult_group: Ipv4Addr,
    mult_port: u16,
    mult_address: SocketAddr,


    local_ip: Ipv4Addr,
    local_port: u16,
    local_address: SocketAddr,

}


pub struct Orquestrator {

    pub recebe_mult_handle: JoinHandle<()>,
    pub recebe_uni_handle: JoinHandle<()>,
    pub receiver: mpsc::Receiver<String>

}


fn address( ip : &str , port: u16) -> String
{
    return format!("{}:{}", ip, port); 
}


// Retorna valor ipv4
fn ip_from_str(ip: &str) -> Ipv4Addr {

    return Ipv4Addr::from_str(ip).unwrap()

}







pub async fn recebe_unicast(net_layer : NetworkLayer , sender : mpsc::Sender<String>)
{
    //let local_sock: SocketAddr = "127.0.0.1:30000".parse().unwrap();

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    socket.set_reuse_address(true).unwrap();
    socket.bind(&net_layer.local_address.into()).unwrap();


    loop {
        let mut buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };

    

        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                
                let u8_buf: &mut [u8];

                unsafe {
                    u8_buf = &mut *(buf.as_mut_ptr() as *mut [u8; 1024]);
                    
                }

                let message = String::from_utf8_lossy(&u8_buf[..size]);
                //println!("Received message from {:?}: {}", src, message);

                sender.send(message.to_string()).unwrap();
            
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => eprintln!("Error receiving message: {}", e),
        }


    }


}
    

pub async fn recebe_multicast(net_layer : NetworkLayer ,  sender : mpsc::Sender<String>)
{
//let teste: SocketAddr = "239.0.0.125:12345".parse().unwrap();


    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    socket.set_reuse_address(true).unwrap();
    socket.bind(&net_layer.mult_address.into()).unwrap();

    socket.join_multicast_v4(&net_layer.mult_group,&Ipv4Addr::UNSPECIFIED).unwrap();

    loop {
        let mut buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };

    

        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                
                let u8_buf: &mut [u8];

                unsafe {
                    u8_buf = &mut *(buf.as_mut_ptr() as *mut [u8; 1024]);
                    
                }

                let message = String::from_utf8_lossy(&u8_buf[..size]);
                //println!("Received message from {:?}: {}", src, message);
                sender.send(message.to_string()).unwrap();
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => eprintln!("Error receiving message: {}", e),
        }


    }
}






impl NetworkLayer {

    pub async fn config (mult_group: &str , mult_port : u16, local_ip : &str , local_port:u16) -> NetworkLayer
    {



        return NetworkLayer {

            mult_group : ip_from_str(mult_group),
            mult_address : address(mult_group, mult_port).parse().unwrap(),
            mult_port : mult_port,

            local_ip : ip_from_str(local_ip),
            local_port : local_port,
            local_address : address(local_ip, local_port).parse().unwrap(),

            

    }
   
    }



    pub async fn run(&self) -> Orquestrator
    {
    
        let (sender,  receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel::<String>();

        let recebe_multicast_handle = tokio::spawn(recebe_multicast(self.clone(),sender.clone()));
        
        let recebe_unicast_handle = tokio::spawn(recebe_unicast(self.clone() , sender.clone()));
        

        
        Orquestrator{
            recebe_mult_handle : recebe_multicast_handle,
            recebe_uni_handle : recebe_unicast_handle ,
            receiver : receiver
        }

        
        
    }


    pub async fn send(&self,msg : &str , ip : &str , port: u16 )
    {   
      
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        socket.join_multicast_v4(&self.mult_group,&Ipv4Addr::UNSPECIFIED).unwrap();

        
        let dst = SocketAddrV4::new(ip_from_str(ip),port);

        println!("Sending {:?} -> {}", dst, msg);

        socket.send_to(msg.as_bytes(), &dst.into()).unwrap();

    }

  
}



