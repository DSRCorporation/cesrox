use cesrox_core::error::CesrResult;
pub use cesrox_core::message::{
    CustomMessage,
    Message,
    MessageList,
};

/*
    Message
*/
pub struct MessageFromStreamResult {
    pub rest: Vec<u8>,
    pub message: Message
}

pub fn message_from_bytes(bytes: &[u8]) -> CesrResult<Message> {
    Message::from_bytes(bytes)
}

pub fn message_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageFromStreamResult> {
    let (rest, message) = Message::from_stream_bytes(bytes)?;
    Ok(MessageFromStreamResult {
        rest: rest.to_vec(),
        message
    })
}

pub fn message_to_bytes(message: &Message) -> CesrResult<Vec<u8>> {
    message.to_bytes()
}

pub fn message_to_str(message: &Message) -> CesrResult<String> {
    message.to_str()
}

pub fn message_from_str(value: &str) -> CesrResult<Message> {
    Message::from_str(value)
}
