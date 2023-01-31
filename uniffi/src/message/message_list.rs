use cesrox_core::error::CesrResult;

pub use cesrox_core::message::{
    CustomMessage,
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

pub fn message_list_from_bytes(bytes: &[u8]) -> CesrResult<MessageList> {
    MessageList::from_bytes(bytes)
}

pub fn message_list_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageListFromStreamResult> {
    let (rest, messages) = MessageList::from_stream_bytes(bytes)?;
    Ok(MessageListFromStreamResult {
        rest: rest.to_vec(),
        messages
    })
}

pub fn message_list_to_bytes(message_list: &MessageList) -> CesrResult<Vec<u8>> {
    message_list.to_bytes()
}

pub fn message_list_to_str(message_list: &MessageList) -> CesrResult<String> {
    message_list.to_str()
}

pub fn message_list_from_str(value: &str) -> CesrResult<MessageList> {
    MessageList::from_str(value)
}
