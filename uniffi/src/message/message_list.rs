use cesrox::error::ParsideResult;

use cesrox::{
    MessageList as ParsideMessageList,
};
use crate::message::message::Message;

pub struct MessageList {
    pub messages: Vec<Message>,
}

pub struct MessageListFromStreamResult {
    pub rest: Vec<u8>,
    pub messages: Vec<Message>
}

pub fn message_list_from_stream_bytes(bytes: &[u8]) -> ParsideResult<MessageListFromStreamResult> {
    let (rest, message_list) = ParsideMessageList::from_stream_bytes(bytes)?;
    let messages = message_list.messages.into_iter().map(|message| Message::from(message)).collect();
    Ok(MessageListFromStreamResult {
        rest: rest.to_vec(),
        messages
    })
}
