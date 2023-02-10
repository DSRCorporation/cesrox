use cesrox_core::error::CesrResult;
use cesrox_core::{Matter, Group};
pub use cesrox_core::message::groups::ControllerIdxSigs;

pub fn controller_idx_sigs_create(value: Vec<Matter>) -> ControllerIdxSigs {
    ControllerIdxSigs::new(value)
}

pub fn controller_idx_sigs_qb64(controller_idx_sigs: &ControllerIdxSigs) -> CesrResult<String> {
    controller_idx_sigs.qb64()
}

pub fn controller_idx_sigs_qb64b(controller_idx_sigs: &ControllerIdxSigs) -> CesrResult<Vec<u8>> {
    controller_idx_sigs.qb64b()
}

pub fn controller_idx_sigs_qb2(controller_idx_sigs: &ControllerIdxSigs) -> CesrResult<Vec<u8>> {
    controller_idx_sigs.qb2()
}
