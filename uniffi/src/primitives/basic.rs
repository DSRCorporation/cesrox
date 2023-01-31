use core::str::FromStr;

use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::Prefix;
pub use cesrox_core::primitives::prefix::BasicPrefix;
pub use cesrox_core::primitives::key::Key;
pub use cesrox_core::primitives::derivation::BasicCode;

/*
    Key
*/
pub fn key_create(value: &[u8]) -> Key {
    Key::new(value)
}

/*
    BasicPrefix
*/
pub struct BasicPrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: BasicPrefix
}

pub fn basic_prefix_create(code: BasicCode, public_key: Key) -> BasicPrefix {
    BasicPrefix::new(code, public_key)
}

pub fn basic_prefix_to_str(basic_prefix: &BasicPrefix) -> String {
    basic_prefix.to_str()
}

pub fn basic_prefix_from_str(str: &str) -> CesrResult<BasicPrefix> {
    BasicPrefix::from_str(str)
}

pub fn basic_prefix_to_bytes(basic_prefix: &BasicPrefix) -> Vec<u8> {
    basic_prefix.to_bytes()
}

pub fn basic_prefix_from_bytes(bytes: &[u8]) -> CesrResult<BasicPrefix> {
    BasicPrefix::from_bytes(bytes)
}

pub fn basic_prefix_from_stream_bytes(bytes: &[u8]) -> CesrResult<BasicPrefixFromStreamResult> {
    let (res, message) = BasicPrefix::from_stream_bytes(bytes)?;
    Ok(BasicPrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}
