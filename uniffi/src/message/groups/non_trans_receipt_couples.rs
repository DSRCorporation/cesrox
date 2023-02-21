use cesrox::error::ParsideResult;
use cesrox::Group;
use cesrox::Cigar;
pub use cesrox::message::groups::{
    NonTransReceiptCouples,
    NonTransReceiptCouple,
};

pub fn non_trans_receipt_couple_create(cigar: Cigar) -> NonTransReceiptCouple {
    NonTransReceiptCouple::new(cigar)
}

pub fn non_trans_receipt_couples_create(value: Vec<NonTransReceiptCouple>) -> NonTransReceiptCouples {
    NonTransReceiptCouples::new(value)
}

pub fn non_trans_receipt_couples_qb64(non_trans_receipt_couples: &NonTransReceiptCouples) -> ParsideResult<String> {
    non_trans_receipt_couples.qb64()
}

pub fn non_trans_receipt_couples_qb64b(non_trans_receipt_couples: &NonTransReceiptCouples) -> ParsideResult<Vec<u8>> {
    non_trans_receipt_couples.qb64b()
}

pub fn non_trans_receipt_couples_qb2(non_trans_receipt_couples: &NonTransReceiptCouples) -> ParsideResult<Vec<u8>> {
    non_trans_receipt_couples.qb2()
}