use std::env;
use std::io::Read;
use std::net::TcpStream;
use std::string::String;

use lib_common::challenge::Challenge as MD5Challenge;
use lib_common::md5::MD5;
use lib_common::message::{
    Challenge, ChallengeAnswer, ChallengeResult, ChallengeValue, EndOfGame, Message, PublicPlayer,
    RoundSummary,
};
use lib_common::recovery_secret::RS;
use lib_common::send_message::{
    buffer_to_object, on_leader_board_message, on_subscribe_result_message, on_welcome_message,
    send_message,
};

struct InfoGame {
    name_player: String,
    players: Vec<PublicPlayer>,
}

/// cette fonction permet de répondre au challenge du serveur
fn on_challenge_message(
    stream: &TcpStream,
    challenge: Challenge,
    game_info: &mut InfoGame,
    name: String,
) {
    match challenge {
        Challenge::MD5HashCash(input) => {
            println!("run the MD5 Challenge {input:?}");
            let test = MD5::new(input);
            let value = test.solve();
            let challenge_answer = ChallengeAnswer::MD5HashCash(value);
            on_message_challenge_answer(stream, challenge_answer, game_info, name);
        }
        Challenge::RecoverSecret(input) => {
            let test = RS::new(input);
            let value = test.solve();
            let challenge_answer = ChallengeAnswer::RecoverSecret(value);
            on_message_challenge_answer(stream, challenge_answer, game_info, name);
        }
        Challenge::ChallengeTimeout(input) => {
            println!("test= {input:?}");
            println!("test 129");
        }
    }
}

/// permet de créer un challenge et de l'envoyer au serveur et de choisir le prochain joueur
fn on_message_challenge_answer(
    stream: &TcpStream,
    challenge_answer: ChallengeAnswer,
    game_info: &mut InfoGame,
    name: String,
) {
    let challenge_result = ChallengeResult {
        answer: challenge_answer,
        next_target: choose_the_player(game_info.players.clone(), name),
    };
    let message = Message::ChallengeResult(challenge_result);
    send_message(stream, message);
}

/// choisi le prochain joueur permettra dans un futur proche de choisir le joueur en multi thread
fn choose_the_player(players: Vec<PublicPlayer>, name: String) -> String {
    for name_player in players {
        println!("{:?}", name_player.name);
        if name_player.name != name {
            return name_player.name;
        }
    }
    name
}

/// permet de savoir si le joueur a gagné ou perdu
fn on_round_summary(_stream: &TcpStream, round: RoundSummary) {
    let _name = round.challenge.to_string();
    let _test: &f64 = &0.0;
    match &round.chain[0].value {
        ChallengeValue::Timeout => println!("Timeout"),
        ChallengeValue::Unreachable => println!("Unreachable"),

        ChallengeValue::Ok {
            used_time: _test,
            next_target: _name,
        } => {
            println!("Ok");
        }
        ChallengeValue::BadResult {
            used_time: _test,
            next_target: _name,
        } => {
            println!("Bad Result");
        }
    }
}

/// permet de signifier la fin de la partie
fn finish_game(end: EndOfGame) {
    println!("finish");
    println!("endOfGame: {end:?}");
}

/// cette fonction permet de faire une boucle infini pour répondre a tout type de message jusqu'a la fin de la partie
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
            Message::Welcome(welcome) => {
                on_welcome_message(_stream, welcome, info_game.name_player.clone())
            }
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

/// connection pour le client et le main pour lancer le client
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
            let mut info_game = InfoGame {
                name_player: name,
                players: vec![],
            };
            loop_message(&stream, &mut info_game);
        }
        Err(_err) => panic!("Cannot connect: {}", _err),
    }
}
