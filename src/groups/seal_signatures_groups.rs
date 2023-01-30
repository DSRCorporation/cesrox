use crate::error::{CesrError, CesrResult};
use crate::groups::counter::Counter;
use crate::groups::indexed_controller_signatures::IndexedControllerSignatures;
use crate::groups::CesrGroup;
use crate::groups::EventSeal;
use crate::nomify;
use crate::primitives::codes::Codes;
use crate::primitives::prefix::{AttachedSignaturePrefix, Prefix};
use nom::multi::count;
use nom::sequence::tuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::primitives::derivation::attached_signature::b64_count;
use crate::primitives::prefix::serial_number::SerialNumberPrefix;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransferableIndexedSignatureGroups {
    pub value: Vec<TransferableIndexedSignatureGroup>,
}

impl TransferableIndexedSignatureGroups {
    pub fn new(
        value: Vec<TransferableIndexedSignatureGroup>,
    ) -> TransferableIndexedSignatureGroups {
        Self { value }
    }

    pub fn to_str(&self) -> String {
        let data = self.value.iter().fold("".into(), |acc, group| {
            [
                acc,
                group.event_seal.prefix.to_str(),
                SerialNumberPrefix::to_str(group.event_seal.sn),
                group.event_seal.event_digest.to_str(),
                // TODO: avoid cloning here
                CesrGroup::IndexedControllerSignatures(IndexedControllerSignatures::new(
                    group.signature_prefixes.to_vec(),
                ))
                    .to_str(),
            ]
                .join("")
        });
        Counter::new(Codes::MF, self.value.len(), data).pack()
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

    pub fn from_stream_bytes<'a>(s: &'a [u8]) -> CesrResult<(&[u8], TransferableIndexedSignatureGroups)> {
        let (rest, sc) = b64_count(s)?;
        let (rest, parsed) = count(
            tuple((nomify!(EventSeal::from_stream_bytes), nomify!(IndexedControllerSignatures::from_stream_group_bytes))),
            sc as usize,
        )(rest)?;
        let signatures_groups = parsed
            .into_iter()
            .map(|(event_seal, signature)| {
                TransferableIndexedSignatureGroup::new(event_seal, signature.value)
            })
            .collect();
        Ok((
            rest,
            TransferableIndexedSignatureGroups::new(signatures_groups),
        ))
    }
}

impl Serialize for TransferableIndexedSignatureGroups {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for TransferableIndexedSignatureGroups {
    fn deserialize<D>(deserializer: D) -> Result<TransferableIndexedSignatureGroups, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        TransferableIndexedSignatureGroups::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferableIndexedSignatureGroup {
    event_seal: EventSeal,
    signature_prefixes: Vec<AttachedSignaturePrefix>,
}

impl TransferableIndexedSignatureGroup {
    pub fn new(
        event_seal: EventSeal,
        signature_prefixes: Vec<AttachedSignaturePrefix>,
    ) -> TransferableIndexedSignatureGroup {
        Self {
            event_seal,
            signature_prefixes,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::groups::{TransferableIndexedSignatureGroup, TransferableIndexedSignatureGroups};
    use super::*;

    #[test]
    fn test_parse_signature_groups() {
        let attached_str = "-FABED9EB3sA5u2vCPOEmX3d7bEyHiSh7Xi8fjew2KMl3FQM0AAAAAAAAAAAAAAAAAAAAAAAEeGqW24EnxUgO_wfuFo6GR_vii-RNv5iGo8ibUrhe6Z0-AABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let (_rest, seal) = CesrGroup::from_bytes(attached_str.as_bytes()).unwrap();

        assert_eq!(
            seal,
            CesrGroup::TransferableIndexedSignaturesGroups(
                TransferableIndexedSignatureGroups {
                    value: vec![
                        TransferableIndexedSignatureGroup {
                            event_seal: EventSeal {
                                prefix: "ED9EB3sA5u2vCPOEmX3d7bEyHiSh7Xi8fjew2KMl3FQM"
                                    .parse()
                                    .unwrap(),
                                sn: 0,
                                event_digest: "EeGqW24EnxUgO_wfuFo6GR_vii-RNv5iGo8ibUrhe6Z0"
                                    .parse()
                                    .unwrap(),
                            },
                            signature_prefixes: vec!["AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".parse().unwrap()],
                        }
                    ]
                }
            )
        );
    }
}