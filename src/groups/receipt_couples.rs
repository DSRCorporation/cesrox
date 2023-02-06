use crate::error::{CesrError, CesrResult};
use crate::groups::counter::Counter;
use crate::nomify;
use crate::primitives::codes::Codes;
use crate::primitives::prefix::{BasicPrefix, Prefix, SelfSigningPrefix};
use nom::multi::count;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use nom::sequence::tuple;
use crate::primitives::derivation::attached_signature::b64_count;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NontransferableIdentifierReceiptCouples {
    pub value: Vec<NontransferableIdentifierReceiptCouple>,
}

impl NontransferableIdentifierReceiptCouples {
    pub fn new(
        value: Vec<NontransferableIdentifierReceiptCouple>,
    ) -> NontransferableIdentifierReceiptCouples {
        Self { value }
    }

    pub fn to_str(&self) -> String {
        let data = self.value.iter().fold("".into(), |acc, couple| {
            [acc, couple.basic.to_str(), couple.self_signing.to_str()].join("")
        });
        Counter::new(Codes::MC, self.value.len(), data).pack()
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

    pub fn from_stream_bytes<'a>(
        s: &'a [u8],
    ) -> CesrResult<(&'a [u8], NontransferableIdentifierReceiptCouples)> {
        let (rest, sc) = b64_count(s)?;

        let (rest, parsed) = count(
            tuple((
                nomify!(BasicPrefix::from_stream_bytes),
                nomify!(SelfSigningPrefix::from_stream_bytes))
            ),
            sc as usize,
        )(rest)?;
        let couples = parsed
            .into_iter()
            .map(|(basic, self_signing)| {
                NontransferableIdentifierReceiptCouple::new(basic, self_signing)
            })
            .collect();
        Ok((rest, NontransferableIdentifierReceiptCouples::new(couples)))
    }
}

impl Serialize for NontransferableIdentifierReceiptCouples {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for NontransferableIdentifierReceiptCouples {
    fn deserialize<D>(deserializer: D) -> Result<NontransferableIdentifierReceiptCouples, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        NontransferableIdentifierReceiptCouples::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NontransferableIdentifierReceiptCouple {
    pub basic: BasicPrefix,
    pub self_signing: SelfSigningPrefix,
}

impl NontransferableIdentifierReceiptCouple {
    pub fn new(
        basic: BasicPrefix,
        self_signing: SelfSigningPrefix,
    ) -> NontransferableIdentifierReceiptCouple {
        Self {
            basic,
            self_signing,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::groups::CesrGroup;
    use super::*;

    #[test]
    fn test_parse_receipt_couples() {
        let attached_str = "-CABBed2Tpxc8KeCEWoq3_RKKRjU_3P-chSser9J4eAtAK6I0B8npsG58rX1ex73gaGe-jvRnw58RQGsDLzoSXaGn-kHRRNu6Kb44zXDtMnx-_8CjnHqskvDbz6pbEbed3JTOnCQ";
        let a: BasicPrefix = "Bed2Tpxc8KeCEWoq3_RKKRjU_3P-chSser9J4eAtAK6I".parse().unwrap();
        println!("basic {:?}", a.public_key.value);
        let a: SelfSigningPrefix = "0B8npsG58rX1ex73gaGe-jvRnw58RQGsDLzoSXaGn-kHRRNu6Kb44zXDtMnx-_8CjnHqskvDbz6pbEbed3JTOnCQ".parse().unwrap();
        println!("self_signing {:?}", a.signature);


        let (_rest, seal) = CesrGroup::from_stream_bytes(attached_str.as_bytes()).unwrap();

        // assert_eq!(
        //     seal,
        //     CesrGroup::NontransferableIdentifierReceiptCouplesVariant {
        //         value: NontransferableIdentifierReceiptCouples {
        //             value: vec![
        //                 NontransferableIdentifierReceiptCouple {
        //                     basic: "Bed2Tpxc8KeCEWoq3_RKKRjU_3P-chSser9J4eAtAK6I".parse().unwrap(),
        //                     self_signing: "0B8npsG58rX1ex73gaGe-jvRnw58RQGsDLzoSXaGn-kHRRNu6Kb44zXDtMnx-_8CjnHqskvDbz6pbEbed3JTOnCQ".parse().unwrap(),
        //                 }
        //             ]
        //         }
        //     }
        // );
    }
}