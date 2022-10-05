use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct WelcomeMessage {
    definition: Box<str>,
    version: Box<str>,
    number: i32,

}
