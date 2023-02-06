use crate::error::{CesrError, CesrResult};
use crate::groups::counter::Counter;
use crate::nomify;
use crate::primitives::codes::Codes;
use crate::primitives::prefix::{IdentifierPrefix, Prefix, SelfAddressingPrefix};
use nom::multi::count;
use nom::sequence::tuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_hex::{Compact, SerHex};
use crate::primitives::derivation::attached_signature::b64_count;
use crate::primitives::prefix::serial_number::SerialNumberPrefix;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SealSourceCouplets {
    pub value: Vec<SourceSeal>,
}

impl SealSourceCouplets {
    pub fn new(value: Vec<SourceSeal>) -> SealSourceCouplets {
        Self { value }
    }

    pub fn to_str(&self) -> String {
        let data = self.value.iter().fold("".into(), |acc, s| {
            [acc, SerialNumberPrefix::to_str(s.sn), s.digest.to_str()].join("")
        });
        Counter::new(Codes::MG, self.value.len(), data).pack()
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

    pub fn from_stream_bytes<'a>(s: &'a [u8]) -> CesrResult<(&[u8], SealSourceCouplets)> {
        let (rest, sc) = b64_count(s)?;

        let (rest, attachment) = count(
            tuple((
                nomify!(SerialNumberPrefix::from_stream_bytes),
                nomify!(SelfAddressingPrefix::from_stream_bytes))
            ),
            sc as usize,
        )(rest)?;
        let source_seals = attachment
            .into_iter()
            .map(|(sn, digest)| SourceSeal::new(sn.value, digest))
            .collect();

        Ok((rest, SealSourceCouplets::new(source_seals)))
    }
}

impl Serialize for SealSourceCouplets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for SealSourceCouplets {
    fn deserialize<D>(deserializer: D) -> Result<SealSourceCouplets, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        SealSourceCouplets::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct EventSeal {
    #[serde(rename = "i")]
    pub prefix: IdentifierPrefix,

    #[serde(rename = "s", with = "SerHex::<Compact>")]
    pub sn: u64,

    #[serde(rename = "d")]
    pub event_digest: SelfAddressingPrefix,
}

impl EventSeal {
    pub fn new(prefix: IdentifierPrefix, sn: u64, event_digest: SelfAddressingPrefix) -> Self {
        Self {
            prefix,
            sn,
            event_digest,
        }
    }

    pub fn from_stream_bytes(s: &[u8]) -> CesrResult<(&[u8], EventSeal)> {
        let (rest, identifier) = IdentifierPrefix::from_bytes(s)?;
        let (rest, sn) = SerialNumberPrefix::from_stream_bytes(rest)?;
        let (rest, event_digest) = SelfAddressingPrefix::from_stream_bytes(rest)?;
        let seal = EventSeal {
            prefix: identifier,
            sn: sn.value,
            event_digest,
        };
        Ok((rest, seal))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceSeal {
    pub sn: u64,
    pub digest: SelfAddressingPrefix,
}

impl SourceSeal {
    pub fn new(sn: u64, digest: SelfAddressingPrefix) -> Self {
        Self { sn, digest }
    }
}

#[cfg(test)]
mod tests {
    use crate::groups::CesrGroup;
    use super::*;

    #[test]
    fn test_parse_seal_source_couples() {
        let attached_str = "-GAC0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII0AAAAAAAAAAAAAAAAAAAAAAQE3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII";
        let (_rest, group) = CesrGroup::from_stream_bytes(attached_str.as_bytes()).unwrap();
        // assert_eq!(
        //     group,
        //     CesrGroup::SealSourceCoupletsVariant {
        //         value: SealSourceCouplets {
        //             value: vec![
        //                 SourceSeal {
        //                     sn: 1,
        //                     digest: "E3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII"
        //                         .parse()
        //                         .unwrap(),
        //                 },
        //                 SourceSeal {
        //                     sn: 1,
        //                     digest: "E3fUycq1G-P1K1pL2OhvY6ZU-9otSa3hXiCcrxuhjyII"
        //                         .parse()
        //                         .unwrap(),
        //                 },
        //             ]
        //         }
        //     }
        // );
    }
}