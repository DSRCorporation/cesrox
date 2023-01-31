use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::{
    IdentifierPrefix,
    AttachedSignaturePrefix,
};
pub use cesrox_core::groups::{
    LastEstSignaturesGroups,
    LastEstSignaturesGroup,
};

/*
    LastEstSignaturesGroup
*/
pub fn last_est_signatures_group_create(identifier_prefix: IdentifierPrefix, attached_signature_prefixes: Vec<AttachedSignaturePrefix>) -> LastEstSignaturesGroup {
    LastEstSignaturesGroup::new(identifier_prefix, attached_signature_prefixes)
}

/*
    LastEstSignaturesGroups
*/
pub struct LastEstSignaturesGroupsFromStreamResult {
    pub rest: Vec<u8>,
    pub message: LastEstSignaturesGroups
}

pub fn last_est_signatures_groups_create(value: Vec<LastEstSignaturesGroup>) -> LastEstSignaturesGroups {
    LastEstSignaturesGroups::new(value)
}

pub fn last_est_signatures_groups_to_str(last_est_signatures_group: &LastEstSignaturesGroups) -> String {
    last_est_signatures_group.to_str()
}

pub fn last_est_signatures_groups_from_str(str: &str) -> CesrResult<LastEstSignaturesGroups> {
    LastEstSignaturesGroups::from_str(str)
}

pub fn last_est_signatures_groups_to_bytes(last_est_signatures_group: &LastEstSignaturesGroups) -> Vec<u8> {
    last_est_signatures_group.to_bytes()
}

pub fn last_est_signatures_groups_from_bytes(bytes: &[u8]) -> CesrResult<LastEstSignaturesGroups> {
    LastEstSignaturesGroups::from_bytes(bytes)
}

pub fn last_est_signatures_groups_from_stream_bytes(str: &[u8]) -> CesrResult<LastEstSignaturesGroupsFromStreamResult> {
    let (res, message) = LastEstSignaturesGroups::from_stream_bytes(str)?;
    Ok(LastEstSignaturesGroupsFromStreamResult {
        rest: res.to_vec(),
        message
    })
}