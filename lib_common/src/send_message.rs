use std::io::Write;
use std::net::TcpStream;

use crate::message::{Message, PublicPlayer, Subscribe, SubscribeResult, Welcome};

/// permet d'envoyer un message au serveur en  le transformant en json au format que veux le serveur
/// et en lui envoyant la taille du message en premier
pub fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");

    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    record
}

/// permet d'envoyer un message au serveur en  le transformant en json au format que veux le serveur
/// et d'afficher le contenu du message
pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;
    println!("{}", serialized);
    println!("{}", serialized_size);

    stream
        .write_all(&serialized_size.to_be_bytes())
        .expect("failed to send message size");
    stream
        .write_all(&serialized.as_bytes())
        .expect("failed to send message");
}

/// afficher le leader board
pub fn on_leader_board_message(leader_board: &Vec<PublicPlayer>) {
    println!("leader_board: {leader_board:?}");
}

/// permet d'envoyer une demande de subscribe du joueur pour participer au jeux et de savoir si le joueur a bien été inscrit
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

/// afficher le message de bienvenue
pub fn on_welcome_message(stream: &TcpStream, welcome: Welcome, name: String) {
    println!("welcome: {welcome:?}");
    let message_subscribe = Subscribe { name: name.clone() };
    send_message(stream, Message::Subscribe(message_subscribe));
}
