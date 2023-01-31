use nom::multi::count;
use nom::sequence::tuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

use crate::error::{CesrError, CesrResult};
use crate::groups::counter::Counter;
use crate::groups::indexed_controller_signatures::IndexedControllerSignatures;
use crate::groups::CesrGroup;
use crate::nomify;
use crate::primitives::codes::Codes;
use crate::primitives::derivation::attached_signature::b64_count;
use crate::primitives::prefix::{AttachedSignaturePrefix, IdentifierPrefix, Prefix};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LastEstSignaturesGroups {
    pub value: Vec<LastEstSignaturesGroup>,
}

impl LastEstSignaturesGroups {
    pub fn new(value: Vec<LastEstSignaturesGroup>) -> LastEstSignaturesGroups {
        Self { value }
    }

    pub fn to_str(&self) -> String {
        let data = self.value.iter().fold("".to_string(), |acc, item| {
            [
                acc,
                item.identifier_prefix.to_str(),
                // TODO: avoid cloning here
                CesrGroup::IndexedControllerSignaturesVariant {
                    value: IndexedControllerSignatures {
                        value: item.attached_signature_prefixes.clone(),
                    }
                }
                    .to_str(),
            ]
                .concat()
        });
        Counter::new(Codes::MH, self.value.len(), data).pack()
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

    pub fn from_stream_bytes<'a>(s: &'a [u8]) -> CesrResult<(&'a [u8], LastEstSignaturesGroups)> {
        let (rest, sc) = b64_count(s)?;
        let (rest, parsed) = count(
            tuple((nomify!(IdentifierPrefix::from_bytes), nomify!(IndexedControllerSignatures::from_stream_group_bytes))),
            sc as usize,
        )(rest)?;
        let signatures_groups = parsed
            .into_iter()
            .map(|(identifier_prefix, attached_signature_prefixes)| {
                LastEstSignaturesGroup::new(identifier_prefix, attached_signature_prefixes.value)
            })
            .collect();
        Ok((rest, LastEstSignaturesGroups::new(signatures_groups)))
    }
}

impl FromStr for LastEstSignaturesGroups {
    type Err = CesrError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Serialize for LastEstSignaturesGroups {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for LastEstSignaturesGroups {
    fn deserialize<D>(deserializer: D) -> Result<LastEstSignaturesGroups, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        LastEstSignaturesGroups::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LastEstSignaturesGroup {
    pub identifier_prefix: IdentifierPrefix,
    pub attached_signature_prefixes: Vec<AttachedSignaturePrefix>,
}

impl LastEstSignaturesGroup {
    pub fn new(
        identifier_prefix: IdentifierPrefix,
        attached_signature_prefixes: Vec<AttachedSignaturePrefix>,
    ) -> LastEstSignaturesGroup {
        Self {
            identifier_prefix,
            attached_signature_prefixes,
        }
    }
}
