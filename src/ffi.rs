use std::str::FromStr;

use crate::{
    error::CesrResult,
    prefix::{BasicPrefix, Prefix, SelfAddressingPrefix, SelfSigningPrefix},
    Message, MessageList,
};

pub fn cesr_message_from_stream(bytes: &[u8]) -> CesrResult<(&[u8], Message)> {
    Message::from_stream(bytes)
}

pub fn cesr_message_to_stream(message: &Message) -> CesrResult<Vec<u8>> {
    message.to_stream()
}

pub fn cesr_parse_message_list_from_stream(bytes: &[u8]) -> CesrResult<(&[u8], Vec<Message>)> {
    MessageList::from_stream(bytes)
}

pub fn cesr_message_list_to_stream(messages: &MessageList) -> CesrResult<Vec<u8>> {
    messages.to_stream()
}

pub fn cesr_basic_prefix_from_string(bytes: &str) -> CesrResult<BasicPrefix> {
    BasicPrefix::from_str(bytes)
}

pub fn cesr_basic_prefix_to_string(basic_prefix: &BasicPrefix) -> CesrResult<String> {
    Ok(basic_prefix.to_str())
}

pub fn cesr_self_signed_prefix_from_string(bytes: &str) -> CesrResult<SelfSigningPrefix> {
    SelfSigningPrefix::from_str(bytes)
}

pub fn cesr_self_signed_prefix_to_string(basic_prefix: &SelfSigningPrefix) -> CesrResult<String> {
    Ok(basic_prefix.to_str())
}

pub fn cesr_self_addressing_prefix_from_string(bytes: &str) -> CesrResult<SelfAddressingPrefix> {
    SelfAddressingPrefix::from_str(bytes)
}

pub fn cesr_self_addressing_prefix_to_string(
    basic_prefix: &SelfAddressingPrefix,
) -> CesrResult<String> {
    Ok(basic_prefix.to_str())
}
