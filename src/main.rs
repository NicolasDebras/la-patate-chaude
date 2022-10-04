use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};


fn main() {
    println!("Tententive de connexion au serveur ...");
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 7878;
    if let Ok(stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}

