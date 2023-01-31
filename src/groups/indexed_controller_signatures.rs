use crate::error::{CesrError, CesrResult};
use crate::groups::counter::Counter;
use crate::nomify;
use crate::primitives::codes::Codes;
use crate::primitives::prefix::{AttachedSignaturePrefix, Prefix};
use nom::multi::count;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use crate::primitives::derivation::attached_signature::b64_count;
use crate::utils::nom::take_bytes;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IndexedControllerSignatures {
    pub value: Vec<AttachedSignaturePrefix>,
}

impl IndexedControllerSignatures {
    pub fn new(value: Vec<AttachedSignaturePrefix>) -> IndexedControllerSignatures {
        Self { value }
    }

    pub fn to_str(&self) -> String {
        let data = self
            .value
            .iter()
            .fold("".into(), |acc, sig| [acc, sig.to_str()].join(""));

        Counter::new(Codes::MA, self.value.len(), data).pack()
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

    pub fn from_stream_bytes<'a>(s: &'a [u8]) -> CesrResult<(&'a [u8], IndexedControllerSignatures)> {
        let (rest, sc) = b64_count(s)?;
        let (rest, signatures) = count(nomify!(AttachedSignaturePrefix::from_stream_bytes), sc as usize)(rest)?;
        Ok((rest, IndexedControllerSignatures::new(signatures)))
    }

    pub(crate) fn from_stream_group_bytes<'a>(s: &'a [u8]) -> CesrResult<(&'a [u8], IndexedControllerSignatures)> {
        let (rest, payload_type) = take_bytes(s, 2u8)?;
        if payload_type != "-A".as_bytes() {
            return Err(CesrError::IncorrectDigest);
        }
        Self::from_stream_bytes(rest)
    }
}

impl FromStr for IndexedControllerSignatures {
    type Err = CesrError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Serialize for IndexedControllerSignatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for IndexedControllerSignatures {
    fn deserialize<D>(deserializer: D) -> Result<IndexedControllerSignatures, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        IndexedControllerSignatures::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::groups::CesrGroup;
    use crate::primitives::derivation::SelfSigningCode;
    use crate::primitives::prefix::SelfSigningPrefix;
    use super::*;

    #[test]
    fn test_sigs() {
        assert_eq!(
            CesrGroup::from_stream_bytes("-AABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes()).unwrap(),
            ("".as_bytes(),
             CesrGroup::IndexedControllerSignaturesVariant {
                 value: IndexedControllerSignatures::new(
                     vec![AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed25519Sha512, vec![0u8; 64]), 0)])
             })
        );

        assert!(
            CesrGroup::from_stream_bytes("-AABAA0Q7bqPvenjWXo_YIikMBKOg-pghLKwBi1Plm0PEqdv67L1_c6dq9bll7OFnoLp0a74Nw1cBGdjIPcu-yAllHAw".as_bytes()).is_ok());

        assert_eq!(
            CesrGroup::from_stream_bytes("-AACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA0AACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAextra data".as_bytes()).unwrap(),
            ("extra data".as_bytes(),
             CesrGroup::IndexedControllerSignaturesVariant {
                 value:
                 IndexedControllerSignatures::new(vec![
                     AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed25519Sha512, vec![0u8; 64]), 0),
                     AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed448, vec![0u8; 114]), 2),
                 ])
             })
        );

        assert_eq!(
            CesrGroup::from_stream_bytes("-AACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA0AACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes()).unwrap(),
            ("".as_bytes(),
             CesrGroup::IndexedControllerSignaturesVariant {
                 value: IndexedControllerSignatures::new(vec![
                     AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed25519Sha512, vec![0u8; 64]), 0),
                     AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed448, vec![0u8; 114]), 2),
                 ])
             }
            )
        )
    }
}


