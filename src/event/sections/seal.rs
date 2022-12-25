use crate::prefix::{IdentifierPrefix, SelfAddressingPrefix};
use serde::{Deserialize, Serialize};
use serde_hex::{Compact, SerHex};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct EventSeal {
    #[serde(rename = "i")]
    pub prefix: IdentifierPrefix,

    #[serde(rename = "s", with = "SerHex::<Compact>")]
    pub sn: u64,

    #[serde(rename = "d")]
    pub event_digest: SelfAddressingPrefix,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceSeal {
    pub sn: u64,
    pub digest: SelfAddressingPrefix,
}

impl SourceSeal {
    pub fn new(sn: u64, digest: SelfAddressingPrefix) -> Self {
        Self { sn, digest }
    }
}
