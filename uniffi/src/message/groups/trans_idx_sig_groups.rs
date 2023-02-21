use cesrox::error::ParsideResult;
use cesrox::{Prefixer, Seqner};
use cesrox::Saider;
pub use cesrox::message::groups::{
    TransIdxSigGroups,
    TransIdxSigGroup,
    ControllerIdxSigs
};

pub fn trans_idx_sig_group_create(prefixer: Prefixer, seqner: Seqner, saider: Saider, isigers: ControllerIdxSigs) -> TransIdxSigGroup {
    TransIdxSigGroup::new(prefixer, seqner,saider ,isigers)
}

pub fn trans_idx_sig_groups_create(value: Vec<TransIdxSigGroup>) -> TransIdxSigGroups {
    TransIdxSigGroups::new(value)
}

pub fn trans_idx_sig_groups_qb64(trans_idx_sig_groups: &TransIdxSigGroups) -> ParsideResult<String> {
    trans_idx_sig_groups.qb64()
}

pub fn trans_idx_sig_groups_qb64b(trans_idx_sig_groups: &TransIdxSigGroups) -> ParsideResult<Vec<u8>> {
    trans_idx_sig_groups.qb64b()
}

pub fn trans_idx_sig_groups_qb2(trans_idx_sig_groups: &TransIdxSigGroups) -> ParsideResult<Vec<u8>> {
    trans_idx_sig_groups.qb2()
}
