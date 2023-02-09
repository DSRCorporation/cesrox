use crate::error::{CesrError, CesrResult};
use crate::message::group::CesrGroup;
use parside::message::custom_payload::CustomPayload;
use parside::message::message::Message as ParsideMessage;

#[derive(Debug)]
pub enum Message {
    Custom { value: CustomPayload },
    Group { value: CesrGroup },
}

impl Message {
    pub fn from_stream_bytes(bytes: &[u8]) -> CesrResult<(&[u8], Message)> {
        let (rest, message) = ParsideMessage::from_stream_bytes(bytes)?;
        let message = match message {
            ParsideMessage::Group { value } => Message::Group {
                value: CesrGroup::try_from(value)?,
            },
            ParsideMessage::Custom { value } => Message::Custom { value },
        };
        Ok((rest, message))
    }

    pub fn payload(&self) -> CesrResult<&CustomPayload> {
        match self {
            Message::Group { .. } => Err(CesrError::InvalidState),
            Message::Custom { value } => Ok(value),
        }
    }

    pub fn group(&self) -> CesrResult<&CesrGroup> {
        match self {
            Message::Group { value } => Ok(value),
            Message::Custom { .. } => Err(CesrError::InvalidState),
        }
    }
}

impl TryFrom<ParsideMessage> for Message {
    type Error = CesrError;

    fn try_from(message: ParsideMessage) -> CesrResult<Message> {
        match message {
            ParsideMessage::Custom { value } => Ok(Message::Custom { value }),
            ParsideMessage::Group { value } => Ok(Message::Group {
                value: CesrGroup::try_from(value)?,
            }),
        }
    }
}
