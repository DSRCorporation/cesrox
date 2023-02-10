use cesrox_core::error::CesrResult;

use cesrox_core::{
    MessageList as CesroxMessageList,
};
use crate::message::message::Message;

/*
    MessageList
*/
pub struct MessageList {
    pub messages: Vec<Message>,
}

pub struct MessageListFromStreamResult {
    pub rest: Vec<u8>,
    pub messages: Vec<Message>
}

pub fn message_list_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageListFromStreamResult> {
    let (rest, message_list) = CesroxMessageList::from_stream_bytes(bytes)?;
    let messages = message_list.messages.into_iter().map(|message| Message::from(message)).collect();
    Ok(MessageListFromStreamResult {
        rest: rest.to_vec(),
        messages
    })
}
