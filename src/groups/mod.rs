pub mod counter;
pub mod frame;
pub mod indexed_controller_signatures;
pub mod indexed_witness_signatures;
pub mod last_est_signatures_groups;
pub mod receipt_couples;
pub mod seal_signatures_groups;
pub mod seal_source_couples;

pub use self::last_est_signatures_groups::{LastEstSignaturesGroup, LastEstSignaturesGroups};
pub use self::receipt_couples::{
    NontransferableIdentifierReceiptCouple, NontransferableIdentifierReceiptCouples,
};
pub use self::seal_signatures_groups::{
    TransferableIndexedSignatureGroup, TransferableIndexedSignatureGroups,
};
pub use self::seal_source_couples::{EventSeal, SealSourceCouplets, SourceSeal};
use crate::error::CesrResult;
use crate::groups::frame::Frame;
pub use crate::groups::indexed_controller_signatures::IndexedControllerSignatures;
pub use crate::groups::indexed_witness_signatures::IndexedWitnessSignatures;
use crate::primitives::codes::Codes;
use std::str::from_utf8;
use crate::utils::nom::take_bytes;

#[derive(Debug, PartialEq)]
pub enum CesrGroup {
    // BasicPrefix(BasicPrefix),
    // SelfSigning(SelfSigningPrefix),
    // SelfAddressing(SelfAddressingPrefix),
    // Count codes
    IndexedControllerSignatures(IndexedControllerSignatures),
    IndexedWitnessSignatures(IndexedWitnessSignatures),
    NontransferableIdentifierReceiptCouples(NontransferableIdentifierReceiptCouples),
    // FirstSeenReplyCouples(Vec<(u64, Timestamp)>),

    // Group codes
    SealSourceCouplets(SealSourceCouplets),
    TransferableIndexedSignaturesGroups(TransferableIndexedSignatureGroups),

    // List of signatures made using keys from last establishment event od identifier of prefix
    LastEstSignaturesGroups(LastEstSignaturesGroups),
    // Frame codes
    Frame(Frame),
}

impl CesrGroup {
    pub fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        Ok(self.to_str().as_bytes().to_vec())
    }

    pub fn from_bytes(bytes: &[u8]) -> CesrResult<(&[u8], CesrGroup)> {
        let (rest, payload_type) = take_bytes(bytes, 2u8)?;
        let payload_type: Codes = Codes::try_from(from_utf8(payload_type)?)?;
        match payload_type {
            Codes::MG => {
                let (rest, couples) = SealSourceCouplets::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::SealSourceCouplets(couples)))
            }
            Codes::MF => {
                let (rest, couples) = TransferableIndexedSignatureGroups::from_stream_bytes(rest)?;
                Ok((
                    rest,
                    CesrGroup::TransferableIndexedSignaturesGroups(couples),
                ))
            }
            Codes::MA => {
                let (rest, sigs) = IndexedControllerSignatures::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::IndexedControllerSignatures(sigs)))
            }
            Codes::MB => {
                let (rest, sigs) = IndexedWitnessSignatures::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::IndexedWitnessSignatures(sigs)))
            }
            Codes::MC => {
                let (rest, couplets) = NontransferableIdentifierReceiptCouples::from_stream_bytes(rest)?;
                Ok((
                    rest,
                    CesrGroup::NontransferableIdentifierReceiptCouples(couplets),
                ))
            }
            Codes::MH => {
                let (rest, identifier_sigs) = LastEstSignaturesGroups::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::LastEstSignaturesGroups(identifier_sigs)))
            }
            Codes::MV => {
                let (rest, frame) = Frame::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::Frame(frame)))
            }
            _ => todo!(),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            // CesrGroup::BasicPrefix(prefix) => prefix.to_str(),
            // CesrGroup::SelfSigning(self_signing) => self_signing.to_str(),
            // CesrGroup::SelfAddressing(self_addressing) => self_addressing.to_str(),
            CesrGroup::IndexedControllerSignatures(signatures) => signatures.to_str(),
            CesrGroup::IndexedWitnessSignatures(signatures) => signatures.to_str(),
            CesrGroup::SealSourceCouplets(sources) => sources.to_str(),
            CesrGroup::NontransferableIdentifierReceiptCouples(couplets) => couplets.to_str(),
            CesrGroup::TransferableIndexedSignaturesGroups(seals_signatures) => {
                seals_signatures.to_str()
            }
            CesrGroup::LastEstSignaturesGroups(signers) => signers.to_str(),
            CesrGroup::Frame(frame) => frame.to_str(),
        }
    }
}
