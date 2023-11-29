use std::mem::MaybeUninit;
use std::net::{SocketAddr, IpAddr,Ipv4Addr};
use std::str::FromStr;
use socket2::*;
use std::io::{self, ErrorKind};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};

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
async fn main() -> io::Result<()> {



    let local_sock: SocketAddr = "127.0.0.1:30000".parse().unwrap();
    let teste: SocketAddr = "239.0.0.125:12345".parse().unwrap();

   
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    socket.set_reuse_address(true)?;
    socket.bind(&teste.into())?;

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
