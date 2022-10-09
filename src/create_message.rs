
use crate::{Welcome, WelcomeMessage};
use crate::struct_game::{Subscribe, SubscribeMessage};

pub fn welcome_message(version: u8) -> String {
    let message = WelcomeMessage{  version };
    let welcome = Welcome{Welcome: message};
    let ret = serde_json::to_string(&welcome).unwrap();
    return ret

}

pub fn subscribe_message(name: String) -> String {
    let message = SubscribeMessage{ name};
    let  sub = Subscribe{Subscribe : message};
    let ret = serde_json::to_string(&sub).unwrap();
    return ret
}