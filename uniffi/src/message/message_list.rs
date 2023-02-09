use cesrox_core::error::CesrResult;

pub use cesrox_core::{
    CustomPayload,
    Message,
    MessageList,
};

/*
    MessageList
*/
pub struct MessageListFromStreamResult {
    pub rest: Vec<u8>,
    pub messages: MessageList
}

pub fn message_list_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageListFromStreamResult> {
    let (rest, messages) = MessageList::from_stream_bytes(bytes)?;
    Ok(MessageListFromStreamResult {
        rest: rest.to_vec(),
        messages
    })
}
