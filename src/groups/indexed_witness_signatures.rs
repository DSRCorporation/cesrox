use crate::error::{CesrError, CesrResult};
use crate::groups::counter::Counter;
use crate::nomify;
use crate::primitives::codes::Codes;
use crate::primitives::prefix::{AttachedSignaturePrefix, Prefix};
use nom::multi::count;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use crate::primitives::derivation::attached_signature::b64_count;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IndexedWitnessSignatures {
    pub value: Vec<AttachedSignaturePrefix>,
}

impl IndexedWitnessSignatures {
    pub fn new(value: Vec<AttachedSignaturePrefix>) -> IndexedWitnessSignatures {
        Self { value }
    }

    pub fn to_str(&self) -> String {
        let data = self
            .value
            .iter()
            .fold("".into(), |acc, sig| [acc, sig.to_str()].join(""));
        Counter::new(Codes::MB, self.value.len(), data).pack()
    }

    pub fn from_str(s: &str) -> CesrResult<Self> {
        Self::from_bytes(s.as_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_str().as_bytes().to_vec()
    }

    pub fn from_bytes<'a>(s: &'a[u8]) -> CesrResult<Self> {
        let (rest, parsed) = Self::from_stream_bytes(s)?;
        if !rest.is_empty() {
            return Err(CesrError::NotImplementedError);
        }
        Ok(parsed)
    }

    pub fn from_stream_bytes<'a>(s: &'a [u8]) -> CesrResult<(&'a [u8], IndexedWitnessSignatures)> {
        let (rest, sc) = b64_count(s)?;
        let (rest, signatures) = count(nomify!(AttachedSignaturePrefix::from_stream_bytes), sc as usize)(rest)?;
        Ok((rest, IndexedWitnessSignatures::new(signatures)))
    }
}

impl FromStr for IndexedWitnessSignatures {
    type Err = CesrError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Serialize for IndexedWitnessSignatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for IndexedWitnessSignatures {
    fn deserialize<D>(deserializer: D) -> Result<IndexedWitnessSignatures, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        IndexedWitnessSignatures::from_str(&s).map_err(serde::de::Error::custom)
    }
}
