use nom::multi::many0;

use crate::error::{CesrError, CesrResult};
use crate::{nomify, Message};

pub struct MessageList {
    pub messages: Vec<Message>,
}

impl MessageList {
    pub fn from_stream_bytes<'a>(bytes: &'a [u8]) -> CesrResult<(&'a [u8], MessageList)> {
        let (rest, messages) = many0(nomify!(Message::from_stream_bytes))(bytes)?;
        return Ok((
            rest,
            MessageList { messages }
        ));
    }

    pub fn from_bytes(bytes: &[u8]) -> CesrResult<MessageList> {
        let (rest, parsed) = Self::from_stream_bytes(bytes)?;
        if !rest.is_empty() {
            return Err(CesrError::TooMuch);
        }
        Ok(parsed)
    }

    pub fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        Ok(self
            .messages
            .iter()
            .map(|message| message.to_bytes())
            .collect::<CesrResult<Vec<Vec<u8>>>>()?
            .into_iter()
            .flatten()
            .collect()
        )
    }

    pub fn to_str(&self) -> CesrResult<String> {
        Ok(self
            .messages
            .iter()
            .map(|message| message.to_str())
            .collect::<CesrResult<Vec<String>>>()?
            .join("")
        )
    }

    pub fn from_str(s: &str) -> CesrResult<MessageList> {
        MessageList::from_bytes(s.as_bytes())
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
        let (rest, message_list): (&[u8], MessageList) = MessageList::from_stream_bytes(stream).unwrap();

        assert_eq!(1, message_list.messages.len());
        assert_eq!(br#"{"name""#, rest);
        let message = message_list.messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant {..}));

        // parse to specific messages
        let (_rest, message_list): (&[u8], MessageList) = MessageList::from_stream_bytes(stream).unwrap();

        assert_eq!(1, message_list.messages.len());
        let message = message_list.messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant {..}));
        let message = message.typed_payload::<TestMessage>().unwrap();
        assert_eq!("Cesr".to_string(), message.name);

        // Parse multiple messages to list of generic structures
        let stream = br#"{"name":"Cesr"}{"name":"Cesr"}{"name":"Cesr"}{"name""#;
        let (rest, message_list): (&[u8], MessageList) = MessageList::from_stream_bytes(stream).unwrap();

        assert_eq!(3, message_list.messages.len());
        assert_eq!(br#"{"name""#, rest);
        let message = message_list.messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant {..}));
    }

    #[test]
    pub fn test_parse_stream_into_message_list_with_specifying_enum_as_supported_message_types() {
        let stream = br#"{"name":"Cesr"}{"surname":"Parser"}{"name""#;

        let (rest, message_list): (&[u8], MessageList) = MessageList::from_stream_bytes(stream).unwrap();

        assert_eq!(2, message_list.messages.len());
        assert_eq!(br#"{"name""#, rest);

        let message = message_list.messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant {..}));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage(..)));

        let message = message_list.messages.get(1).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant {..}));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage2(..)));
    }

    #[test]
    pub fn test_parse_mixed_stream_into_message_list() {
        let stream = br#"{"name":"Cesr"}-GAC0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII{"surname":"Parse"}"#;

        let (rest, message_list): (&[u8], MessageList) = MessageList::from_stream_bytes(stream).unwrap();

        assert_eq!(3, message_list.messages.len());
        assert_eq!(b"", rest);

        let message = message_list.messages.get(0).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant{..}));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage(..)));

        let message = message_list.messages.get(1).unwrap();
        assert!(matches!(message, Message::CesrGroupVariant{..}));
        let message = message.cesr_group().unwrap();
        // assert!(matches!(message, CesrGroup::SealSourceCoupletsVariant {..}));
        // assert_eq!(
        //     *message,
        //     CesrGroup::SealSourceCoupletsVariant {
        //         value: SealSourceCouplets::new(
        //             vec![
        //                 SourceSeal {
        //                     sn: 1,
        //                     digest: "E3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII"
        //                         .parse()
        //                         .unwrap(),
        //                 },
        //                 SourceSeal {
        //                     sn: 1,
        //                     digest: "E3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII"
        //                         .parse()
        //                         .unwrap(),
        //                 },
        //             ]
        //         )
        //     }
        // );

        let message = message_list.messages.get(2).unwrap();
        assert!(matches!(message, Message::CustomMessageVariant{..}));
        let message = message.typed_payload::<SupportedMessages>().unwrap();
        assert!(matches!(message, SupportedMessages::TestMessage2(..)));
    }

    #[test]
    pub fn test_serialize_mixed_message_list() {
        let stream = br#"{"name":"Cesr"}-GAC0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII{"surname":"Parse"}"#;

        let (_rest, message_list): (&[u8], MessageList) = MessageList::from_stream_bytes(stream).unwrap();

        let serialized = message_list.to_bytes().unwrap();
        assert_eq!(
            std::str::from_utf8(stream),
            std::str::from_utf8(&serialized)
        );
    }
}
