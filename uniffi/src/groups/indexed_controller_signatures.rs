use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::{
    AttachedSignaturePrefix,
};
pub use cesrox_core::groups::{
    IndexedControllerSignatures,
};

/*
    IndexedControllerSignatures
*/
pub struct IndexedControllerSignaturesFromStreamResult {
    pub rest: Vec<u8>,
    pub message: IndexedControllerSignatures
}

pub fn indexed_controller_signatures_create(value: Vec<AttachedSignaturePrefix>) -> IndexedControllerSignatures {
    IndexedControllerSignatures::new(value)
}

pub fn indexed_controller_signatures_to_str(indexed_controller_signatures: &IndexedControllerSignatures) -> String {
    indexed_controller_signatures.to_str()
}

pub fn indexed_controller_signatures_from_str(str: &str) -> CesrResult<IndexedControllerSignatures> {
    IndexedControllerSignatures::from_str(str)
}

pub fn indexed_controller_signatures_to_bytes(indexed_controller_signatures: &IndexedControllerSignatures) -> Vec<u8> {
    indexed_controller_signatures.to_bytes()
}

pub fn indexed_controller_signatures_from_bytes(bytes: &[u8]) -> CesrResult<IndexedControllerSignatures> {
    IndexedControllerSignatures::from_bytes(bytes)
}

pub fn indexed_controller_signatures_from_stream_bytes(str: &[u8]) -> CesrResult<IndexedControllerSignaturesFromStreamResult> {
    let (res, message) = IndexedControllerSignatures::from_stream_bytes(str)?;
    Ok(IndexedControllerSignaturesFromStreamResult {
        rest: res.to_vec(),
        message
    })
}
