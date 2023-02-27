use crate::message::Message;

pub fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");

    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    record
}
