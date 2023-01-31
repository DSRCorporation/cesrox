use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::{
    AttachedSignaturePrefix,
};
pub use cesrox_core::groups::{
    IndexedControllerSignatures,
    IndexedWitnessSignatures,
};

/*
    IndexedWitnessSignatures
*/
pub struct IndexedWitnessSignaturesFromStreamResult {
    pub rest: Vec<u8>,
    pub message: IndexedWitnessSignatures
}

pub fn witness_controller_signatures_create(value: Vec<AttachedSignaturePrefix>) -> IndexedWitnessSignatures {
    IndexedWitnessSignatures::new(value)
}

pub fn witness_controller_signatures_to_str(witness_controller_signatures: &IndexedWitnessSignatures) -> String {
    witness_controller_signatures.to_str()
}

pub fn witness_controller_signatures_from_str(str: &str) -> CesrResult<IndexedWitnessSignatures> {
    IndexedWitnessSignatures::from_str(str)
}

pub fn witness_controller_signatures_to_bytes(witness_controller_signatures: &IndexedWitnessSignatures) -> Vec<u8> {
    witness_controller_signatures.to_bytes()
}

pub fn witness_controller_signatures_from_bytes(bytes: &[u8]) -> CesrResult<IndexedWitnessSignatures> {
    IndexedWitnessSignatures::from_bytes(bytes)
}

pub fn witness_controller_signatures_from_stream_bytes(str: &[u8]) -> CesrResult<IndexedWitnessSignaturesFromStreamResult> {
    let (res, message) = IndexedWitnessSignatures::from_stream_bytes(str)?;
    Ok(IndexedWitnessSignaturesFromStreamResult {
        rest: res.to_vec(),
        message
    })
}
