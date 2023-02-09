use cesride::Counter;
use cesride::counter::Codex;
use crate::error::CesrResult;

pub mod attached_material_quadlets;
pub mod controller_idx_sigs;
pub mod first_seen_replay_couples;
pub mod non_trans_receipt_couples;
pub mod pathed_material_quadlets;
pub mod sad_path_sig;
pub mod sad_path_sig_group;
pub mod seal_source_couples;
pub mod trans_idx_sig_groups;
pub mod trans_last_idx_sig_groups;
pub mod trans_receipt_quadruples;
pub mod witness_idx_sigs;

pub use attached_material_quadlets::AttachedMaterialQuadlets;
pub use controller_idx_sigs::ControllerIdxSigs;
pub use first_seen_replay_couples::FirstSeenReplayCouple;
pub use non_trans_receipt_couples::NonTransReceiptCouples;
pub use pathed_material_quadlets::PathedMaterialQuadlets;
pub use sad_path_sig::SadPathSig;
pub use sad_path_sig_group::SadPathSigGroup;
pub use seal_source_couples::{SealSourceCouples, SealSourceCouple};
pub use trans_idx_sig_groups::{TransIdxSigGroups, TransIdxSigGroup};
pub use trans_last_idx_sig_groups::{TransLastIdxSigGroups, TransLastIdxSigGroup};
pub use trans_receipt_quadruples::{TransReceiptQuadruples, TransReceiptQuadruple};
pub use witness_idx_sigs::WitnessIdxSigs;

pub trait Group {
    const CODE: Codex;
    fn count(&self) -> u32;
    fn counter(&self) -> Counter {
        Counter::new(&Self::CODE.code(), self.count())
    }
    fn to_string(&self) -> CesrResult<String>;
    fn to_bytes(&self) -> CesrResult<Vec<u8>>;
}
