use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::String;

use crate::create_message::{Challenge, EndOfGame, Message, PublicPlayer, RoundSummary, Subscribe, SubscribeResult, Welcome};

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

fn  on_challenge_message(stream: &TcpStream, challenge: Challenge){
    println!("hello2");
    match challenge {
        Challenge::MD5HashCash() => println!("hello"),
        Challenge::RecoverSecret() => println!("test"),
    }
}



pub fn on_welcome_message(stream: &TcpStream, welcome: Welcome, name: String) {
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

fn on_round_summary(stream: &TcpStream, round: RoundSummary){
    println!("summary: {round:?}");

}

fn finish_game(end: EndOfGame){
    println!("finish");
    println!("endOfGame: {end:?}");
}
fn loop_message(mut stream: &TcpStream, name: String) {
    let mut buf = [0; 4];
    send_message(stream, Message::Hello);
    loop {
        println!("helop2");
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
            Message::Welcome(welcome) => on_welcome_message(stream, welcome, name.clone()),
            Message::Subscribe(_) => {}
            Message::SubscribeResult(subscribe_result) => on_subscribe_result_message(subscribe_result),
            Message::PublicLeaderBoard(leader_board) => {
                let leader_board = LeaderBoard{ players : leader_board};
                on_leader_board_message(&leader_board.players);
            }
            Message::Challenge(challenge) => {
                on_challenge_message(stream, challenge);
            }
            Message::RoundSummary(round) => {
                on_round_summary(stream, round);
            }
            Message::EndOfGame(end) => {
                finish_game(end)
            }
            _ => {
                println!("help")
            }

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
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    match stream {
        Ok(stream) => {
            loop_message(&stream, name.clone());
        }
        Err(err) => panic!("Cannot connect: {err}"),
    }
}
