use std::io::{ Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};


use crate::create_message::{Message, Welcome};

mod create_message;

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;

    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send message size");
    stream.write_all(&serialized.as_bytes()).expect("failed to send message");
}



pub fn on_welcome(stream: &TcpStream, welcome: Welcome) {
    println!("welcome: {welcome:?}");

}


fn main_loop(mut stream: &TcpStream) {
    let mut buf = [0; 4];

    send_message(stream, Message::Hello);
    match stream.read_exact(&mut buf) {
        Ok(_) => {}
        Err(_) => {
            println!("help");
        }
    }
    let message_size = u32::from_be_bytes(buf);

    let mut message_buf = vec![0; message_size as usize];
    stream
        .read_exact(&mut message_buf)
        .expect("failed to read message");

    let record = buffer_to_object(&mut message_buf);
    match record {
        Message::Hello => {}
        Message::Welcome(welcome) => on_welcome(stream, welcome),
        _ => {}
    }


}

fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");
    //  println!("message: {message:?}");

    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    //  println!("message: {record:?}");
    record
}

fn main() {
    let stream = TcpStream::connect("localhost:7878");


    match stream {
        Ok(stream) => {
            main_loop(&stream);
        }
        Err(err) => panic!("Cannot connect: {err}"),
    }
}
