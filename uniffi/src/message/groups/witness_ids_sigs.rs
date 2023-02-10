use cesrox_core::error::CesrResult;
use cesrox_core::{Matter, Group};
pub use cesrox_core::message::groups::{
    WitnessIdxSigs,
    SealSourceCouple,
    ControllerIdxSigs
};

pub fn witness_ids_sigs_create(value: Vec<Matter>) -> WitnessIdxSigs {
    WitnessIdxSigs::new(value)
}

pub fn witness_ids_sigs_qb64(witness_ids_sigs: &WitnessIdxSigs) -> CesrResult<String> {
    witness_ids_sigs.qb64()
}

pub fn witness_ids_sigs_qb64b(witness_ids_sigs: &WitnessIdxSigs) -> CesrResult<Vec<u8>> {
    witness_ids_sigs.qb64b()
}

pub fn witness_ids_sigs_qb2(witness_ids_sigs: &WitnessIdxSigs) -> CesrResult<Vec<u8>> {
    witness_ids_sigs.qb2()
}
