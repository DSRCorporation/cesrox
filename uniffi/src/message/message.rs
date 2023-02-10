use crate::message::group::CesrGroup;
use cesrox_core::error::CesrResult;
use cesrox_core::{
    CustomPayload,
    Message as CesroxMessage,
};

pub struct MessageFromStreamResult {
    pub rest: Vec<u8>,
    pub message: Message,
}

pub struct Message {
    pub payload: Option<CustomPayload>,
    pub group: Option<CesrGroup>,
}

pub fn message_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageFromStreamResult> {
    let (rest, message) = CesroxMessage::from_stream_bytes(bytes)?;
    Ok(MessageFromStreamResult {
        rest: rest.to_vec(),
        message: Message::from(message),
    })
}

impl From<CesroxMessage> for Message {
    fn from(message: CesroxMessage) -> Message {
        match message {
            CesroxMessage::Group { value } => Message { group: Some(CesrGroup::from(value)), payload: None },
            CesroxMessage::Custom { value } => Message { payload: Some(value), group: None },
        }
    }
}