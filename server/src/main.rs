use std::env;
use std::fmt::format;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::ptr::null;

use lib_common::message::{Challenge, ChallengeAnswer, ChallengeResult, EndOfGame, Message, PublicPlayer, RoundSummary, SubscribeResult, ChallengeValue, MD5HashCashInput, MD5HashCashOutput, Welcome, Subscribe, ChallengeTimeout, PublicLeaderBoard, ReportedChallengeResult};


fn on_message_challenge_result(_stream: &TcpStream, challenge_result: ChallengeResult, info_game: &String, players_vec:   Vec<PublicPlayer>) ->  Vec<PublicPlayer>  {
    println!("{challenge_result:?}");
    let result_answer = challenge_result.answer;
    let concurrent = challenge_result.next_target;
    match result_answer {
        ChallengeAnswer::MD5HashCash(md5_output) => {
            println!("{md5_output:?}");
            if md5_output.seed == 0 && md5_output.hashcode == "hello" {
                return on_message_round_summary(_stream, &concurrent, info_game, players_vec);
            }
        }
    }
    return players_vec;

}

fn on_message_round_summary(_stream: &TcpStream, concurrent: &String, info_game: &String, leader_board:  Vec<PublicPlayer>, ) -> Vec<PublicPlayer> {
    let round_result = RoundSummary { challenge: info_game.clone().to_string(), chain: create_vec_reported_challenge(concurrent,  leader_board.clone()) };
    send_message(_stream, Message::RoundSummary(round_result));
    let leader_board_new = on_public_leader_board(_stream, leader_board.clone());
    on_challenge_message(_stream, info_game.clone().to_string());
    return leader_board_new;
}

fn create_reported_challenge(player: PublicPlayer, concurrent: String ) -> ReportedChallengeResult {
    ReportedChallengeResult { name: player.name, value: ChallengeValue::Ok { used_time: 0.0, next_target:concurrent   } }
}

fn create_vec_reported_challenge(concurrent: &String, leader_board: Vec<PublicPlayer>) -> Vec<ReportedChallengeResult> {
    let mut result: Vec<ReportedChallengeResult> = Vec::new();
    for player in leader_board {
        result.push(create_reported_challenge(player, concurrent.to_string()));
    }
    result
}

fn on_message_end_of_game(_stream: &TcpStream, leader_board: Vec<PublicPlayer>) {
    let result_finish = EndOfGame { leader_board };
    send_message(_stream, Message::EndOfGame(result_finish));

}

fn loop_message(mut stream: &TcpStream, game_name: &String, mut players_vec: Vec<PublicPlayer>) {
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
                on_hello_message(stream);
            }
            Message::Welcome(welcome) => println!("Welcome : {welcome:?}"),
            Message::Subscribe(subscribe) => {
                players_vec = on_subscribe_message(stream, subscribe, players_vec);
                println!("{players_vec:?}");
                players_vec = on_public_leader_board(stream, players_vec);
                on_challenge_message(stream, game_name.clone());
            }
            Message::SubscribeResult(subscribe_result) => println!("{subscribe_result:?}"),
            Message::Challenge(challenge) => println!("{challenge:?}"),
            Message::ChallengeResult(challenge_result) => {
                players_vec = on_message_challenge_result(stream, challenge_result, game_name,  players_vec);
            }
            Message::RoundSummary(round_summary) => println!("{round_summary:?}"),
            Message::EndOfGame(end_game) => println!("{end_game:?}"),
            Message::ChallengeTimeout(challenge_timeout) => println!("{challenge_timeout:?}"),
            _ => todo!(),
        }
    }
}

fn on_challenge_message(stream: &TcpStream, game_name: String) {
    if game_name == String::from("md5-hash-cash") {
        let input_md5 = MD5HashCashInput {
            complexity: 1,
            // message to sign
            message: String::from("first Server"),
        };
        let challenge = Challenge::MD5HashCash(input_md5);
        send_message(stream, Message::Challenge(challenge));
    } else if game_name == String::from("recover-secret") {
        println!("Not Implement");
    }
}

fn on_hello_message(stream: &TcpStream) {
    let message_welcome = Welcome { version: 1 };
    send_message(stream, Message::Welcome(message_welcome));
}

fn on_subscribe_message(stream: &TcpStream, subscribe: Subscribe, mut players_vec: Vec<PublicPlayer>) -> Vec<PublicPlayer> {
    println!("Subscribe = {subscribe:?}");
    let state_subscribe: SubscribeResult = SubscribeResult::Ok;
    send_message(stream, Message::SubscribeResult(state_subscribe));
    let _players = &players_vec[0];
    let stream_id = &_players.stream_id;
    let player_modified = PublicPlayer {
        name: subscribe.name,
        stream_id: stream_id.to_string(),
        score: _players.score,
        steps: 0,
        is_active: true,
        total_used_time: 0.0,
    };
    players_vec[0] = player_modified;
    return players_vec;
}

fn on_public_leader_board(stream: &TcpStream, players_vec: Vec<PublicPlayer>) -> Vec<PublicPlayer> {
    let _players = &players_vec[0];
    let stream_id = &_players.stream_id;
    let name = &_players.name;
    let player_modified = PublicPlayer {
        name: name.to_string(),
        stream_id: stream_id.to_string(),
        score: _players.score,
        steps: 0,
        is_active: true,
        total_used_time: 0.0,
    };
    let mut player_vec_copy: Vec<PublicPlayer> = Vec::new();
    player_vec_copy.push(player_modified);
    let message = Message::PublicLeaderBoard(player_vec_copy);
    println!("{message:?}");
    send_message(stream, message);
    return players_vec;
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
    let name_game = String::from(&args[1]);
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let player = PublicPlayer {
                    name: String::from("null"),
                    stream_id: format!("{}", stream.peer_addr().unwrap()),
                    score: 0,
                    steps: 0,
                    is_active: true,
                    total_used_time: 0.0,
                };
                let mut player_vec: Vec<PublicPlayer> = Vec::new();
                player_vec.push(player);
                loop_message(&stream, &name_game, player_vec);
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}
