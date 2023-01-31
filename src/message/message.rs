use serde::de::DeserializeOwned;

use crate::error::{CesrError, CesrResult};
use crate::groups::CesrGroup;
use crate::message::message_type::CesrMessageType;
use crate::message::custom::CustomMessage;

#[derive(Debug)]
pub enum Message {
    CustomMessageVariant {
        value: CustomMessage
    },
    CesrGroupVariant {
        value: CesrGroup
    },
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> CesrResult<Message> {
        if bytes.is_empty() {
            return Err(CesrError::EmptyBytesStream);
        }
        let msg_type = CesrMessageType::try_from(bytes[0])?;
        match msg_type {
            CesrMessageType::CESR => CesrGroup::from_bytes(bytes)
                .map(|value| Message::CesrGroupVariant { value }),
            CesrMessageType::JSON => CustomMessage::from_json(bytes)
                .map(|value| Message::CustomMessageVariant { value }),
            CesrMessageType::CBOR => CustomMessage::from_cbor(bytes)
                .map(|value| Message::CustomMessageVariant { value }),
            CesrMessageType::MGPK => CustomMessage::from_mgpk(bytes)
                .map(|value| Message::CustomMessageVariant { value }),
        }
    }

    pub fn from_stream_bytes(bytes: &[u8]) -> CesrResult<(&[u8], Message)> {
        if bytes.is_empty() {
            return Err(CesrError::EmptyBytesStream);
        }

        let msg_type = CesrMessageType::try_from(bytes[0])?;
        match msg_type {
            CesrMessageType::CESR => CesrGroup::from_stream_bytes(bytes)
                .map(|(rest, value)| (rest, Message::CesrGroupVariant { value })),
            CesrMessageType::JSON => CustomMessage::from_json_stream(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomMessageVariant { value })),
            CesrMessageType::CBOR => CustomMessage::from_cbor_stream(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomMessageVariant { value })),
            CesrMessageType::MGPK => CustomMessage::from_mgpk_stream(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomMessageVariant { value })),
        }
    }

    pub fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        match self {
            Message::CesrGroupVariant { value } => value.to_bytes(),
            Message::CustomMessageVariant { value } => value.to_bytes(),
        }
    }

    pub fn to_str(&self) -> CesrResult<String> {
        match self {
            Message::CesrGroupVariant { value } => Ok(value.to_str()),
            Message::CustomMessageVariant { value } => value.to_str(),
        }
    }

    pub fn from_str(s: &str) -> CesrResult<Message> {
        Message::from_bytes(s.as_bytes())
    }

    pub fn payload(&self) -> CesrResult<&CustomMessage> {
        match self {
            Message::CesrGroupVariant { .. } => Err(CesrError::NotExist),
            Message::CustomMessageVariant { value } => Ok(value),
        }
    }

    pub fn typed_payload<'de, D: DeserializeOwned>(&self) -> CesrResult<D> {
        match self {
            Message::CesrGroupVariant { .. } => Err(CesrError::NotExist),
            Message::CustomMessageVariant { value } => value.to_typed_message::<D>(),
        }
    }

    pub fn cesr_group(&self) -> CesrResult<&CesrGroup> {
        match self {
            Message::CesrGroupVariant { value } => Ok(value),
            Message::CustomMessageVariant { .. } => Err(CesrError::NotExist),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use serde::{Deserialize, Serialize};

    use crate::Message;

    #[derive(Deserialize, Serialize)]
    struct TestMessage {
        name: String,
    }

    #[derive(Deserialize, Serialize)]
    struct TestMessage2 {
        surname: String,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(untagged)]
    enum SupportedMessages {
        TestMessage(TestMessage),
        TestMessage2(TestMessage2),
    }

    #[test]
    pub fn test_parse_stream_into_single_message() {
        let stream = br#"{"name":"Cesr"}"#;

        let message = Message::from_bytes(stream).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant {..}));

        // Convert message to stream
        assert_eq!(stream.to_vec(), message.to_bytes().unwrap());

        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Cesr".to_string(), message.name);

        // Parse multiple messages to single generic structure
        let stream = br#"{"name":"Cesr"}{"name":"Cesr"}{"name":"Cesr"}{"name""#;
        let (rest, message): (&[u8], Message) = Message::from_stream_bytes(stream).unwrap();

        assert_eq!(br#"{"name":"Cesr"}{"name":"Cesr"}{"name""#, rest);
        assert!(matches!(message, Message::CustomMessageVariant {..}));
    }

    #[test]
    pub fn test_parse_different_options_of_generic_message() {
        let message = TestMessage {
            name: "Test".to_string(),
        };

        let json_bytes = serde_json::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_stream_bytes(&json_bytes).unwrap();
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);

        let rmp_bytes = rmp_serde::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_stream_bytes(&rmp_bytes).unwrap();
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);

        let cbor_bytes = serde_cbor::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_stream_bytes(&cbor_bytes).unwrap();
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);
    }
}
