use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};


fn main() {
    println!("Tententive de connexion au serveur ...");
    /*match TcpStream::connect("127.0.0.1:7878") {
        OK(_) => {
            println!("Connexion au serveur réussie !");
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }*/
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 7878;

    let tcp_s = TcpStream::connect(SocketAddrV4::new(ip, port));
    let tcp_s = TcpStream::connect((ip, port));
    let tcp_s = TcpStream::connect(("127.0.0.1", port));
    let tcp_s = TcpStream::connect(("localhost", port));
    let tcp_s = TcpStream::connect("127.0.0.1:7878");
    let tcp_s = TcpStream::connect("localhost:7878");
}