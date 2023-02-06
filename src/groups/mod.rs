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
use cesride::{Codex, Counter, Matter};
use cesride::counter::sizage;
use nom::count;
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::message_type::{CesrMessageFormat, CesrMessageType};
use crate::nomify;
use crate::utils::nom::take_bytes;

#[derive(Debug)]
pub struct CesrGroup {
    pub counter: Counter,
    pub body: Vec<Vec<Matter>>,
}

impl CesrGroup {
    pub fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        Ok(self.to_str().as_bytes().to_vec())
    }

    pub fn from_bytes(bytes: &[u8]) -> CesrResult<CesrGroup> {
        // let (rest, payload_type) = take_bytes(bytes, 2u8)?;
        // let payload_type: Codes = Codes::try_from(from_utf8(payload_type)?)?;
        // match payload_type {
        //     Codes::MG => {
        //         let couples = SealSourceCouplets::from_bytes(rest)?;
        //         Ok(CesrGroup::SealSourceCoupletsVariant { value: couples })
        //     }
        //     Codes::MF => {
        //         let couples = TransferableIndexedSignaturesGroups::from_bytes(rest)?;
        //         Ok(CesrGroup::TransferableIndexedSignaturesGroupsVariant { value: couples })
        //     }
        //     Codes::MA => {
        //         let signatures = IndexedControllerSignatures::from_bytes(rest)?;
        //         Ok(CesrGroup::IndexedControllerSignaturesVariant { value: signatures })
        //     }
        //     Codes::MB => {
        //         let signatures = IndexedWitnessSignatures::from_bytes(rest)?;
        //         Ok(CesrGroup::IndexedWitnessSignaturesVariant { value: signatures })
        //     }
        //     Codes::MC => {
        //         let couplets = NontransferableIdentifierReceiptCouples::from_bytes(rest)?;
        //         Ok(CesrGroup::NontransferableIdentifierReceiptCouplesVariant { value: couplets })
        //     }
        //     Codes::MH => {
        //         let signatures = LastEstSignaturesGroups::from_bytes(rest)?;
        //         Ok(CesrGroup::LastEstSignaturesGroupsVariant { value: signatures })
        //     }
        //     Codes::MV => {
        //         let frame = Frame::from_bytes(rest)?;
        //         Ok(CesrGroup::FrameVariant { value: frame })
        //     }
        //     _ => todo!(),
        // }
        unimplemented!()
    }

    fn matter_from_stream<'a>(bytes: &'a [u8]) -> CesrResult<(&'a [u8], Matter)> {
        // let (bytes, byte) = take_bytes(bytes, 1u8)?;
        let matter = Matter::new_with_qb64b(bytes).unwrap();
        // let matter = match format {
        //     CesrMessageFormat::CtB64 => Matter::new_with_qb64b(bytes).unwrap(),
        //     CesrMessageFormat::OpB64 => {
        //         let qb64 = String::from_utf8(bytes.to_vec())?;
        //         Matter::new_with_qb64(&qb64).unwrap()
        //     }
        //     CesrMessageFormat::CtOpB2 => Matter::new_with_qb2(bytes).unwrap(),
        // };
        let size = matter.size() as usize;
        Ok((&bytes[size..], matter))
    }

    pub fn from_stream_bytes<'a>(bytes: &'a [u8]) -> CesrResult<(&'a [u8], CesrGroup)> {
        // let format = CesrMessageFormat::try_from(bytes[0])?;
        let counter = Counter::new_with_qb64b(&bytes).unwrap();
        let table = sizage(&counter.code()).unwrap();

        let (rest, counter_bytes) = take_bytes(bytes, table.fs)?;

        let size = counter.count() as usize;

        if counter.code() == Codex::ControllerIdxSigs.code().to_string() {
            let (rest, body) = count(
                nomify!(Self::matter_from_stream),
                size,
            )(rest)?;
            return Ok((rest, CesrGroup { counter, body: vec![body] }));
        }

        if counter.code() == Codex::WitnessIdxSigs.code().to_string() {
            let (rest, body) = count(
                nomify!(Self::matter_from_stream),
                size,
            )(rest)?;
            return Ok((rest, CesrGroup { counter, body: vec![body] }));
        }

        if counter.code() == Codex::NonTransReceiptCouples.code().to_string() {
            let (rest, body) = count(
                tuple((
                    nomify!(Self::matter_from_stream),
                    nomify!(Self::matter_from_stream),
                )),
                size,
            )(rest)?;
            let body =
                body
                    .into_iter()
                    .map(|(verfer, cigar)| vec![verfer, cigar])
                    .collect();

            return Ok((rest, CesrGroup { counter, body }));
        }

        if counter.code() == Codex::TransReceiptQuadruples.code().to_string() {
            let (rest, body) = count(
                tuple((
                    nomify!(Self::matter_from_stream),
                    nomify!(Self::matter_from_stream),
                    nomify!(Self::matter_from_stream),
                    nomify!(Self::matter_from_stream),
                )),
                size,
            )(rest)?;
            let body =
                body
                    .into_iter()
                    .map(|(prefixer, seqner, saider, siger)| vec![prefixer, seqner, saider, siger])
                    .collect();

            return Ok((rest, CesrGroup { counter, body }));
        }

        if counter.code() == Codex::FirstSeenReplayCouples.code().to_string() {
            let (rest, body) = count(
                tuple((
                    nomify!(Self::matter_from_stream),
                    nomify!(Self::matter_from_stream),
                )),
                size,
            )(rest)?;
            let body =
                body
                    .into_iter()
                    .map(|(firner, dater)| vec![firner, dater])
                    .collect();

            return Ok((rest, CesrGroup { counter, body }));
        }

        if counter.code() == Codex::SealSourceCouples.code().to_string() {
            let (rest, body) = count(
                tuple((
                    nomify!(Self::matter_from_stream),
                    nomify!(Self::matter_from_stream),
                )),
                size,
            )(rest)?;
            let body =
                body
                    .into_iter()
                    .map(|(seqner, saider)| vec![seqner, saider])
                    .collect();

            return Ok((rest, CesrGroup { counter, body }));
        }

        if counter.code() == Codex::SadPathSigGroup.code().to_string() {
            let (rest, body) = count(
                tuple((
                    nomify!(Self::matter_from_stream),
                    nomify!(Self::matter_from_stream),
                )),
                size,
            )(rest)?;
            let body =
                body
                    .into_iter()
                    .map(|(seqner, saider)| vec![seqner, saider])
                    .collect();

            return Ok((rest, CesrGroup { counter, body }));
        }


        unimplemented!();
    }

    pub fn to_str(&self) -> String {
        unimplemented!();
        //
        // match self {
        //     CesrGroup::IndexedControllerSignaturesVariant { value } => value.to_str(),
        //     CesrGroup::IndexedWitnessSignaturesVariant { value } => value.to_str(),
        //     CesrGroup::SealSourceCoupletsVariant { value } => value.to_str(),
        //     CesrGroup::NontransferableIdentifierReceiptCouplesVariant { value } => value.to_str(),
        //     CesrGroup::TransferableIndexedSignaturesGroupsVariant { value } => {
        //         value.to_str()
        //     }
        //     CesrGroup::LastEstSignaturesGroupsVariant { value } => value.to_str(),
        //     CesrGroup::FrameVariant { value } => value.to_str(),
        // }
    }

    pub fn from_str(str: &str) -> CesrResult<Self> {
        unimplemented!();
        // let (rest, parsed) = CesrGroup::from_stream_bytes(str.as_bytes())?;
        // if !rest.is_empty() {
        //     return Err(CesrError::NotImplementedError);
        // }
        // Ok(parsed)
    }
}
