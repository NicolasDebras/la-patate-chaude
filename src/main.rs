use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::String;

use crate::create_message::{Message, PublicPlayer, Subscribe, SubscribeResult, Welcome};

mod create_message;

struct LeaderBoard {
    pub players: Vec<PublicPlayer>,
}
pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;

    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send message size");
    stream.write_all(&serialized.as_bytes()).expect("failed to send message");
}



pub fn on_welcome_message(stream: &TcpStream, welcome: Welcome) {
    let name = String::from("aristide");
    println!("welcome: {welcome:?}");
    let message_subscribe = Subscribe{ name: name.clone()};
    send_message(stream, Message::Subscribe(message_subscribe));
}

pub fn on_subscribe_result_message(subscribe_result: SubscribeResult){
    println!("subscribe_result: {subscribe_result:?}");
}


pub fn on_leader_board_message(leader_board: &Vec<PublicPlayer>){
    println!("hello");
    println!("learder_board: {leader_board:?}");
}


fn loop_message(mut stream: &TcpStream) {
    let mut buf = [0; 4];
    send_message(stream, Message::Hello);
    loop {
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
            Message::Welcome(welcome) => on_welcome_message(stream, welcome),
            Message::Subscribe(_) => {}
            Message::SubscribeResult(subscribe_result) => on_subscribe_result_message(subscribe_result),
            Message::PublicLeaderBoard(leader_board) => {
                let leader_board = LeaderBoard{ players : leader_board};
                on_leader_board_message(&leader_board.players);
            }
            _ => {}
        }
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
    for argument in env::args() {
        println!("{argument}");
    }

    match stream {
        Ok(stream) => {
            loop_message(&stream);
        }
        Err(err) => panic!("Cannot connect: {err}"),
    }
}
