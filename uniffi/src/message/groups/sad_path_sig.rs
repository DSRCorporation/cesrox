use cesrox_core::error::CesrResult;
use cesrox_core::{Matter, Group};
pub use cesrox_core::message::groups::{
    SadPathSig,
};

pub fn sad_path_sig_create(value: Vec<Matter>) -> SadPathSig {
    SadPathSig::new(value)
}

pub fn sad_path_sig_qb64(sad_path_sig: &SadPathSig) -> CesrResult<String> {
    sad_path_sig.qb64()
}

pub fn sad_path_sig_qb64b(sad_path_sig: &SadPathSig) -> CesrResult<Vec<u8>> {
    sad_path_sig.qb64b()
}

pub fn sad_path_sig_qb2(sad_path_sig: &SadPathSig) -> CesrResult<Vec<u8>> {
    sad_path_sig.qb2()
}

