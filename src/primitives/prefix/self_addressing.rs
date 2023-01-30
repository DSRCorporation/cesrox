use super::Prefix;
use crate::error::{CesrError, CesrResult};
use crate::primitives::derivation::{DerivationCode, SelfAddressingCode};
use base64::decode_config;
use core::{fmt, str::FromStr};
use std::str::from_utf8;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::utils::nom::take_bytes;

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct SelfAddressingPrefix {
    pub derivation: SelfAddressingCode,
    pub digest: Vec<u8>,
}

impl SelfAddressingPrefix {
    pub fn new(code: SelfAddressingCode, digest: Vec<u8>) -> Self {
        Self {
            derivation: code,
            digest,
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

    pub fn from_stream_bytes(s: &[u8]) -> CesrResult<(&[u8], SelfAddressingPrefix)> {
        const EXT: &[u8] = "0".as_bytes();
        let (_, type_c) = take_bytes(s, 1u8)?;

        let count_bytes = match type_c {
            EXT => 2u8,
            _ => 1u8,
        };
        let (rest, code_str) = take_bytes(s, count_bytes)?;

        let code: SelfAddressingCode = String::from_utf8(code_str.to_vec())?.parse()?;

        let (extra, b) = take_bytes(rest, code.derivative_b64_len())?;

        let pref: SelfAddressingPrefix = from_utf8(&[code_str, b].concat())?.parse()?;

        Ok((extra, pref))
    }

    pub fn verify_binding(&self, sed: &[u8]) -> bool {
        self.derivation.digest(sed) == self.digest
    }
}

impl FromStr for SelfAddressingPrefix {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = SelfAddressingCode::from_str(s)?;
        let c_len = code.code_len();
        let p_len = code.prefix_b64_len();
        if s.len() == code.prefix_b64_len() {
            Ok(Self::new(
                code,
                decode_config(&s[c_len..p_len], base64::URL_SAFE)?,
            ))
        } else {
            Err(CesrError::SemanticError(format!(
                "Incorrect Prefix Length: {}",
                s
            )))
        }
    }
}

impl Prefix for SelfAddressingPrefix {
    fn derivative(&self) -> Vec<u8> {
        self.digest.as_slice().to_vec()
    }
    fn derivation_code(&self) -> String {
        self.derivation.to_str()
    }
}

impl fmt::Display for SelfAddressingPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

/// Serde compatible Serialize
impl Serialize for SelfAddressingPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

/// Serde compatible Deserialize
impl<'de> Deserialize<'de> for SelfAddressingPrefix {
    fn deserialize<D>(deserializer: D) -> Result<SelfAddressingPrefix, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        SelfAddressingPrefix::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Default for SelfAddressingPrefix {
    fn default() -> Self {
        Self {
            derivation: SelfAddressingCode::Blake3_256,
            digest: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_self_addressing_prefix() {
        let sap: SelfAddressingPrefix = "EJJR2nmwyYAfSVPzhzS6b5CMZAoTNZH3ULvaU6Z-i0d8"
            .parse()
            .unwrap();
        let str_to_parse = [&sap.to_str(), "more"].join("");
        assert_eq!(
            SelfAddressingPrefix::from_stream_bytes(str_to_parse.as_bytes()).unwrap(),
            ("more".as_bytes(), sap)
        );
    }
}