use cesrox_core::error::CesrResult;
use cesrox_core::primitives::prefix::{
    BasicPrefix,
    SelfSigningPrefix,
};
pub use cesrox_core::groups::{
    NontransferableIdentifierReceiptCouples,
    NontransferableIdentifierReceiptCouple,
};
/*
    NontransferableIdentifierReceiptCouple
*/
pub fn non_transferable_identifier_receipt_couple_create(basic: BasicPrefix, self_signing: SelfSigningPrefix) -> NontransferableIdentifierReceiptCouple {
    NontransferableIdentifierReceiptCouple::new(basic, self_signing)
}

/*
    NontransferableIdentifierReceiptCouples
*/
pub struct NontransferableIdentifierReceiptCouplesFromStreamResult {
    pub rest: Vec<u8>,
    pub message: NontransferableIdentifierReceiptCouples
}

pub fn non_transferable_identifier_receipt_couples_create(value: Vec<NontransferableIdentifierReceiptCouple>) -> NontransferableIdentifierReceiptCouples {
    NontransferableIdentifierReceiptCouples::new(value)
}

pub fn non_transferable_identifier_receipt_couples_to_str(non_transferable_identifier_receipt_couples: &NontransferableIdentifierReceiptCouples) -> String {
    non_transferable_identifier_receipt_couples.to_str()
}

pub fn non_transferable_identifier_receipt_couples_from_str(str: &str) -> CesrResult<NontransferableIdentifierReceiptCouples> {
    NontransferableIdentifierReceiptCouples::from_str(str)
}

pub fn non_transferable_identifier_receipt_couples_to_bytes(non_transferable_identifier_receipt_couples: &NontransferableIdentifierReceiptCouples) -> Vec<u8> {
    non_transferable_identifier_receipt_couples.to_bytes()
}

pub fn non_transferable_identifier_receipt_couples_from_bytes(bytes: &[u8]) -> CesrResult<NontransferableIdentifierReceiptCouples> {
    NontransferableIdentifierReceiptCouples::from_bytes(bytes)
}

pub fn non_transferable_identifier_receipt_couples_from_stream_bytes(str: &[u8]) -> CesrResult<NontransferableIdentifierReceiptCouplesFromStreamResult> {
    let (res, message) = NontransferableIdentifierReceiptCouples::from_stream_bytes(str)?;
    Ok(NontransferableIdentifierReceiptCouplesFromStreamResult {
        rest: res.to_vec(),
        message
    })
}