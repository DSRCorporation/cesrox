use nom::multi::many0;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

use crate::error::{CesrError, CesrResult};
use crate::groups::counter::Counter;
use crate::groups::CesrGroup;
use crate::nomify;
use crate::primitives::codes::Codes;
use crate::primitives::derivation::attached_signature::b64_count;
use crate::utils::nom::take_bytes;

#[derive(Debug, Default)]
pub struct Frame {
    pub value: Vec<CesrGroup>,
}

impl Frame {
    pub fn new(value: Vec<CesrGroup>) -> Frame {
        Self { value }
    }

    pub fn to_str(&self) -> String {
        let packed_attachments = self
            .value
            .iter()
            .fold("".to_string(), |acc, att| [acc, att.to_str()].concat());
        Counter::new(
            Codes::MV,
            packed_attachments.len(),
            packed_attachments,
        )
            .pack()
    }

    pub fn from_str(s: &str) -> CesrResult<Self> {
        Self::from_bytes(s.as_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_str().as_bytes().to_vec()
    }

    pub fn from_bytes<'a>(s: &'a [u8]) -> CesrResult<Self> {
        let (rest, parsed) = Self::from_stream_bytes(s)?;
        if !rest.is_empty() {
            return Err(CesrError::NotImplementedError);
        }
        Ok(parsed)
    }

    pub fn from_stream_bytes<'a>(rest: &'a [u8]) -> CesrResult<(&'a [u8], Frame)> {
        let (rest, sc) = b64_count(rest)?;
        // sc * 4 is all attachments length
        let (rest, total) = take_bytes(rest, sc * 4)?;
        let (extra, atts) = match many0(nomify!(CesrGroup::from_stream_bytes))(total) {
            Ok((extra, atts)) => (extra, atts),
            Err(nom::Err::Error((rest, _))) => {
                return Err(CesrError::Incomplete((sc * 4) as usize - rest.len()));
            }
            Err(e) => return Err(CesrError::DeserializeError(e.to_string())),
        };
        if !extra.is_empty() {
            return Err(CesrError::Incomplete((sc * 4) as usize - rest.len()));
        }
        Ok((rest, Frame::new(atts)))
    }
}

impl FromStr for Frame {
    type Err = CesrError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Serialize for Frame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for Frame {
    fn deserialize<D>(deserializer: D) -> Result<Frame, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Frame::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frame() {
        let cesr_attachment = "-VAj-HABE4YPqsEOaPNaZxVIbY-Gx2bJgP-c7AH_K7pEE-YfcI9E-AABAAMX88afPpEfF_HF-E-1uZKyv8b_TdILi2x8vC3Yi7Q7yzHn2fR6Bkl2yn-ZxPqmsTfV3f-H_VQwMgk7jYEukVCA";
        let (rest, att) = CesrGroup::from_stream_bytes(cesr_attachment.as_bytes()).unwrap();
        assert!(rest.is_empty());
        // assert!(matches!(att, CesrGroup::FrameVariant {..}));
    }
}