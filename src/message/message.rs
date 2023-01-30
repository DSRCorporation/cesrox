use serde::de::DeserializeOwned;

use crate::error::{CesrError, CesrResult};
use crate::groups::CesrGroup;
use crate::message::message_type::CesrMessageType;
use crate::message::custom::CustomMessage;

#[derive(Debug)]
pub enum Message {
    CustomPayload(CustomMessage),
    CesrGroup(CesrGroup),
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> CesrResult<(&[u8], Message)> {
        if bytes.is_empty() {
            return Err(CesrError::EmptyBytesStream);
        }

        let msg_type = CesrMessageType::try_from(bytes[0])?;
        match msg_type {
            CesrMessageType::CESR => CesrGroup::from_bytes(bytes)
                .map(|(rest, message)| (rest, Message::CesrGroup(message))),
            CesrMessageType::JSON => CustomMessage::from_json(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomPayload(value))),
            CesrMessageType::CBOR => CustomMessage::from_cbor(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomPayload(value))),
            CesrMessageType::MGPK => CustomMessage::from_mgpk(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomPayload(value))),
        }
    }

    pub fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        match self {
            Message::CesrGroup(msg) => msg.to_bytes(),
            Message::CustomPayload(msg) => msg.to_bytes(),
        }
    }

    pub fn payload(&self) -> CesrResult<&CustomMessage> {
        match self {
            Message::CesrGroup(_) => Err(CesrError::NotExist),
            Message::CustomPayload(ref message) => Ok(message),
        }
    }

    pub fn typed_payload<'de, D: DeserializeOwned>(&self) -> CesrResult<D> {
        match self {
            Message::CesrGroup(_) => Err(CesrError::NotExist),
            Message::CustomPayload(ref message) => message.to_typed_message::<D>(),
        }
    }

    pub fn cesr_group(&self) -> CesrResult<&CesrGroup> {
        match self {
            Message::CesrGroup(ref message) => Ok(message),
            Message::CustomPayload(_) => Err(CesrError::NotExist),
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

        let (_rest, message): (&[u8], Message) = Message::from_bytes(stream).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));

        // Convert message to stream
        assert_eq!(stream.to_vec(), message.to_bytes().unwrap());

        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Cesr".to_string(), message.name);

        // Parse multiple messages to single generic structure
        let stream = br#"{"name":"Cesr"}{"name":"Cesr"}{"name":"Cesr"}{"name""#;
        let (rest, message): (&[u8], Message) = Message::from_bytes(stream).unwrap();

        assert_eq!(br#"{"name":"Cesr"}{"name":"Cesr"}{"name""#, rest);
        assert!(matches!(message, Message::CustomPayload(..)));
    }

    #[test]
    pub fn test_parse_different_options_of_generic_message() {
        let message = TestMessage {
            name: "Test".to_string(),
        };

        let json_bytes = serde_json::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_bytes(&json_bytes).unwrap();
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);

        let rmp_bytes = rmp_serde::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_bytes(&rmp_bytes).unwrap();
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);

        let cbor_bytes = serde_cbor::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_bytes(&cbor_bytes).unwrap();
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);
    }
}
