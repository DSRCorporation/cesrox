use crate::error::{CesrError, CesrResult};
use crate::primitives::derivation::{DerivationCode, SelfSigningCode};
use crate::primitives::prefix::Prefix;
use base64::{decode_config, URL_SAFE};
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::utils::nom::take_bytes;

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct SelfSigningPrefix {
    pub derivation: SelfSigningCode,
    pub signature: Vec<u8>,
}

impl SelfSigningPrefix {
    pub fn new(code: SelfSigningCode, signature: Vec<u8>) -> Self {
        Self {
            derivation: code,
            signature,
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

    pub fn from_stream_bytes(s: &[u8]) -> CesrResult<(&[u8], SelfSigningPrefix)> {
        const EXT: &[u8] = "1".as_bytes();

        let (_, type_c) = take_bytes(s, 1u8)?;

        let count_bytes = match type_c {
            EXT => 4u8,
            _ => 2u8,
        };
        let (rest, code_str) = take_bytes(s, count_bytes)?;

        let code: SelfSigningCode = String::from_utf8(code_str.to_vec())?.parse()?;

        let (extra, b) = take_bytes(rest, code.derivative_b64_len())?;

        let sig = base64::decode_config(b, URL_SAFE)?;
        Ok((extra, code.derive(sig)))
    }
}

impl FromStr for SelfSigningPrefix {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = SelfSigningCode::from_str(s)?;

        if s.len() != code.prefix_b64_len() {
            return Err(CesrError::SemanticError(format!("Incorrect Prefix Length: {}", s)));
        }

        Ok(Self::new(
            code,
            decode_config(&s[code.code_len()..code.prefix_b64_len()], base64::URL_SAFE)?,
        ))
    }
}

impl Prefix for SelfSigningPrefix {
    fn derivative(&self) -> Vec<u8> {
        self.signature.as_slice().to_vec()
    }
    fn derivation_code(&self) -> String {
        self.derivation.to_str()
    }
}

/// Serde compatible Serialize
impl Serialize for SelfSigningPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

/// Serde compatible Deserialize
impl<'de> Deserialize<'de> for SelfSigningPrefix {
    fn deserialize<D>(deserializer: D) -> Result<SelfSigningPrefix, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        SelfSigningPrefix::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_self_signing_prefix() {
        let sig_prefix: SelfSigningPrefix =
            "0Bq1UBr1QD5TokdcnO_FmnoYsd8rB4_-oaQtk0dfFSSXPcxAu7pSaQIVfkhzckCVmTIgrdxyXS21uZgs7NxoyZAQ"
                .parse()
                .unwrap();
        let string_to_parse = [&sig_prefix.to_str(), "more"].join("");

        assert_eq!(
            SelfSigningPrefix::from_stream_bytes(string_to_parse.as_bytes()).unwrap(),
            ("more".as_bytes(), sig_prefix.clone())
        );
    }
}
