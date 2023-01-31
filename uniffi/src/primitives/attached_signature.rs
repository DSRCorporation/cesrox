use core::str::FromStr;

use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::{
    Prefix,
    SelfSigningPrefix,
};
pub use cesrox_core::primitives::prefix::AttachedSignaturePrefix;

/*
    AttachedSignaturePrefix
*/
pub struct AttachedSignaturePrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: AttachedSignaturePrefix
}

pub fn attached_signature_prefix_create(signature: SelfSigningPrefix, index: u16) -> AttachedSignaturePrefix {
    AttachedSignaturePrefix::new(signature, index)
}

pub fn attached_signature_prefix_to_str(attached_signature_prefix: &AttachedSignaturePrefix) -> String {
    attached_signature_prefix.to_str()
}

pub fn attached_signature_prefix_from_str(str: &str) -> CesrResult<AttachedSignaturePrefix> {
    AttachedSignaturePrefix::from_str(str)
}

pub fn attached_signature_prefix_to_bytes(attached_signature_prefix: &AttachedSignaturePrefix) -> Vec<u8> {
    attached_signature_prefix.to_bytes()
}

pub fn attached_signature_prefix_from_bytes(bytes: &[u8]) -> CesrResult<AttachedSignaturePrefix> {
    AttachedSignaturePrefix::from_bytes(bytes)
}

pub fn attached_signature_prefix_from_stream_bytes(str: &[u8]) -> CesrResult<AttachedSignaturePrefixFromStreamResult> {
    let (res, message) = AttachedSignaturePrefix::from_stream_bytes(str)?;
    Ok(AttachedSignaturePrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}
