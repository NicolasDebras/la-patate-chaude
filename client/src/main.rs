use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::String;

use lib_common::message::{
    Challenge, ChallengeAnswer, ChallengeResult, ChallengeValue, EndOfGame, MD5HashCashInput, MD5HashCashOutput, Message,
    PublicPlayer, RoundSummary, Subscribe, SubscribeResult, Welcome,
};

struct InfoGame {
    name_player: String,
    players: Vec<PublicPlayer>,
}

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;

    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send message size");
    stream.write_all(&serialized.as_bytes()).expect("failed to send message");
}

fn md5_challenge_resolve(input: MD5HashCashInput) -> MD5HashCashOutput {
    let result_complexity: String = input.complexity.to_string();
    let result_hashcode: String = input.message;
    println!("input.complexity = {result_complexity:?}");
    println!("input.complexity = {result_hashcode:?}");
    MD5HashCashOutput { seed: 00, hashcode: String::from("hello") }
}

fn on_challenge_message(stream: &TcpStream, challenge: Challenge, game_info:&mut InfoGame) {
    println!("hello2");
    match challenge {
        Challenge::MD5HashCash(input) => {
            println!("run the MD5 Challenge");
            let challenge_answer = ChallengeAnswer::MD5HashCash(md5_challenge_resolve(input));
            on_message_challenge_answer(stream, challenge_answer, game_info );
        }
        Challenge::ChallengeTimeout(input) => {
            println!("test= {input:?}");
            println!("test 129");
        }
        Challenge::RecoverSecret() => println!("test"),
    }
}

fn on_message_challenge_answer(stream: &TcpStream, challenge_answer: ChallengeAnswer, game_info:&mut InfoGame ){
    let challenge_result = ChallengeResult{ answer : challenge_answer , next_target: game_info.players[0].name.clone() };
    let message = Message::ChallengeResult(challenge_result);
    send_message(stream, message);
}

pub fn on_welcome_message(stream: &TcpStream, welcome: Welcome, name: String) {
    println!("welcome: {welcome:?}");
    let message_subscribe = Subscribe { name: name.clone() };
    send_message(stream, Message::Subscribe(message_subscribe));
}

pub fn on_subscribe_result_message(subscribe_result: SubscribeResult) -> u32 {
    return match subscribe_result {
        SubscribeResult::Ok => {
            println!("subscribe_result: {subscribe_result:?}");
            println!("Subscribe Success");
            0
        }
        SubscribeResult::Err(ref _err) => {
            println!("subscribe_result: {subscribe_result:?}");
            println!("Not good");
            1
        }
    };
}


pub fn on_leader_board_message(leader_board: &Vec<PublicPlayer>) {
    println!("leader_board: {leader_board:?}");
}

fn on_round_summary(_stream: &TcpStream, round: RoundSummary) {
    let _name = round.challenge.to_string();
    let _test: &f64 = &0.0;
    match &round.chain[0].value {
        ChallengeValue::Timeout => println!("Timeout"),
        ChallengeValue::Unreachable => println!("Unreachable"),

        ChallengeValue::Ok { used_time: _test, next_target: _name } => {
            println!("Ok");
        }
        ChallengeValue::BadResult { used_time: _test, next_target: _name } => {
            println!("Bad Result");
        }
    }
}

fn finish_game(end: EndOfGame) {
    println!("finish");
    println!("endOfGame: {end:?}");
}



fn loop_message(mut _stream: &TcpStream, info_game: &mut InfoGame) {
    let mut buf = [0; 4];
    send_message(_stream, Message::Hello);
    loop {
        match _stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                println!("help");
            }
        }
        let message_size = u32::from_be_bytes(buf);
        let mut message_buf = vec![0; message_size as usize];
        _stream
            .read_exact(&mut message_buf)
            .expect("failed to read message");

        let record = buffer_to_object(&mut message_buf);
        match record {
            Message::Hello => {}
            Message::Welcome(welcome) => on_welcome_message(_stream, welcome, info_game.name_player.clone()),
            Message::Subscribe(_) => {}
            Message::SubscribeResult(subscribe_result) => {
                let code_return = on_subscribe_result_message(subscribe_result);
                if code_return == 1 {
                    println!("Resend the other  name of user {} ", info_game.name_player);
                    break;
                }
            }
            Message::PublicLeaderBoard(leader_board) => {
                info_game.players = leader_board;
                on_leader_board_message(&info_game.players)
            }
            Message::Challenge(challenge) => {
                on_challenge_message(_stream, challenge,  info_game);
            }
            Message::RoundSummary(round) => {
                println!("roundSummary");
                on_round_summary(_stream, round);
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
    let args: Vec<String> = env::args().collect();
    let ip_address = String::from(&args[2]);
    let name = String::from(&args[1]);
    let  address;
    if args.len() == 4 {
        let port = String::from(&args[3]);
        address = ip_address + ":" + &port;
    } else {
        address = ip_address + ":7878";
    }
    let stream = TcpStream::connect(address);
    match stream {
        Ok(stream) => {
            let mut info_game= InfoGame{name_player:name, players: vec![] };
            loop_message(&stream, &mut info_game);
        }
        Err(_err) => panic!("Cannot connect: {}", _err),
    }
}
