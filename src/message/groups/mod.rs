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

use cesride::counter::Codex;
use cesride::{Counter, Matter};
pub use attached_material_quadlets::AttachedMaterialQuadlets;
pub use controller_idx_sigs::ControllerIdxSigs;
pub use first_seen_replay_couples::{FirstSeenReplayCouple, FirstSeenReplayCouples};
pub use non_trans_receipt_couples::{NonTransReceiptCouples, NonTransReceiptCouple};
pub use pathed_material_quadlets::PathedMaterialQuadlets;
pub use sad_path_sig::SadPathSig;
pub use sad_path_sig_group::SadPathSigGroup;
pub use seal_source_couples::{SealSourceCouples, SealSourceCouple};
pub use trans_idx_sig_groups::{TransIdxSigGroups, TransIdxSigGroup};
pub use trans_last_idx_sig_groups::{TransLastIdxSigGroups, TransLastIdxSigGroup};
pub use trans_receipt_quadruples::{TransReceiptQuadruples, TransReceiptQuadruple};
pub use witness_idx_sigs::WitnessIdxSigs;
use crate::error::{CesrError, CesrResult};

pub trait Group<T: GroupItem> {
    const CODE: Codex;

    fn new(value: Vec<T>) -> Self;

    fn value(&self) -> &Vec<T>;

    fn counter(&self) -> Counter {
        Counter::new(&Self::CODE.code(), self.count())
    }

    fn count(&self) -> u32 {
        self.value().len() as u32
    }

    fn qb64(&self) -> CesrResult<String> {
        let mut out = self.counter().qb64()?;
        for value in self.value().iter() {
            out.push_str(&value.qb64()?);
        }
        Ok(out)
    }

    fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for value in self.value().iter() {
            out.extend_from_slice(&value.qb64b()?);
        }
        Ok(out)
    }

    fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for value in self.value().iter() {
            out.extend_from_slice(&value.qb2()?);
        }
        Ok(out)
    }
}

pub trait GroupItem {
    fn qb64(&self) -> CesrResult<String>;
    fn qb64b(&self) -> CesrResult<Vec<u8>>;
    fn qb2(&self) -> CesrResult<Vec<u8>>;
}

impl GroupItem for Matter {
    fn qb64(&self) -> CesrResult<String> {
        self.qb64().map_err(CesrError::from)
    }

    fn qb64b(&self) -> CesrResult<Vec<u8>> {
        self.qb64b().map_err(CesrError::from)
    }

    fn qb2(&self) -> CesrResult<Vec<u8>> {
        self.qb2().map_err(CesrError::from)
    }
}