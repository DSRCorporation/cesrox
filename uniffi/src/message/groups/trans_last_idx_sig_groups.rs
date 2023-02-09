use cesrox_core::error::CesrResult;
pub use cesrox_core::Matter;
pub use cesrox_core::message::groups::{
    TransLastIdxSigGroups,
    TransLastIdxSigGroup,
    SealSourceCouple,
    ControllerIdxSigs
};

pub fn trans_last_idx_sig_group_create(prefixer: Matter, isigers: ControllerIdxSigs) -> TransLastIdxSigGroup {
    TransLastIdxSigGroup::new(prefixer, isigers)
}

pub fn trans_last_idx_sig_groups_create(value: Vec<TransLastIdxSigGroup>) -> TransLastIdxSigGroups {
    TransLastIdxSigGroups::new(value)
}

pub fn trans_last_idx_sig_groups_qb64(trans_last_idx_sig_groups: &TransLastIdxSigGroups) -> CesrResult<String> {
    trans_last_idx_sig_groups.qb64()
}

pub fn trans_last_idx_sig_groups_qb64b(trans_last_idx_sig_groups: &TransLastIdxSigGroups) -> CesrResult<Vec<u8>> {
    trans_last_idx_sig_groups.qb64b()
}

pub fn trans_last_idx_sig_groups_qb2(trans_last_idx_sig_groups: &TransLastIdxSigGroups) -> CesrResult<Vec<u8>> {
    trans_last_idx_sig_groups.qb2()
}
