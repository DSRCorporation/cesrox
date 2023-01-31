use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::{
    IdentifierPrefix,
    SelfAddressingPrefix,
    AttachedSignaturePrefix
};
pub use cesrox_core::groups::EventSeal;
pub use cesrox_core::groups::{
    TransferableIndexedSignaturesGroup,
    TransferableIndexedSignaturesGroups,
};

/*
    EventSeal
*/
pub fn event_seal_create(prefix: IdentifierPrefix, sn: u64, event_digest: SelfAddressingPrefix) -> EventSeal {
    EventSeal::new(prefix, sn, event_digest)
}

/*
    TransferableIndexedSignatureGroup
*/
pub fn transferable_indexed_signatures_group_create(event_seal: EventSeal, signature_prefixes: Vec<AttachedSignaturePrefix>) -> TransferableIndexedSignaturesGroup {
    TransferableIndexedSignaturesGroup::new(event_seal, signature_prefixes)
}

/*
    TransferableIndexedSignaturesGroups
*/
pub struct TransferableIndexedSignaturesGroupsFromStreamResult {
    pub rest: Vec<u8>,
    pub message: TransferableIndexedSignaturesGroups
}

pub fn transferable_indexed_signatures_groups_create(value: Vec<TransferableIndexedSignaturesGroup>) -> TransferableIndexedSignaturesGroups {
    TransferableIndexedSignaturesGroups::new(value)
}

pub fn transferable_indexed_signatures_groups_to_str(transferable_indexed_signatures_groups: &TransferableIndexedSignaturesGroups) -> String {
    transferable_indexed_signatures_groups.to_str()
}

pub fn transferable_indexed_signatures_groups_from_str(str: &str) -> CesrResult<TransferableIndexedSignaturesGroups> {
    TransferableIndexedSignaturesGroups::from_str(str)
}

pub fn transferable_indexed_signatures_groups_to_bytes(transferable_indexed_signatures_groups: &TransferableIndexedSignaturesGroups) -> Vec<u8> {
    transferable_indexed_signatures_groups.to_bytes()
}

pub fn transferable_indexed_signatures_groups_from_bytes(bytes: &[u8]) -> CesrResult<TransferableIndexedSignaturesGroups> {
    TransferableIndexedSignaturesGroups::from_bytes(bytes)
}

pub fn transferable_indexed_signatures_groups_from_stream_bytes(str: &[u8]) -> CesrResult<TransferableIndexedSignaturesGroupsFromStreamResult> {
    let (res, message) = TransferableIndexedSignaturesGroups::from_stream_bytes(str)?;
    Ok(TransferableIndexedSignaturesGroupsFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

