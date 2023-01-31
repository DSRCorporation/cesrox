use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::SelfAddressingPrefix;
pub use cesrox_core::groups::{
    SealSourceCouplets,
    SourceSeal,
};

/*
    SourceSeal
*/
pub fn source_seal_create(sn: u64, digest: SelfAddressingPrefix) -> SourceSeal {
    SourceSeal::new(sn, digest)
}

/*
    SealSourceCouplets
*/
pub struct SealSourceCoupletsFromStreamResult {
    pub rest: Vec<u8>,
    pub message: SealSourceCouplets
}

pub fn seal_source_couplets_create(value: Vec<SourceSeal>) -> SealSourceCouplets {
    SealSourceCouplets::new(value)
}

pub fn seal_source_couplets_to_str(seal_source_couplets: &SealSourceCouplets) -> String {
    seal_source_couplets.to_str()
}

pub fn seal_source_couplets_from_str(str: &str) -> CesrResult<SealSourceCouplets> {
    SealSourceCouplets::from_str(str)
}

pub fn seal_source_couplets_to_bytes(transferable_indexed_signatures_groups: &SealSourceCouplets) -> Vec<u8> {
    transferable_indexed_signatures_groups.to_bytes()
}

pub fn seal_source_couplets_from_bytes(bytes: &[u8]) -> CesrResult<SealSourceCouplets> {
    SealSourceCouplets::from_bytes(bytes)
}

pub fn seal_source_couplets_from_stream_bytes(str: &[u8]) -> CesrResult<SealSourceCoupletsFromStreamResult> {
    let (res, message) = SealSourceCouplets::from_stream_bytes(str)?;
    Ok(SealSourceCoupletsFromStreamResult {
        rest: res.to_vec(),
        message
    })
}
