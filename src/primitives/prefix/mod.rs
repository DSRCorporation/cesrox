use base64::encode_config;
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod attached_signature;
pub mod basic;
pub mod seed;
pub mod self_addressing;
pub mod self_signing;
pub mod serial_number;
pub mod timestamp;

use crate::error::{CesrError, CesrResult};
pub use attached_signature::AttachedSignaturePrefix;
pub use basic::BasicPrefix;
pub use seed::SeedPrefix;
pub use self_addressing::SelfAddressingPrefix;
pub use self_signing::SelfSigningPrefix;

pub trait Prefix: FromStr<Err=CesrError> {
    fn derivative(&self) -> Vec<u8>;
    fn derivation_code(&self) -> String;
    fn to_str(&self) -> String {
        // empty data cannot be prefixed!
        match self.derivative().len() {
            0 => "".to_string(),
            _ => {
                let dc = self.derivation_code();
                let ec = encode_config(self.derivative(), base64::URL_SAFE_NO_PAD);
                [dc, ec].join("")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdentifierPrefix {
    BasicIdentifier { value: BasicPrefix },
    SelfAddressingIdentifier { value: SelfAddressingPrefix },
    SelfSigningIdentifier { value: SelfSigningPrefix },
}

impl FromStr for IdentifierPrefix {
    type Err = CesrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BasicPrefix::from_str(s) {
            Ok(bp) => Ok(Self::BasicIdentifier { value: bp }),
            Err(err) => {
                match err {
                    CesrError::Base64DecodingError { source: _ } => return Err(err),
                    _ => (),
                }
                match SelfAddressingPrefix::from_str(s) {
                    Ok(sa) => Ok(Self::SelfAddressingIdentifier { value: sa }),
                    Err(_) => Ok(Self::SelfSigningIdentifier {
                        value: SelfSigningPrefix::from_str(s)?,
                    }),
                }
            }
        }
    }
}

impl Prefix for IdentifierPrefix {
    fn derivative(&self) -> Vec<u8> {
        match self {
            IdentifierPrefix::BasicIdentifier { value: bp } => bp.derivative().to_vec(),
            IdentifierPrefix::SelfAddressingIdentifier { value: sap } => sap.derivative().to_vec(),
            IdentifierPrefix::SelfSigningIdentifier { value: ssp } => ssp.derivative().to_vec(),
        }
    }

    fn derivation_code(&self) -> String {
        match self {
            IdentifierPrefix::BasicIdentifier { value: bp } => bp.derivation_code(),
            IdentifierPrefix::SelfAddressingIdentifier { value: sap } => sap.derivation_code(),
            IdentifierPrefix::SelfSigningIdentifier { value: ssp } => ssp.derivation_code(),
        }
    }
}

/// Serde compatible Serialize
impl Serialize for IdentifierPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

/// Serde compatible Deserialize
impl<'de> Deserialize<'de> for IdentifierPrefix {
    fn deserialize<D>(deserializer: D) -> Result<IdentifierPrefix, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        IdentifierPrefix::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Default for IdentifierPrefix {
    fn default() -> Self {
        IdentifierPrefix::SelfAddressingIdentifier {
            value: SelfAddressingPrefix::default(),
        }
    }
}

impl IdentifierPrefix {
    /// extracts Identifier prefix
    pub fn from_bytes(s: &[u8]) -> CesrResult<(&[u8], IdentifierPrefix)> {
        let (rest, identifier) = match SelfAddressingPrefix::from_stream_bytes(s) {
            Ok(sap) => Ok((
                sap.0,
                IdentifierPrefix::SelfAddressingIdentifier { value: sap.1 },
            )),
            Err(_) => match BasicPrefix::from_stream_bytes(s) {
                Ok(bp) => Ok((bp.0, IdentifierPrefix::BasicIdentifier { value: bp.1 })),
                Err(e) => Err(e),
            },
        }?;
        Ok((rest, identifier))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::key::Key;
    use crate::primitives::derivation::{BasicCode, SelfAddressingCode, SelfSigningCode};

    #[test]
    fn simple_deserialize() -> Result<(), CesrError> {
        let pref: IdentifierPrefix = "BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".parse()?;

        assert_eq!(pref.derivation_code(), "B");

        assert_eq!(pref.derivative().len(), 32);

        assert_eq!(pref.derivative().to_vec(), vec![0u8; 32]);

        Ok(())
    }

    #[test]
    fn length() -> Result<(), CesrError> {
        // correct
        assert!(IdentifierPrefix::from_str("BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").is_ok());
        assert!(IdentifierPrefix::from_str("CBBBBBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").is_ok());

        // too short
        assert!(!IdentifierPrefix::from_str("BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").is_ok());

        // too long
        assert!(
            !IdentifierPrefix::from_str("BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").is_ok()
        );

        // not a real prefix
        assert!(
            match IdentifierPrefix::from_str("ZAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
                .unwrap_err()
            {
                CesrError::DeserializeError(msg) => msg.contains("Unknown master code"),
                _ => false,
            }
        );

        // not base 64 URL
        assert!(
            match IdentifierPrefix::from_str("BAAAAAAAAAAAAAAAAAAA/AAAAAAAAAAAAAAAAAAAAAAA")
                .unwrap_err()
            {
                CesrError::Base64DecodingError { source: _ } => true,
                _ => false,
            }
        );

        Ok(())
    }

    #[test]
    fn simple_serialize() -> Result<(), CesrError> {
        let pref = BasicCode::Ed25519NT.derive(Key::new(
            &ed25519_dalek::PublicKey::from_bytes(&[0; 32])?
                .to_bytes(),
        ));

        assert_eq!(
            pref.to_str(),
            "BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        );

        Ok(())
    }

    #[test]
    fn prefix_deserialization() -> Result<(), CesrError> {
        /// Helper function that checks whether all codes fulfill the condition
        /// given by predicate `pred`.
        fn all_codes<F>(codes: Vec<(&str, usize)>, pred: F) -> Result<(), CesrError>
            where
                F: Fn(IdentifierPrefix) -> bool,
        {
            for (code, length) in codes {
                let pref: IdentifierPrefix =
                    [code.to_string(), "A".repeat(length)].join("").parse()?;
                assert!(pred(pref.clone()));
                assert_eq!(pref.derivation_code(), code);
            }
            Ok(())
        }

        // All codes that are mapped to `BasicPrefix`.
        let basic_codes = vec!["B", "C", "D", "L", "1AAA", "1AAB", "1AAC", "1AAD"].into_iter();
        // Allowed string lengths for respective basic codes.
        let allowed_lengths = vec![43, 43, 43, 75, 47, 47, 76, 76].into_iter();
        let is_basic = |identifier| matches!(&identifier, IdentifierPrefix::BasicIdentifier{..});
        all_codes(basic_codes.zip(allowed_lengths).collect(), is_basic)?;

        // All codes that are mapped to `SelfAddressingPrefix`.
        let self_adressing_codes =
            vec!["E", "F", "G", "H", "I", "0D", "0E", "0F", "0G"].into_iter();
        // Allowed string lengths for respective self addressing codes.
        let allowed_lengths = vec![43, 43, 43, 43, 43, 86, 86, 86, 86].into_iter();
        let is_self_addresing =
            |identifier| matches!(&identifier, IdentifierPrefix::SelfAddressingIdentifier{..});
        all_codes(
            self_adressing_codes.zip(allowed_lengths).collect(),
            is_self_addresing,
        )?;

        // All codes that are mapped to `SelfSigningPrefix`.
        let is_self_signing = |identifier| matches!(&identifier, IdentifierPrefix::SelfSigningIdentifier{..});
        // Allowed string lengths for respective self signing codes.
        let self_signing_codes = vec!["0B", "0C", "1AAE"].into_iter();
        let allowed_lengths = vec![86, 86, 152].into_iter();
        all_codes(
            self_signing_codes.zip(allowed_lengths).collect(),
            is_self_signing,
        )?;

        Ok(())
    }

    #[test]
    fn prefix_serialization() -> Result<(), CesrError> {
        // The lengths of respective vectors are choosen according to [0, Section 14.2]
        // [0]: https://github.com/SmithSamuelM/Papers/raw/master/whitepapers/KERI_WP_2.x.web.pdf

        // Test BasicPrefix serialization.
        assert_eq!(
            BasicPrefix::new(
                BasicCode::Ed25519NT,
                Key::new(
                    &ed25519_dalek::PublicKey::from_bytes(&[0; 32])?
                        .to_bytes()
                ),
            )
                .to_str(),
            ["B".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            BasicPrefix::new(
                BasicCode::X25519,
                Key::new(
                    &ed25519_dalek::PublicKey::from_bytes(&[0; 32])?
                        .to_bytes()
                ),
            )
                .to_str(),
            ["C".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            BasicPrefix::new(
                BasicCode::Ed25519,
                Key::new(
                    &ed25519_dalek::PublicKey::from_bytes(&[0; 32])?
                        .to_bytes()
                ),
            )
                .to_str(),
            ["D".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            BasicPrefix::new(BasicCode::X448, Key::new(&[0; 56])).to_str(),
            ["L".to_string(), "A".repeat(75)].join("")
        );
        assert_eq!(
            BasicPrefix::new(BasicCode::ECDSAsecp256k1NT, Key::new(&[0; 33])).to_str(),
            ["1AAA".to_string(), "A".repeat(44)].join("")
        );
        assert_eq!(
            BasicPrefix::new(BasicCode::ECDSAsecp256k1, Key::new(&[0; 33])).to_str(),
            ["1AAB".to_string(), "A".repeat(44)].join("")
        );
        assert_eq!(
            BasicPrefix::new(BasicCode::Ed448NT, Key::new(&[0; 57])).to_str(),
            ["1AAC".to_string(), "A".repeat(76)].join("")
        );
        assert_eq!(
            BasicPrefix::new(BasicCode::Ed448, Key::new(&[0; 57])).to_str(),
            ["1AAD".to_string(), "A".repeat(76)].join("")
        );

        // Test SelfAddressingPrefix serialization.
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::Blake3_256, vec![0; 32]).to_str(),
            ["E".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::Blake2B256 { key: vec![] }, vec![0; 32]).to_str(),
            ["F".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::Blake2S256 { key: vec![] }, vec![0; 32]).to_str(),
            ["G".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::SHA3_256, vec![0; 32]).to_str(),
            ["H".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::SHA2_256, vec![0; 32]).to_str(),
            ["I".to_string(), "A".repeat(43)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::Blake3_512, vec![0; 64]).to_str(),
            ["0D".to_string(), "A".repeat(86)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::SHA3_512, vec![0; 64]).to_str(),
            ["0E".to_string(), "A".repeat(86)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::Blake2B512, vec![0; 64]).to_str(),
            ["0F".to_string(), "A".repeat(86)].join("")
        );
        assert_eq!(
            SelfAddressingPrefix::new(SelfAddressingCode::SHA2_512, vec![0; 64]).to_str(),
            ["0G".to_string(), "A".repeat(86)].join("")
        );

        // Test SelfSigningPrefix serialization.
        assert_eq!(
            SelfSigningPrefix::new(SelfSigningCode::ECDSAsecp256k1Sha256, vec![0; 64]).to_str(),
            ["0C".to_string(), "A".repeat(86)].join("")
        );
        assert_eq!(
            SelfSigningPrefix::new(SelfSigningCode::Ed25519Sha512, vec![0; 64]).to_str(),
            ["0B".to_string(), "A".repeat(86)].join("")
        );
        assert_eq!(
            SelfSigningPrefix::new(SelfSigningCode::Ed448, vec![0; 114]).to_str(),
            ["1AAE".to_string(), "A".repeat(152)].join("")
        );

        Ok(())
    }
}
