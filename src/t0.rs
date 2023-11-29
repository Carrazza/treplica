
use std::net::*; 
//nc -u 127.0.0.1 34254


struct Node {
    ip:Ipv4Addr , 
    socket:UdpSocket ,
    port:u16,
    multicast:(Ipv4Addr,u16),
}



fn create_socket(ip:String,multicast:(String,u16)) -> Node {
    

    let ipv4 = ip.parse();

    match ipv4 {
        Ok(ip) => {
            println!("Parsed IPv4 Address: {:?}", ip);
        }
        Err(err) => {
            println!("Error parsing IPv4 address: {}", err);
        }
    }

    //Porta 0 indica que o sistema operacional vai passar uma porta disponível
    let address = SocketAddr::new(ip, 0);

    let port = address.port();

    let socket = UdpSocket::bind(address).expect("Não foi possível criar o socket");


    let new_node = Node {
        ip : ip,
        socket : socket,
        port : port,
        multicast : multicast
        
    };

    return new_node ;

}

fn main() {

    println!("Hello, world!");

    //let socket: UdpSocket = UdpSocket::bind("127.0.0.1:34254").expect("Não foi possível criar o socket");

    let new_node = create_node("127.0.0.1", ("127.0.0.1",8585));

    println!("{:?}",new_node);

    let mut buf = [0;1024];

    let (number_bytes, src) = socket.recv_from(&mut buf).expect("Erro"); 

    let data : &mut [u8] = &mut buf[..number_bytes];
    let string_data = String::from_utf8_lossy(data).to_string();

    println!("{:?} \n{}", data, string_data);
}
