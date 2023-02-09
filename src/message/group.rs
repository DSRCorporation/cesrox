use cesride::counter::Codex;
use parside::message::group::CesrGroup as ParsideCesrGroup;

use crate::error::{CesrError, CesrResult};
use crate::message::groups::attached_material_quadlets::AttachedMaterialQuadlets;
use crate::message::groups::controller_idx_sigs::ControllerIdxSigs;
use crate::message::groups::first_seen_replay_couples::{
    FirstSeenReplayCouple, FirstSeenReplayCouples,
};
use crate::message::groups::non_trans_receipt_couples::{
    NonTransReceiptCouple, NonTransReceiptCouples,
};
use crate::message::groups::pathed_material_quadlets::PathedMaterialQuadlets;
use crate::message::groups::sad_path_sig::SadPathSig;
use crate::message::groups::sad_path_sig_group::SadPathSigGroup;
use crate::message::groups::seal_source_couples::{SealSourceCouple, SealSourceCouples};
use crate::message::groups::trans_idx_sig_groups::{TransIdxSigGroup, TransIdxSigGroups};
use crate::message::groups::trans_last_idx_sig_groups::{
    TransLastIdxSigGroup, TransLastIdxSigGroups,
};
use crate::message::groups::trans_receipt_quadruples::{
    TransReceiptQuadruple, TransReceiptQuadruples,
};
use crate::message::groups::witness_idx_sigs::WitnessIdxSigs;

#[derive(Debug)]
pub enum CesrGroup {
    ControllerIdxSigsVariant { value: ControllerIdxSigs },
    WitnessIdxSigsVariant { value: WitnessIdxSigs },
    NonTransReceiptCouplesVariant { value: NonTransReceiptCouples },
    TransReceiptQuadruplesVariant { value: TransReceiptQuadruples },
    TransIdxSigGroupsVariant { value: TransIdxSigGroups },
    TransLastIdxSigGroupsVariant { value: TransLastIdxSigGroups },
    FirstSeenReplayCouplesVariant { value: FirstSeenReplayCouples },
    SealSourceCouplesVariant { value: SealSourceCouples },
    AttachedMaterialQuadletsVariant { value: AttachedMaterialQuadlets },
    SadPathSigGroupVariant { value: SadPathSigGroup },
    SadPathSigVariant { value: SadPathSig },
    PathedMaterialQuadletsVariant { value: PathedMaterialQuadlets },
}

impl TryFrom<ParsideCesrGroup> for CesrGroup {
    type Error = CesrError;

    fn try_from(group: ParsideCesrGroup) -> CesrResult<CesrGroup> {
        let code = Codex::from_code(&group.counter.code())?;

        let group = match code {
            ControllerIdxSigs::CODE => CesrGroup::ControllerIdxSigsVariant {
                value: ControllerIdxSigs {
                    value: group.value.single()?,
                },
            },
            WitnessIdxSigs::CODE => CesrGroup::WitnessIdxSigsVariant {
                value: WitnessIdxSigs {
                    value: group.value.single()?,
                },
            },
            NonTransReceiptCouples::CODE => {
                let value = group
                    .value
                    .couple()?
                    .into_iter()
                    .map(|(verfer, cigar)| NonTransReceiptCouple { verfer, cigar })
                    .collect();
                CesrGroup::NonTransReceiptCouplesVariant {
                    value: NonTransReceiptCouples { value },
                }
            }
            TransReceiptQuadruples::CODE => {
                let value = group
                    .value
                    .quadruple()?
                    .into_iter()
                    .map(|(prefixer, seqner, saider, siger)| TransReceiptQuadruple {
                        prefixer,
                        seqner,
                        saider,
                        siger,
                    })
                    .collect();
                CesrGroup::TransReceiptQuadruplesVariant {
                    value: TransReceiptQuadruples { value },
                }
            }
            TransIdxSigGroups::CODE => {
                let value = group
                    .value
                    .quadruple_with_list()?
                    .into_iter()
                    .map(|(prefixer, seqner, saider, isigers)| TransIdxSigGroup {
                        prefixer,
                        seqner,
                        saider,
                        isigers: ControllerIdxSigs::new(isigers),
                    })
                    .collect();
                CesrGroup::TransIdxSigGroupsVariant {
                    value: TransIdxSigGroups { value },
                }
            }
            TransLastIdxSigGroups::CODE => {
                let value = group
                    .value
                    .couple_with_list()?
                    .into_iter()
                    .map(|(prefixer, isigers)| TransLastIdxSigGroup {
                        prefixer,
                        isigers: ControllerIdxSigs::new(isigers),
                    })
                    .collect();
                CesrGroup::TransLastIdxSigGroupsVariant {
                    value: TransLastIdxSigGroups { value },
                }
            }
            FirstSeenReplayCouples::CODE => {
                let value = group
                    .value
                    .couple()?
                    .into_iter()
                    .map(|(firner, dater)| FirstSeenReplayCouple { firner, dater })
                    .collect();
                CesrGroup::FirstSeenReplayCouplesVariant {
                    value: FirstSeenReplayCouples { value },
                }
            }
            SealSourceCouples::CODE => {
                let value = group
                    .value
                    .couple()?
                    .into_iter()
                    .map(|(seqner, saider)| SealSourceCouple { seqner, saider })
                    .collect();
                CesrGroup::SealSourceCouplesVariant {
                    value: SealSourceCouples { value },
                }
            }
            Codex::AttachedMaterialQuadlets => {
                unimplemented!()
            }
            Codex::SadPathSigGroup => {
                unimplemented!()
            }
            Codex::SadPathSig => {
                unimplemented!()
            }
            Codex::PathedMaterialQuadlets => {
                unimplemented!()
            }
            _ => {
                return Err(CesrError::Unexpected(format!(
                    "Unexpected counter code {:?}",
                    group.counter.code()
                )));
            }
        };
        Ok(group)
    }
}
