use cesrox_core::{
    CesrGroup as CesroxGroup,
};
pub use cesrox_core::message::groups::*;

#[derive(Default)]
pub struct CesrGroup {
    pub attached_material_quadlets: Option<AttachedMaterialQuadlets>,
    pub controller_idx_sigs: Option<ControllerIdxSigs>,
    pub first_seen_replay_couples: Option<FirstSeenReplayCouples>,
    pub non_trans_receipt_couples: Option<NonTransReceiptCouples>,
    pub pathed_material_quadlets: Option<PathedMaterialQuadlets>,
    pub sad_path_sig: Option<SadPathSig>,
    pub sad_path_sig_group: Option<SadPathSigGroup>,
    pub seal_source_couples: Option<SealSourceCouples>,
    pub trans_idx_sig_groups: Option<TransIdxSigGroups>,
    pub trans_last_idx_sig_groups: Option<TransLastIdxSigGroups>,
    pub trans_receipt_quadruples: Option<TransReceiptQuadruples>,
    pub witness_idx_sigs: Option<WitnessIdxSigs>,
}

impl From<CesroxGroup> for CesrGroup {
    fn from(group: CesroxGroup) -> CesrGroup {
        match group {
            CesroxGroup::AttachedMaterialQuadletsVariant { value } =>
                CesrGroup {
                    attached_material_quadlets: Some(value),
                    ..Default::default()
                },
            CesroxGroup::ControllerIdxSigsVariant { value } =>
                CesrGroup {
                    controller_idx_sigs: Some(value),
                    ..Default::default()
                },
            CesroxGroup::FirstSeenReplayCouplesVariant { value } =>
                CesrGroup {
                    first_seen_replay_couples: Some(value),
                    ..Default::default()
                },
            CesroxGroup::NonTransReceiptCouplesVariant { value } =>
                CesrGroup {
                    non_trans_receipt_couples: Some(value),
                    ..Default::default()
                },
            CesroxGroup::PathedMaterialQuadletsVariant { value } =>
                CesrGroup {
                    pathed_material_quadlets: Some(value),
                    ..Default::default()
                },
            CesroxGroup::SadPathSigGroupVariant { value } =>
                CesrGroup {
                    sad_path_sig_group: Some(value),
                    ..Default::default()
                },
            CesroxGroup::SadPathSigVariant { value } =>
                CesrGroup {
                    sad_path_sig: Some(value),
                    ..Default::default()
                },
            CesroxGroup::SealSourceCouplesVariant { value } =>
                CesrGroup {
                    seal_source_couples: Some(value),
                    ..Default::default()
                },
            CesroxGroup::TransIdxSigGroupsVariant { value } =>
                CesrGroup {
                    trans_idx_sig_groups: Some(value),
                    ..Default::default()
                },
            CesroxGroup::TransLastIdxSigGroupsVariant { value } =>
                CesrGroup {
                    trans_last_idx_sig_groups: Some(value),
                    ..Default::default()
                },
            CesroxGroup::TransReceiptQuadruplesVariant { value } =>
                CesrGroup {
                    trans_receipt_quadruples: Some(value),
                    ..Default::default()
                },
            CesroxGroup::WitnessIdxSigsVariant { value } =>
                CesrGroup {
                    witness_idx_sigs: Some(value),
                    ..Default::default()
                },
        }
    }
}