use cesrox::{MessageList, Message, CesrGroup};

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_parse_message_list() {
        let stream = br#"{"v":"KERI10JSON00012b_","t":"icp"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG{"v":"KERI10JSON00012b_","t":"icp"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;

        let (rest, message_list) = MessageList::from_stream_bytes(stream).unwrap();

        assert!(rest.is_empty());
        assert_eq!(4, message_list.messages.len());

        assert!(matches!(message_list.messages[0], Message::Custom { .. }));
        assert!(matches!(message_list.messages[1], Message::Group { .. }));
        assert!(matches!(message_list.messages[2], Message::Custom { .. }));
        assert!(matches!(message_list.messages[3], Message::Group { .. }));

        let group = message_list.messages.get(1).unwrap().group().unwrap();
        assert!(matches!(group, CesrGroup::NonTransReceiptCouplesVariant { .. }));

        let group = message_list.messages.get(3).unwrap().group().unwrap();
        match group {
            CesrGroup::NonTransReceiptCouplesVariant { value } => {
                let string = value.to_string().unwrap();
                assert_eq!(string, "-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG");
            },
            _ => panic!("Unexpected group")
        }




    }
}