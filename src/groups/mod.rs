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
    TransferableIndexedSignaturesGroup, TransferableIndexedSignaturesGroups,
};
pub use self::seal_source_couples::{EventSeal, SealSourceCouplets, SourceSeal};
use crate::error::{CesrError, CesrResult};
pub use crate::groups::frame::Frame;
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
    IndexedControllerSignaturesVariant {
        value: IndexedControllerSignatures
    },
    IndexedWitnessSignaturesVariant {
        value: IndexedWitnessSignatures
    },
    NontransferableIdentifierReceiptCouplesVariant {
        value: NontransferableIdentifierReceiptCouples
    },
    // FirstSeenReplyCouples(Vec<(u64, Timestamp)>),

    // Group codes
    SealSourceCoupletsVariant {
        value: SealSourceCouplets
    },
    TransferableIndexedSignaturesGroupsVariant {
        value: TransferableIndexedSignaturesGroups
    },

    // List of signatures made using keys from last establishment event od identifier of prefix
    LastEstSignaturesGroupsVariant {
        value: LastEstSignaturesGroups
    },
    // Frame codes
    FrameVariant {
        value: Frame
    },
}

impl CesrGroup {
    pub fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        Ok(self.to_str().as_bytes().to_vec())
    }

    pub fn from_bytes(bytes: &[u8]) -> CesrResult<CesrGroup> {
        let (rest, payload_type) = take_bytes(bytes, 2u8)?;
        let payload_type: Codes = Codes::try_from(from_utf8(payload_type)?)?;
        match payload_type {
            Codes::MG => {
                let couples = SealSourceCouplets::from_bytes(rest)?;
                Ok(CesrGroup::SealSourceCoupletsVariant { value: couples })
            }
            Codes::MF => {
                let couples = TransferableIndexedSignaturesGroups::from_bytes(rest)?;
                Ok(CesrGroup::TransferableIndexedSignaturesGroupsVariant { value: couples })
            }
            Codes::MA => {
                let signatures = IndexedControllerSignatures::from_bytes(rest)?;
                Ok(CesrGroup::IndexedControllerSignaturesVariant { value: signatures })
            }
            Codes::MB => {
                let signatures = IndexedWitnessSignatures::from_bytes(rest)?;
                Ok(CesrGroup::IndexedWitnessSignaturesVariant { value: signatures })
            }
            Codes::MC => {
                let couplets = NontransferableIdentifierReceiptCouples::from_bytes(rest)?;
                Ok(CesrGroup::NontransferableIdentifierReceiptCouplesVariant { value: couplets })
            }
            Codes::MH => {
                let signatures = LastEstSignaturesGroups::from_bytes(rest)?;
                Ok(CesrGroup::LastEstSignaturesGroupsVariant { value: signatures })
            }
            Codes::MV => {
                let frame = Frame::from_bytes(rest)?;
                Ok(CesrGroup::FrameVariant { value: frame })
            }
            _ => todo!(),
        }
    }

    pub fn from_stream_bytes(bytes: &[u8]) -> CesrResult<(&[u8], CesrGroup)> {
        let (rest, payload_type) = take_bytes(bytes, 2u8)?;
        let payload_type: Codes = Codes::try_from(from_utf8(payload_type)?)?;
        match payload_type {
            Codes::MG => {
                let (rest, couples) = SealSourceCouplets::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::SealSourceCoupletsVariant { value: couples }))
            }
            Codes::MF => {
                let (rest, couples) = TransferableIndexedSignaturesGroups::from_stream_bytes(rest)?;
                Ok((
                    rest,
                    CesrGroup::TransferableIndexedSignaturesGroupsVariant { value: couples },
                ))
            }
            Codes::MA => {
                let (rest, sigs) = IndexedControllerSignatures::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::IndexedControllerSignaturesVariant { value: sigs }))
            }
            Codes::MB => {
                let (rest, sigs) = IndexedWitnessSignatures::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::IndexedWitnessSignaturesVariant { value: sigs }))
            }
            Codes::MC => {
                let (rest, couplets) = NontransferableIdentifierReceiptCouples::from_stream_bytes(rest)?;
                Ok((
                    rest,
                    CesrGroup::NontransferableIdentifierReceiptCouplesVariant { value: couplets },
                ))
            }
            Codes::MH => {
                let (rest, identifier_sigs) = LastEstSignaturesGroups::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::LastEstSignaturesGroupsVariant { value: identifier_sigs }))
            }
            Codes::MV => {
                let (rest, frame) = Frame::from_stream_bytes(rest)?;
                Ok((rest, CesrGroup::FrameVariant { value: frame }))
            }
            _ => todo!(),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            // CesrGroup::BasicPrefix(prefix) => prefix.to_str(),
            // CesrGroup::SelfSigning(self_signing) => self_signing.to_str(),
            // CesrGroup::SelfAddressing(self_addressing) => self_addressing.to_str(),
            CesrGroup::IndexedControllerSignaturesVariant { value } => value.to_str(),
            CesrGroup::IndexedWitnessSignaturesVariant { value } => value.to_str(),
            CesrGroup::SealSourceCoupletsVariant { value } => value.to_str(),
            CesrGroup::NontransferableIdentifierReceiptCouplesVariant { value } => value.to_str(),
            CesrGroup::TransferableIndexedSignaturesGroupsVariant { value } => {
                value.to_str()
            }
            CesrGroup::LastEstSignaturesGroupsVariant { value } => value.to_str(),
            CesrGroup::FrameVariant { value } => value.to_str(),
        }
    }

    pub fn from_str(str: &str) -> CesrResult<Self> {
        let (rest, parsed) = CesrGroup::from_stream_bytes(str.as_bytes())?;
        if !rest.is_empty() {
            return Err(CesrError::NotImplementedError);
        }
        Ok(parsed)
    }
}
