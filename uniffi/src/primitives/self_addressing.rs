use core::str::FromStr;

use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::Prefix;
pub use cesrox_core::primitives::prefix::SelfAddressingPrefix;
pub use cesrox_core::primitives::derivation::SelfAddressingCode;

/*
    SelfAddressingPrefix
*/
pub struct SelfAddressingPrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: SelfAddressingPrefix
}

pub fn self_addressing_prefix_create(code: SelfAddressingCode, digest: Vec<u8>) -> SelfAddressingPrefix {
    SelfAddressingPrefix::new(code, digest)
}

pub fn self_addressing_prefix_to_str(self_addressing_prefix: &SelfAddressingPrefix) -> String {
    self_addressing_prefix.to_str()
}

pub fn self_addressing_prefix_from_str(str: &str) -> CesrResult<SelfAddressingPrefix> {
    SelfAddressingPrefix::from_str(str)
}

pub fn self_addressing_prefix_to_bytes(attached_signature_prefix: &SelfAddressingPrefix) -> Vec<u8> {
    attached_signature_prefix.to_bytes()
}

pub fn self_addressing_prefix_from_bytes(bytes: &[u8]) -> CesrResult<SelfAddressingPrefix> {
    SelfAddressingPrefix::from_bytes(bytes)
}

pub fn self_addressing_prefix_from_stream_bytes(str: &[u8]) -> CesrResult<SelfAddressingPrefixFromStreamResult> {
    let (res, message) = SelfAddressingPrefix::from_stream_bytes(str)?;
    Ok(SelfAddressingPrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}