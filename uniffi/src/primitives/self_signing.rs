use core::str::FromStr;

use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::Prefix;
pub use cesrox_core::primitives::prefix::SelfSigningPrefix;
pub use cesrox_core::primitives::derivation::SelfSigningCode;

/*
    SelfSigningPrefix
*/
pub struct SelfSigningPrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: SelfSigningPrefix
}

pub fn self_signing_prefix_create(code: SelfSigningCode, signature: Vec<u8>) -> SelfSigningPrefix {
    SelfSigningPrefix::new(code, signature)
}

pub fn self_signing_prefix_to_str(self_signing_prefix: &SelfSigningPrefix) -> String {
    self_signing_prefix.to_str()
}

pub fn self_signing_prefix_from_str(str: &str) -> CesrResult<SelfSigningPrefix> {
    SelfSigningPrefix::from_str(str)
}

pub fn self_signing_prefix_to_bytes(self_signing_prefix: &SelfSigningPrefix) -> Vec<u8> {
    self_signing_prefix.to_bytes()
}

pub fn self_signing_prefix_from_bytes(bytes: &[u8]) -> CesrResult<SelfSigningPrefix> {
    SelfSigningPrefix::from_bytes(bytes)
}

pub fn self_signing_prefix_from_stream_bytes(str: &[u8]) -> CesrResult<SelfSigningPrefixFromStreamResult> {
    let (res, message) = SelfSigningPrefix::from_stream_bytes(str)?;
    Ok(SelfSigningPrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}
