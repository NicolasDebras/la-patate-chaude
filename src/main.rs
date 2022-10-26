mod create_message;


use std::io::{Read, Write};
use std::net::TcpStream;
use crate::create_message::Message;

fn main() {

    let mut stream = TcpStream::connect("localhost:7878").expect("connection failed");

    send_message(&stream, create_message::Message::Hello);
    let mut text = String::new();
    stream.read_to_string(&mut text).expect("read failed");
    println!("got '{}'", text);





    }

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;

    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send message size");
    stream.write_all(&serialized.as_bytes()).expect("failed to send message");
}