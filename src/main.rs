use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::String;

use crate::create_message::{Challenge, ChallengeAnswer, ChallengeValue, EndOfGame, MD5HashCashInput, MD5HashCashOutput, Message, PublicPlayer, RoundSummary, Subscribe, SubscribeResult, Welcome};

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

fn md5_challenge_resolve(input: MD5HashCashInput) -> MD5HashCashOutput{
    let result_complexity : String = input.complexity.to_string();
    let result_hashcode : String = input.message;
    println!("input.complexity = {result_complexity:?}");
    println!("input.complexity = {result_hashcode:?}");
    MD5HashCashOutput{seed : 00, hashcode: String::from("hello")}
}

fn  on_challenge_message(stream: &TcpStream, challenge: Challenge){
    println!("hello2");
    match challenge {
        Challenge::MD5HashCash(input) => {
            println!("run the MD5 Challenge");
            ChallengeAnswer::MD5HashCash(md5_challenge_resolve(input ));
        },
        Challenge::ChallengeTimeout(input) => {
            println!("test= {input:?}");
            println!("test 129");
        }
        Challenge::RecoverSecret() => println!("test"),
    }
}



pub fn on_welcome_message(stream: &TcpStream, welcome: Welcome, name: String) {
    println!("welcome: {welcome:?}");
    let message_subscribe = Subscribe{ name: name.clone()};
    send_message(stream, Message::Subscribe(message_subscribe));
}

pub fn on_subscribe_result_message(subscribe_result: SubscribeResult) -> u32{

    return match subscribe_result {
        SubscribeResult::Ok => {
            println!("subscribe_result: {subscribe_result:?}");
            println!("Subscribe Success");
            0
        }
        SubscribeResult::Err(ref err) => {
            println!("subscribe_result: {subscribe_result:?}");
            println!("Not good");
            1
        }
    }
}


pub fn on_leader_board_message(leader_board: &Vec<PublicPlayer>){
    println!("learder_board: {leader_board:?}");
}

fn on_round_summary(stream: &TcpStream, round: RoundSummary){
    let name = round.challenge.to_string();
    println!("summary: {name:?}");
    match &round.chain[0].value {
        ChallengeValue::Timeout => println!("Timeout"),
        ChallengeValue::Unreachable => println!("Unreachable"),
        ChallengeValue::Ok  { used_time, next_target }=>  {
            println!("Ok");
        }
        ChallengeValue::BadResult  { used_time, next_target }=> {
            println!("Bad Result");
        }
    }

}

fn finish_game(end: EndOfGame){
    println!("finish");
    println!("endOfGame: {end:?}");
}

fn on_challenge_value(challenge_value: ChallengeValue){
    match challenge_value {
        ChallengeValue::Timeout => println!("Timeout finish : {challenge_value:?}"),
        ChallengeValue::Unreachable => println!("Unreachable : {challenge_value:?}"),
        _ => {println!("Other message")}
    }
}

fn loop_message(mut stream: &TcpStream, name: String) {
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
            Message::Welcome(welcome) => on_welcome_message(stream, welcome, name.clone()),
            Message::Subscribe(_) => {}
            Message::SubscribeResult(subscribe_result) => {
                let codeReturn = on_subscribe_result_message(subscribe_result);
                if codeReturn == 1 {
                    println!("Resend the other  name of user ${name} ");
                    break;
                }
            },
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
                finish_game(end);
                break;
            }
            Message::ChallengeTimeout(times) => {
                println!("times : {times:?}");
                break;
            }
            Message::ChallengeResult(_) => {
                println!("yes")
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
