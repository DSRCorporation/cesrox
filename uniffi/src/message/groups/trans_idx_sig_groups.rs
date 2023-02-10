use cesrox_core::error::CesrResult;
use cesrox_core::{Matter, Group};
pub use cesrox_core::message::groups::{
    TransIdxSigGroups,
    TransIdxSigGroup,
    SealSourceCouple,
    ControllerIdxSigs
};

pub fn trans_idx_sig_group_create(prefixer: Matter, seqner: Matter, saider: Matter, isigers: ControllerIdxSigs) -> TransIdxSigGroup {
    TransIdxSigGroup::new(prefixer, seqner,saider ,isigers)
}

pub fn trans_idx_sig_groups_create(value: Vec<TransIdxSigGroup>) -> TransIdxSigGroups {
    TransIdxSigGroups::new(value)
}

pub fn trans_idx_sig_groups_qb64(trans_idx_sig_groups: &TransIdxSigGroups) -> CesrResult<String> {
    trans_idx_sig_groups.qb64()
}

pub fn trans_idx_sig_groups_qb64b(trans_idx_sig_groups: &TransIdxSigGroups) -> CesrResult<Vec<u8>> {
    trans_idx_sig_groups.qb64b()
}

pub fn trans_idx_sig_groups_qb2(trans_idx_sig_groups: &TransIdxSigGroups) -> CesrResult<Vec<u8>> {
    trans_idx_sig_groups.qb2()
}
