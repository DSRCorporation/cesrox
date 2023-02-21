use cesrox::error::ParsideResult;
use cesrox::Group;
use cesrox::{Prefixer, Seqner, Saider, Siger};
pub use cesrox::message::groups::{
    TransReceiptQuadruples,
    TransReceiptQuadruple,
};

pub fn trans_receipt_quadruple_create(prefixer: Prefixer, seqner: Seqner, saider: Saider, siger: Siger) -> TransReceiptQuadruple {
    TransReceiptQuadruple::new(prefixer, seqner, saider, siger)
}

pub fn trans_receipt_quadruples_create(value: Vec<TransReceiptQuadruple>) -> TransReceiptQuadruples {
    TransReceiptQuadruples::new(value)
}

pub fn trans_receipt_quadruples_qb64(trans_receipt_quadruples: &TransReceiptQuadruples) -> ParsideResult<String> {
    trans_receipt_quadruples.qb64()
}

pub fn trans_receipt_quadruples_qb64b(trans_receipt_quadruples: &TransReceiptQuadruples) -> ParsideResult<Vec<u8>> {
    trans_receipt_quadruples.qb64b()
}

pub fn trans_receipt_quadruples_qb2(trans_receipt_quadruples: &TransReceiptQuadruples) -> ParsideResult<Vec<u8>> {
    trans_receipt_quadruples.qb2()
}
