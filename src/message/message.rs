use nom::multi::many0;

use serde::de::DeserializeOwned;
use serde_json::{self, Value};

use crate::prefix::Prefix;
use crate::{
    error::{CesrError, CesrResult},
    event::sections::seal::{EventSeal, SourceSeal},
    message::{
        message_type::CesrMessageType,
        parsers::{cbor_message, cesr_message, json_message, mgpk_message},
    },
    nomify,
    prefix::{
        AttachedSignaturePrefix, BasicPrefix, IdentifierPrefix, SelfAddressingPrefix,
        SelfSigningPrefix,
    },
};

#[derive(Debug)]
pub enum Message {
    CustomMessage(CustomMessage),
    CesrMessage(CesrMessage),
}

#[derive(Debug)]
pub struct CustomMessage(pub(crate) Value);

impl CustomMessage {
    pub fn value(&self) -> &Value {
        return &self.0;
    }

    pub fn to_stream(&self) -> CesrResult<Vec<u8>> {
        serde_json::to_vec(self.value()).map_err(|err| err.into())
    }

    pub fn to_typed_message<'de, D: DeserializeOwned>(&self) -> CesrResult<D> {
        serde_json::from_value::<D>(self.value().clone()).map_err(|err| err.into())
    }
}

#[derive(Debug, PartialEq)]
pub enum CesrMessage {
    BasicPrefix(BasicPrefix),
    SelfSigning(SelfSigningPrefix),
    SelfAddressing(SelfAddressingPrefix),
    // Count codes
    SealSourceCouplets(Vec<SourceSeal>),
    AttachedSignatures(Vec<AttachedSignaturePrefix>),
    ReceiptCouplets(Vec<(BasicPrefix, SelfSigningPrefix)>),
    // Group codes
    SealSignaturesGroups(Vec<(EventSeal, Vec<AttachedSignaturePrefix>)>),
    // List of signatures made using keys from last establishment event od identifier of prefix
    LastEstSignaturesGroups(Vec<(IdentifierPrefix, Vec<AttachedSignaturePrefix>)>),
    // Frame codes
    Frame(Vec<CesrMessage>),
}

impl CesrMessage {
    pub fn to_stream(&self) -> CesrResult<Vec<u8>> {
        match self {
            CesrMessage::BasicPrefix(prefix) => Ok(prefix.to_str().as_bytes().to_vec()),
            CesrMessage::SelfSigning(self_signing) => Ok(self_signing.to_str().as_bytes().to_vec()),
            CesrMessage::SelfAddressing(self_addressing) => {
                Ok(self_addressing.to_str().as_bytes().to_vec())
            }
            CesrMessage::SealSourceCouplets(_seal_source_couples) => unimplemented!(),
            CesrMessage::AttachedSignatures(_attached_signatures) => unimplemented!(),
            CesrMessage::ReceiptCouplets(_receipt_couples) => unimplemented!(),
            CesrMessage::SealSignaturesGroups(_seal_signature_groups) => unimplemented!(),
            CesrMessage::LastEstSignaturesGroups(_last_est_signatures_groups) => unimplemented!(),
            CesrMessage::Frame(_frame) => unimplemented!(),
        }
    }
}

impl Message {
    pub fn from_stream(bytes: &[u8]) -> CesrResult<(&[u8], Message)> {
        if bytes.is_empty() {
            return Err(CesrError::EmptyBytesStream);
        }

        let msg_type = Self::parse_type(bytes)?;
        match msg_type {
            CesrMessageType::CESR => {
                cesr_message(bytes).map(|(rest, message)| (rest, Message::CesrMessage(message)))
            }
            CesrMessageType::JSON => json_message::<Value>(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomMessage(CustomMessage(value)))),
            CesrMessageType::CBOR => cbor_message::<Value>(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomMessage(CustomMessage(value)))),
            CesrMessageType::MGPK => mgpk_message::<Value>(bytes)
                .map(|(value, _, rest)| (rest, Message::CustomMessage(CustomMessage(value)))),
        }
    }

    pub fn to_stream(&self) -> CesrResult<Vec<u8>> {
        match self {
            Message::CesrMessage(msg) => msg.to_stream(),
            Message::CustomMessage(msg) => msg.to_stream(),
        }
    }

    fn parse_type(bytes: &[u8]) -> CesrResult<CesrMessageType> {
        if bytes.is_empty() {
            return Err(CesrError::EmptyBytesStream);
        }
        CesrMessageType::try_from(bytes[0])
    }

    pub fn custom_message(&self) -> CesrResult<&CustomMessage> {
        match self {
            Message::CesrMessage(_) => Err(CesrError::NotExist),
            Message::CustomMessage(ref message) => Ok(message),
        }
    }

    pub fn custom_typed_message<'de, D: DeserializeOwned>(&self) -> CesrResult<D> {
        match self {
            Message::CesrMessage(_) => Err(CesrError::NotExist),
            Message::CustomMessage(ref message) => message.to_typed_message::<D>(),
        }
    }

    pub fn cesr_message(&self) -> CesrResult<&CesrMessage> {
        match self {
            Message::CesrMessage(ref message) => Ok(message),
            Message::CustomMessage(_) => Err(CesrError::NotExist),
        }
    }
}

pub struct MessageList(pub(crate) Vec<Message>);

impl MessageList {
    pub fn value(&self) -> &Vec<Message> {
        &self.0
    }

    pub fn from_stream<'a>(bytes: &'a [u8]) -> CesrResult<(&'a [u8], Vec<Message>)> {
        let (rest, messages) = many0(nomify!(Message::from_stream))(bytes)?;
        return Ok((rest, messages));
    }

    pub fn to_stream(&self) -> CesrResult<Vec<u8>> {
        Ok(self
            .value()
            .iter()
            .map(|message| message.to_stream())
            .collect::<CesrResult<Vec<Vec<u8>>>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

#[cfg(test)]
pub mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{
        event::sections::seal::SourceSeal, message::message::CesrMessage, Message, MessageList,
    };

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

        let (_rest, message): (&[u8], Message) = Message::from_stream(stream).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));

        // Convert message to stream
        assert_eq!(stream.to_vec(), message.to_stream().unwrap());

        let message = message.custom_typed_message::<TestMessage>().unwrap();
        assert_eq!("Cesr".to_string(), message.name);

        // Parse multiple messages to single generic structure
        let stream = br#"{"name":"Cesr"}{"name":"Cesr"}{"name":"Cesr"}{"name""#;
        let (rest, message): (&[u8], Message) = Message::from_stream(stream).unwrap();

        assert_eq!(br#"{"name":"Cesr"}{"name":"Cesr"}{"name""#, rest);
        assert!(matches!(message, Message::CustomMessage(..)));
    }

    #[test]
    pub fn test_parse_stream_into_message_list() {
        let stream = br#"{"name":"Cesr"}{"name""#;

        // Parse to list of generic structures
        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(1, messages.len());
        assert_eq!(br#"{"name""#, rest);
        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));

        // parse to specific messages
        let (_rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(1, messages.len());
        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));
        let message = message.custom_typed_message::<TestMessage>().unwrap();
        assert_eq!("Cesr".to_string(), message.name);

        // Parse multiple messages to list of generic structures
        let stream = br#"{"name":"Cesr"}{"name":"Cesr"}{"name":"Cesr"}{"name""#;
        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(3, messages.len());
        assert_eq!(br#"{"name""#, rest);
        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));
    }

    #[test]
    pub fn test_parse_stream_into_message_list_with_specifying_enum_as_supported_message_types() {
        let stream = br#"{"name":"Cesr"}{"surname":"Parser"}{"name""#;

        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(2, messages.len());
        assert_eq!(br#"{"name""#, rest);

        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));
        let message = message.custom_typed_message::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage(..)));

        let message = messages.get(1).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));
        let message = message.custom_typed_message::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage2(..)));
    }

    #[test]
    pub fn test_parse_mixed_stream_into_message_list() {
        let stream = br#"{"name": "Cesr"}-GAC0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII{"surname": "Parse"}"#;

        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(3, messages.len());
        assert_eq!(b"", rest);

        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));
        let message = message.custom_typed_message::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage(..)));

        let message = messages.get(1).unwrap();
        assert!(matches!(message, Message::CesrMessage(..)));
        let message = message.cesr_message().unwrap();
        assert!(matches!(message, CesrMessage::SealSourceCouplets(..)));
        assert_eq!(
            *message,
            CesrMessage::SealSourceCouplets(vec![
                SourceSeal {
                    sn: 1,
                    digest: "E3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII"
                        .parse()
                        .unwrap(),
                },
                SourceSeal {
                    sn: 1,
                    digest: "E3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII"
                        .parse()
                        .unwrap(),
                },
            ])
        );

        let message = messages.get(2).unwrap();
        assert!(matches!(message, Message::CustomMessage(..)));
        let message = message.custom_typed_message::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage2(..)));
    }

    #[test]
    pub fn test_parse_different_options_of_generic_message() {
        let message = TestMessage {
            name: "Test".to_string(),
        };

        let json_bytes = serde_json::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_stream(&json_bytes).unwrap();
        let message = message.custom_typed_message::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);

        let rmp_bytes = rmp_serde::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_stream(&rmp_bytes).unwrap();
        let message = message.custom_typed_message::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);

        let cbor_bytes = serde_cbor::to_vec(&message).unwrap();
        let (_rest, message): (&[u8], Message) = Message::from_stream(&cbor_bytes).unwrap();
        let message = message.custom_typed_message::<TestMessage>().unwrap();
        assert_eq!("Test".to_string(), message.name);
    }
}
