use std::env;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

use lib_common::message::{
    Challenge, ChallengeAnswer, ChallengeResult, EndOfGame, Message, PublicPlayer, RoundSummary, SubscribeResult,
    ChallengeValue, MD5HashCashInput,  MD5HashCashOutput, Welcome, Subscribe,  ChallengeTimeout,
};

struct LeaderBoard {
    pub players: Vec<PublicPlayer>,
}

fn loop_message(mut stream: &TcpStream){
    let mut buf = [0; 4];
    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                println!("help");
            }
        }
        println!("run");
        let message_size = u32::from_be_bytes(buf);
        let mut message_buf = vec![0; message_size as usize];
        stream
            .read_exact(&mut message_buf)
            .expect("failed to read message");
        let record = buffer_to_object(&mut message_buf);
        println!("record : {record:?}");
        match record {
            Message::Hello => {
                println!("Hello");
                on_hello_message(stream );
            },
            Message::Welcome(welcome) => println!("Welcome : {welcome:?}"),
            Message::Subscribe(subscribe) => {
                on_subscribe_message(stream, subscribe);
            }
            Message::SubscribeResult(SubscribeResult) => println!("Subscribe"),
            Message::Challenge(Challenge) => {

            },
            Message::ChallengeResult(ChallengeResult) => println!("Cinq"),
            Message::RoundSummary(RoundSummary) => println!("Six"),
            Message::EndOfGame(EndOfGame) => println!("Sept"),
            Message::ChallengeTimeout(ChallengeTimeout) => println!("Huit"),
            _ => todo!(),
        }
    }
}

fn on_hello_message(stream: &TcpStream){
    let message_welcome = Welcome{ version : 1 };
    send_message(stream, Message::Welcome(message_welcome));
}

fn on_subscribe_message(stream: &TcpStream, subscribe: Subscribe){
    println!("Subscribe = {subscribe:?}");
    let state_subscribe : SubscribeResult  = SubscribeResult::Ok;
    send_message(stream, Message::SubscribeResult(state_subscribe));
    let player = PublicPlayer {
         name: subscribe.name,
         stream_id: "0".to_string(),
         score: 0,
         steps: 0,
         is_active: true,
        total_used_time: 0.0,
    };
    let player_vec: Vec<PublicPlayer> = Vec::new();
    player_vec.push(player);
    send_message(stream, Message::PublicLeaderBoard(player_vec));
}

fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");
    //  println!("message: {message:?}");

    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    //  println!("message: {record:?}");
    record
}

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;

    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send message size");
    stream.write_all(&serialized.as_bytes()).expect("failed to send message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let game = String::from(&args[1]);
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                loop_message(&stream);
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

}
