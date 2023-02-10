use cesrox_core::error::CesrResult;
use cesrox_core::{Matter, Group};
pub use cesrox_core::message::groups::{
    TransReceiptQuadruples,
    TransReceiptQuadruple,
    SealSourceCouple,
    ControllerIdxSigs
};

pub fn trans_receipt_quadruple_create(prefixer: Matter, seqner: Matter, saider: Matter, siger: Matter) -> TransReceiptQuadruple {
    TransReceiptQuadruple::new(prefixer, seqner, saider, siger)
}

pub fn trans_receipt_quadruples_create(value: Vec<TransReceiptQuadruple>) -> TransReceiptQuadruples {
    TransReceiptQuadruples::new(value)
}

pub fn trans_receipt_quadruples_qb64(trans_receipt_quadruples: &TransReceiptQuadruples) -> CesrResult<String> {
    trans_receipt_quadruples.qb64()
}

pub fn trans_receipt_quadruples_qb64b(trans_receipt_quadruples: &TransReceiptQuadruples) -> CesrResult<Vec<u8>> {
    trans_receipt_quadruples.qb64b()
}

pub fn trans_receipt_quadruples_qb2(trans_receipt_quadruples: &TransReceiptQuadruples) -> CesrResult<Vec<u8>> {
    trans_receipt_quadruples.qb2()
}
