use parside::MessageList as ParsideMessageList;

use crate::error::CesrResult;
use crate::message::message::Message;

pub struct MessageList {
    pub messages: Vec<Message>,
}

impl MessageList {
    pub fn from_stream_bytes(bytes: &[u8]) -> CesrResult<(&[u8], MessageList)> {
        let (rest, message_list) = ParsideMessageList::from_stream_bytes(bytes)?;
        let messages = message_list
            .messages
            .into_iter()
            .map(|message| Message::try_from(message))
            .collect::<CesrResult<Vec<Message>>>()?;
        return Ok((rest, MessageList { messages }));
    }
}
