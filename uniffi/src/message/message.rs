use crate::message::group::CesrGroup;
use cesrox::error::ParsideResult;
pub use cesrox::{
    CustomPayload,
    Message as ParsideMessage,
};

pub struct MessageFromStreamResult {
    pub rest: Vec<u8>,
    pub message: Message,
}

pub struct Message {
    pub payload: Option<CustomPayload>,
    pub group: Option<CesrGroup>,
}

pub fn message_from_stream_bytes(bytes: &[u8]) -> ParsideResult<MessageFromStreamResult> {
    let (rest, message) = ParsideMessage::from_stream_bytes(bytes)?;
    Ok(MessageFromStreamResult {
        rest: rest.to_vec(),
        message: Message::from(message),
    })
}

impl From<ParsideMessage> for Message {
    fn from(message: ParsideMessage) -> Message {
        match message {
            ParsideMessage::Group { value } => Message { group: Some(CesrGroup::from(value)), payload: None },
            ParsideMessage::Custom { value } => Message { payload: Some(value), group: None },
        }
    }
}
