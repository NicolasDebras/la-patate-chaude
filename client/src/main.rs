use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::String;
use lib_common::challenge::Challenge as MD5Challenge;
use lib_common::message::{
    Challenge, ChallengeAnswer, ChallengeResult, ChallengeValue, EndOfGame, MD5HashCashInput, MD5HashCashOutput, Message,
    PublicPlayer, RoundSummary, Subscribe, SubscribeResult, Welcome, RecoverSecretOutput,
};
use lib_common::md5::MD5;
use lib_common::recovery_secret::RS;

struct InfoGame {
    name_player: String,
    players: Vec<PublicPlayer>,
}

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;
    println!("{}", serialized);
    println!("{}", serialized_size);

    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send message size");
    stream.write_all(&serialized.as_bytes()).expect("failed to send message");
}


fn on_challenge_message(stream: &TcpStream, challenge: Challenge, game_info: &mut InfoGame, name: String) {
    println!("hello2");
    match challenge {
        Challenge::MD5HashCash(input) => {
            println!("run the MD5 Challenge {input:?}");
            let test = MD5::new(input);
            let value = test.solve();
            print!("hello");
            let challenge_answer = ChallengeAnswer::MD5HashCash(value);
            print!("help");
            on_message_challenge_answer(stream, challenge_answer, game_info, name);
        }
        Challenge::RecoverSecret(input ) => {
            print!("run the Recovery Challenge");
            let test = RS::new(input);
            on_message_challenge_answer(stream, test.solve(), game_info, name)
            
        }
        Challenge::ChallengeTimeout(input) => {
            println!("test= {input:?}");
            println!("test 129");
        }
    }
}

fn on_message_challenge_answer(stream: &TcpStream, challenge_answer: ChallengeAnswer, game_info: &mut InfoGame, name: String) {
    let challenge_result = ChallengeResult { answer: challenge_answer, next_target: choose_the_player(game_info.players.clone(),name)  };
    let message = Message::ChallengeResult(challenge_result);
    send_message(stream, message);
}

fn choose_the_player(players: Vec<PublicPlayer>, name: String)-> String {
    for name_player in players{
        println!("{:?}", name_player.name);
        if name_player.name != name{
            return name_player.name;
        }
    }
    name
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
    //on_challenge_message(stream: &TcpStream, challenge: Challenge, game_info:&mut InfoGame);
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
                on_leader_board_message(&info_game.players);
            }
            Message::Challenge(challenge) => {
                println!("challenge");
                on_challenge_message(_stream, challenge, info_game, info_game.name_player.clone());
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
    let address;
    if args.len() == 4 {
        let port = String::from(&args[3]);
        address = ip_address + ":" + &port;
    } else {
        address = ip_address + ":7878";
    }
    let stream = TcpStream::connect(address);
    match stream {
        Ok(stream) => {
            let mut info_game = InfoGame { name_player: name, players: vec![] };
            loop_message(&stream, &mut info_game);
        }
        Err(_err) => panic!("Cannot connect: {}", _err),
    }
}
