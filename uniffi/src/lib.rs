pub use cesrox_core::error::CesrError;
pub use cesrox_core::error::CesrResult;
pub use cesrox_core::message::message::{CesrMessage, CustomMessage, Message, MessageList};
pub use cesrox_core::prefix::BasicPrefix;
pub use cesrox_core::prefix::Prefix;
pub use cesrox_core::prefix::SelfAddressingPrefix;
pub use cesrox_core::prefix::SelfSigningPrefix;

use std::str::FromStr;

pub struct FFIMessageListReturn {
    pub rest: u64,
    pub list: Vec<Message>,
}

pub fn parse_message_list(bytes: &Vec<u8>) -> CesrResult<FFIMessageListReturn> {
    let (rest, messages) = MessageList::from_stream(bytes).unwrap();
    return Ok(FFIMessageListReturn {
        rest: rest.len().try_into().unwrap(),
        list: messages,
    });
}

uniffi_macros::include_scaffolding!("cesrox");
