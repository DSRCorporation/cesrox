use cesrox_core::error::CesrResult;
pub use cesrox_core::{
    CustomPayload,
    Message,
    MessageList,
};

pub struct MessageFromStreamResult {
    pub rest: Vec<u8>,
    pub message: Message
}

pub fn message_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageFromStreamResult> {
    let (rest, message) = Message::from_stream_bytes(bytes)?;
    Ok(MessageFromStreamResult {
        rest: rest.to_vec(),
        message
    })
}