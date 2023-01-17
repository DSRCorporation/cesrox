pub use cesrox_core::error::CesrError;
pub use cesrox_core::message::message::{list_from_vec, FFIMessageListReturn, Message, MessageList};
pub use cesrox_core::prefix::self_addressing::SelfAddressingPrefix;
pub use cesrox_core::prefix::BasicPrefix;
pub use cesrox_core::prefix::Prefix;
pub use cesrox_core::prefix::SelfSigningPrefix;
pub use cesrox_core::prefix::SelfAddressingPrefix;

pub fn list_from_vec(bytes: &Vec<u8>) -> CesrResult<FFIMessageListReturn> {
    let (rest, messages) = MessageList::from_stream(bytes).unwrap();
    return Ok(FFIMessageListReturn {
        rest: rest.len().try_into().unwrap(),
        list: messages,
    });
}

uniffi_macros::include_scaffolding!("cesrox");