pub use cesrox_core::error::{CesrError, CesrResult};
pub use cesrox_core::Matter;

pub fn matter_new_with_code_and_raw(code: &str, raw: &[u8], raw_size: u64) -> CesrResult<Matter> {
    Matter::new_with_code_and_raw(code, raw, raw_size as usize).map_err(CesrError::from)
}

pub fn matter_new_with_qb64(qb64: &str) -> CesrResult<Matter> {
    Matter::new_with_qb64(qb64).map_err(CesrError::from)
}

pub fn matter_new_with_qb64b(qb64b: &[u8]) -> CesrResult<Matter> {
    Matter::new_with_qb64b(qb64b).map_err(CesrError::from)
}

pub fn matter_new_with_qb2(qb2: &[u8]) -> CesrResult<Matter> {
    Matter::new_with_qb2(qb2).map_err(CesrError::from)
}

pub fn matter_code(matter: &Matter) -> String {
    matter.code()
}

pub fn matter_size(matter: &Matter) -> u32 {
    matter.size()
}

pub fn matter_raw(matter: &Matter) -> Vec<u8> {
    matter.raw()
}

pub fn matter_qb64(matter: &Matter) -> CesrResult<String> {
    matter.qb64().map_err(CesrError::from)
}

pub fn matter_qb64b(matter: &Matter) -> CesrResult<Vec<u8>> {
    matter.qb64b().map_err(CesrError::from)
}

pub fn matter_qb2(matter: &Matter) -> CesrResult<Vec<u8>> {
    matter.qb2().map_err(CesrError::from)
}

pub fn matter_full_size(matter: &Matter) -> CesrResult<u32> {
    matter.full_size().map_err(CesrError::from)
}