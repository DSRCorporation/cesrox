use cesrox_core::error::CesrResult;
use cesrox_core::{Matter, Group};
pub use cesrox_core::message::groups::{
    NonTransReceiptCouples,
    NonTransReceiptCouple,
};

pub fn non_trans_receipt_couple_create(verfer: Matter, cigar: Matter) -> NonTransReceiptCouple {
    NonTransReceiptCouple::new(verfer, cigar)
}

pub fn non_trans_receipt_couples_create(value: Vec<NonTransReceiptCouple>) -> NonTransReceiptCouples {
    NonTransReceiptCouples::new(value)
}

pub fn non_trans_receipt_couples_qb64(non_trans_receipt_couples: &NonTransReceiptCouples) -> CesrResult<String> {
    non_trans_receipt_couples.qb64()
}

pub fn non_trans_receipt_couples_qb64b(non_trans_receipt_couples: &NonTransReceiptCouples) -> CesrResult<Vec<u8>> {
    non_trans_receipt_couples.qb64b()
}

pub fn non_trans_receipt_couples_qb2(non_trans_receipt_couples: &NonTransReceiptCouples) -> CesrResult<Vec<u8>> {
    non_trans_receipt_couples.qb2()
}