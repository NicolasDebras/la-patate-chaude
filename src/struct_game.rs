use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeMessage {
    pub(crate) version: u8,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    pub(crate) Welcome:  WelcomeMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeMessage{
    pub(crate)  name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe{
    pub(crate) Subscribe : SubscribeMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Message {
    Hello,

}