use crate::error::{CesrError, CesrResult};
use crate::primitives::derivation::attached_signature::{b64_to_num, AttachedSignatureCode};
use crate::primitives::derivation::{DerivationCode, SelfSigningCode};
use crate::primitives::prefix::{Prefix, SelfSigningPrefix};
use base64::decode_config;
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::utils::nom::take_bytes;

#[derive(Debug, PartialEq, Clone)]
pub struct AttachedSignaturePrefix {
    pub index: u16,
    pub signature: SelfSigningPrefix,
}

impl AttachedSignaturePrefix {
    pub fn new(signature: SelfSigningPrefix, index: u16) -> Self {
        Self {
            signature,
            index,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_str().as_bytes().to_vec()
    }

    pub fn from_bytes(s: &[u8]) -> CesrResult<Self> {
        let (rest, parsed) = Self::from_stream_bytes(s)?;
        if !rest.is_empty() {
            return Err(CesrError::TooMuch);
        }
        Ok(parsed)
    }

    // TODO this could be a lot nicer, but is currently written to be careful and "easy" to follow
    pub fn from_stream_bytes(s: &[u8]) -> CesrResult<(&[u8], AttachedSignaturePrefix)> {
        let (more, type_c) = take_bytes(s, 1u8)?;

        const A: &[u8] = "A".as_bytes();
        const B: &[u8] = "B".as_bytes();
        const Z: &[u8] = "0".as_bytes();

        match type_c {
            A => {
                let (maybe_sig, index_c) = take_bytes(more, 1u8)?;

                let index = b64_to_num(index_c)?;

                let (rest, sig_s) = take_bytes(maybe_sig, 86u8)?;

                let sig = base64::decode_config(sig_s, base64::URL_SAFE)?;

                Ok((
                    rest,
                    AttachedSignaturePrefix::new(
                        SelfSigningPrefix::new(SelfSigningCode::Ed25519Sha512, sig),
                        index
                    ),
                ))
            }
            B => {
                let (maybe_sig, index_c) = take_bytes(more, 1u8)?;

                let index = b64_to_num(index_c)?;

                let (rest, sig_s) = take_bytes(maybe_sig, 86u8)?;

                let sig = base64::decode_config(sig_s, base64::URL_SAFE)?;

                Ok((
                    rest,
                    AttachedSignaturePrefix::new(
                        SelfSigningPrefix::new(SelfSigningCode::ECDSAsecp256k1Sha256, sig),
                        index),
                ))
            }
            Z => {
                let (maybe_count, type_c_2) = take_bytes(more, 1u8)?;
                match type_c_2 {
                    A => {
                        let (maybe_sig, index_c) = take_bytes(maybe_count, 2u8)?;

                        let index = b64_to_num(index_c)?;

                        let (rest, sig_s) = take_bytes(maybe_sig, 152u8)?;

                        let sig = base64::decode_config(sig_s, base64::URL_SAFE)?;

                        Ok((
                            rest,
                            AttachedSignaturePrefix::new(
                                SelfSigningPrefix::new(SelfSigningCode::Ed448, sig),
                                index
                            ),
                        ))
                    }
                    _ => Err(CesrError::InvalidState),
                }
            }
            _ => Err(CesrError::InvalidState),
        }
    }
}

impl FromStr for AttachedSignaturePrefix {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = AttachedSignatureCode::from_str(s)?;

        if (s.len()) == code.prefix_b64_len() {
            Ok(Self::new(
                SelfSigningPrefix::new(code.code,decode_config(&s[code.code_len()..code.prefix_b64_len()], base64::URL_SAFE)?),
                code.index,
            ))
        } else {
            Err(CesrError::SemanticError(format!(
                "Incorrect Prefix Length: {}",
                s
            )))
        }
    }
}

impl Prefix for AttachedSignaturePrefix {
    fn derivative(&self) -> Vec<u8> {
        self.signature.signature.as_slice().to_vec()
    }
    fn derivation_code(&self) -> String {
        AttachedSignatureCode::new(self.signature.derivation, self.index).to_str()
    }
}

/// Serde compatible Serialize
impl Serialize for AttachedSignaturePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

/// Serde compatible Deserialize
impl<'de> Deserialize<'de> for AttachedSignaturePrefix {
    fn deserialize<D>(deserializer: D) -> Result<AttachedSignaturePrefix, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        AttachedSignaturePrefix::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attached_signature_prefix() {
        assert_eq!(
            AttachedSignaturePrefix::from_stream_bytes("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes()).unwrap(),
            ("".as_bytes(),
             AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed25519Sha512, vec![0u8; 64]), 0))
        );

        assert_eq!(
            AttachedSignaturePrefix::from_stream_bytes("BCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes()).unwrap(),
            ("AA".as_bytes(), AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::ECDSAsecp256k1Sha256, vec![0u8; 64]), 2))
        );
    }

    #[test]
    fn deserialize() -> Result<(), CesrError> {
        let attached_ed_1 = "ABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let attached_secp_2 = "BCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let attached_448_3 = "0AADAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

        let pref_ed_1 = AttachedSignaturePrefix::from_str(attached_ed_1)?;
        let pref_secp_2 = AttachedSignaturePrefix::from_str(attached_secp_2)?;
        let pref_448_3 = AttachedSignaturePrefix::from_str(attached_448_3)?;

        assert_eq!(1, pref_ed_1.index);
        assert_eq!(2, pref_secp_2.index);
        assert_eq!(3, pref_448_3.index);

        assert_eq!(SelfSigningCode::Ed25519Sha512, pref_ed_1.signature.derivation);
        assert_eq!(
            SelfSigningCode::ECDSAsecp256k1Sha256,
            pref_secp_2.signature.derivation
        );
        assert_eq!(SelfSigningCode::Ed448, pref_448_3.signature.derivation);
        Ok(())
    }

    #[test]
    fn serialize() -> Result<(), CesrError> {
        let pref_ed_2 = AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed25519Sha512, vec![0u8; 64]), 2);
        let pref_secp_6 =
            AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::ECDSAsecp256k1Sha256, vec![0u8; 64]), 6);
        let pref_448_4 = AttachedSignaturePrefix::new(SelfSigningPrefix::new(SelfSigningCode::Ed448, vec![0u8; 114]), 4);

        assert_eq!(88, pref_ed_2.to_str().len());
        assert_eq!(88, pref_secp_6.to_str().len());
        assert_eq!(156, pref_448_4.to_str().len());

        assert_eq!("ACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", pref_ed_2.to_str());
        assert_eq!("BGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", pref_secp_6.to_str());
        assert_eq!("0AAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", pref_448_4.to_str());
        Ok(())
    }
}
