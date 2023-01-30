use nom::multi::many0;

use crate::error::CesrResult;
use crate::{nomify, Message};

pub struct MessageList(pub(crate) Vec<Message>);

impl MessageList {
    pub fn value(&self) -> &Vec<Message> {
        &self.0
    }

    pub fn from_stream<'a>(bytes: &'a [u8]) -> CesrResult<(&'a [u8], Vec<Message>)> {
        let (rest, messages) = many0(nomify!(Message::from_bytes))(bytes)?;
        return Ok((rest, messages));
    }

    pub fn to_stream(&self) -> CesrResult<Vec<u8>> {
        Ok(self
            .value()
            .iter()
            .map(|message| message.to_bytes())
            .collect::<CesrResult<Vec<Vec<u8>>>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

#[cfg(test)]
pub mod tests {
    use serde::{Deserialize, Serialize};

    use crate::groups::seal_source_couples::SourceSeal;
    use crate::groups::{CesrGroup, SealSourceCouplets};
    use crate::{Message, MessageList};

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
    pub fn test_parse_stream_into_message_list() {
        let stream = br#"{"name":"Cesr"}{"name""#;

        // Parse to list of generic structures
        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(1, messages.len());
        assert_eq!(br#"{"name""#, rest);
        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));

        // parse to specific messages
        let (_rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(1, messages.len());
        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Cesr".to_string(), message.name);

        // Parse multiple messages to list of generic structures
        let stream = br#"{"name":"Cesr"}{"name":"Cesr"}{"name":"Cesr"}{"name""#;
        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(3, messages.len());
        assert_eq!(br#"{"name""#, rest);
        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));
    }

    #[test]
    pub fn test_parse_stream_into_message_list_with_specifying_enum_as_supported_message_types() {
        let stream = br#"{"name":"Cesr"}{"surname":"Parser"}{"name""#;

        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(2, messages.len());
        assert_eq!(br#"{"name""#, rest);

        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage(..)));

        let message = messages.get(1).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage2(..)));
    }

    #[test]
    pub fn test_parse_mixed_stream_into_message_list() {
        let stream = br#"{"name":"Cesr"}-GAC0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII{"surname":"Parse"}"#;

        let (rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        assert_eq!(3, messages.len());
        assert_eq!(b"", rest);

        let message = messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage(..)));

        let message = messages.get(1).unwrap();
        assert!(matches!(message, Message::CesrGroup(..)));
        let message = message.cesr_group().unwrap();
        assert!(matches!(message, CesrGroup::SealSourceCouplets(..)));
        assert_eq!(
            *message,
            CesrGroup::SealSourceCouplets(
                SealSourceCouplets::new(
                    vec![
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
                    ]
                )
            )
        );

        let message = messages.get(2).unwrap();
        assert!(matches!(message, Message::CustomPayload(..)));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage2(..)));
    }

    #[test]
    pub fn test_serialize_mixed_message_list() {
        let stream = br#"{"name":"Cesr"}-GAC0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII{"surname":"Parse"}"#;

        let (_rest, messages): (&[u8], Vec<Message>) = MessageList::from_stream(stream).unwrap();

        let message_list = MessageList(messages);

        let serialized = message_list.to_stream().unwrap();
        assert_eq!(
            std::str::from_utf8(stream),
            std::str::from_utf8(&serialized)
        );
    }
}
