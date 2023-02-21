use cesrox::{
    CesrGroup as ParsideCesrGroup,
};
use crate::message::groups::*;

#[derive(Default, Clone)]
pub struct CesrGroup {
    pub attached_material_quadlets: Option<AttachedMaterialQuadlets>,
    pub controller_idx_sigs: Option<ControllerIdxSigs>,
    pub first_seen_replay_couples: Option<FirstSeenReplayCouples>,
    pub non_trans_receipt_couples: Option<NonTransReceiptCouples>,
    pub pathed_material_quadlets: Option<PathedMaterialQuadlets>,
    pub sad_path_sigs: Option<SadPathSigs>,
    pub sad_path_sig_groups: Option<SadPathSigGroups>,
    pub seal_source_couples: Option<SealSourceCouples>,
    pub trans_idx_sig_groups: Option<TransIdxSigGroups>,
    pub trans_last_idx_sig_groups: Option<TransLastIdxSigGroups>,
    pub trans_receipt_quadruples: Option<TransReceiptQuadruples>,
    pub witness_idx_sigs: Option<WitnessIdxSigs>,
}

impl From<ParsideCesrGroup> for CesrGroup {
    fn from(group: ParsideCesrGroup) -> CesrGroup {
        match group {
            ParsideCesrGroup::AttachedMaterialQuadletsVariant { value } =>
                CesrGroup {
                    attached_material_quadlets: Some(AttachedMaterialQuadlets::from(value)),
                    ..Default::default()
                },
            ParsideCesrGroup::ControllerIdxSigsVariant { value } =>
                CesrGroup {
                    controller_idx_sigs: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::FirstSeenReplayCouplesVariant { value } =>
                CesrGroup {
                    first_seen_replay_couples: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::NonTransReceiptCouplesVariant { value } =>
                CesrGroup {
                    non_trans_receipt_couples: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::PathedMaterialQuadletsVariant { value } =>
                CesrGroup {
                    pathed_material_quadlets: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::SadPathSigGroupVariant { value } =>
                CesrGroup {
                    sad_path_sig_groups: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::SadPathSigVariant { value } =>
                CesrGroup {
                    sad_path_sigs: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::SealSourceCouplesVariant { value } =>
                CesrGroup {
                    seal_source_couples: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::TransIdxSigGroupsVariant { value } =>
                CesrGroup {
                    trans_idx_sig_groups: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::TransLastIdxSigGroupsVariant { value } =>
                CesrGroup {
                    trans_last_idx_sig_groups: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::TransReceiptQuadruplesVariant { value } =>
                CesrGroup {
                    trans_receipt_quadruples: Some(value),
                    ..Default::default()
                },
            ParsideCesrGroup::WitnessIdxSigsVariant { value } =>
                CesrGroup {
                    witness_idx_sigs: Some(value),
                    ..Default::default()
                },
        }
    }
}

impl Into<ParsideCesrGroup> for CesrGroup {
    fn into(self) -> ParsideCesrGroup {
        if let Some(value) = self.attached_material_quadlets {
            return ParsideCesrGroup::AttachedMaterialQuadletsVariant { value: value.into() }
        }
        if let Some(value) = self.controller_idx_sigs {
            return ParsideCesrGroup::ControllerIdxSigsVariant { value }
        }
        if let Some(value) = self.first_seen_replay_couples {
            return ParsideCesrGroup::FirstSeenReplayCouplesVariant { value }
        }
        if let Some(value) = self.non_trans_receipt_couples {
            return ParsideCesrGroup::NonTransReceiptCouplesVariant { value }
        }
        if let Some(value) = self.pathed_material_quadlets {
            return ParsideCesrGroup::PathedMaterialQuadletsVariant { value }
        }
        if let Some(value) = self.sad_path_sig_groups {
            return ParsideCesrGroup::SadPathSigGroupVariant { value }
        }
        if let Some(value) = self.sad_path_sigs {
            return ParsideCesrGroup::SadPathSigVariant { value }
        }
        if let Some(value) = self.seal_source_couples {
            return ParsideCesrGroup::SealSourceCouplesVariant { value }
        }
        if let Some(value) = self.trans_idx_sig_groups {
            return ParsideCesrGroup::TransIdxSigGroupsVariant { value }
        }
        if let Some(value) = self.trans_last_idx_sig_groups {
            return ParsideCesrGroup::TransLastIdxSigGroupsVariant { value }
        }
        if let Some(value) = self.trans_receipt_quadruples {
            return ParsideCesrGroup::TransReceiptQuadruplesVariant { value }
        }
        if let Some(value) = self.witness_idx_sigs {
            return ParsideCesrGroup::WitnessIdxSigsVariant { value }
        }
        panic!("Unexpected case")
    }
}