use crate::error::{CesrError, CesrResult};
use crate::primitives::derivation::{BasicCode, DerivationCode};
use crate::primitives::key::Key;
use crate::primitives::prefix::Prefix;
use base64::{decode_config, URL_SAFE};
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::utils::nom::take_bytes;

#[derive(Debug, Clone)]
pub struct BasicPrefix {
    pub derivation: BasicCode,
    pub public_key: Key,
}

impl BasicPrefix {
    pub fn new(code: BasicCode, public_key: Key) -> Self {
        Self {
            derivation: code,
            public_key,
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

    pub fn from_stream_bytes(s: &[u8]) -> CesrResult<(&[u8], BasicPrefix)> {
        const EXT: &[u8] = "1".as_bytes();

        let (_, type_c) = take_bytes(s, 1u8)?;

        let count_bytes = match type_c {
            EXT => 4u8,
            _ => 1u8,
        };
        let (rest, code_str) = take_bytes(s, count_bytes)?;

        let code: BasicCode = String::from_utf8(code_str.to_vec())?.parse()?;

        let (extra, b) = take_bytes(rest, code.derivative_b64_len())?;

        let key = base64::decode_config(b.to_vec(), URL_SAFE)?;

        let pk = Key::new(key.as_slice());
        Ok((extra, code.derive(pk)))
    }
}

impl PartialEq for BasicPrefix {
    fn eq(&self, other: &Self) -> bool {
        self.derivation == other.derivation && self.public_key == other.public_key
    }
}

impl FromStr for BasicPrefix {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = BasicCode::from_str(s)?;

        if s.len() == code.prefix_b64_len() {
            let k_vec =
                decode_config(&s[code.code_len()..code.prefix_b64_len()], base64::URL_SAFE)?;
            Ok(Self::new(code, Key::new(k_vec.as_slice())))
        } else {
            Err(CesrError::SemanticError(format!(
                "Incorrect Prefix Length: {}",
                s
            )))
        }
    }
}

impl Prefix for BasicPrefix {
    fn derivative(&self) -> Vec<u8> {
        self.public_key.value().to_vec()
    }
    fn derivation_code(&self) -> String {
        self.derivation.to_str()
    }
}

/// Serde compatible Serialize
impl Serialize for BasicPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

/// Serde compatible Deserialize
impl<'de> Deserialize<'de> for BasicPrefix {
    fn deserialize<D>(deserializer: D) -> Result<BasicPrefix, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        BasicPrefix::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let kp = Keypair::generate(&mut OsRng);

        let bp = BasicPrefix {
            derivation: BasicCode::Ed25519,
            public_key: Key::new(&kp.public.to_bytes()),
        };

        let serialized = serde_json::to_string(&bp);
        assert!(serialized.is_ok());

        let deserialized = serde_json::from_str(&serialized.unwrap());

        assert!(deserialized.is_ok());
        assert_eq!(bp, deserialized.unwrap());
    }

    #[test]
    fn to_from_string() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let kp = Keypair::generate(&mut OsRng);
        println!("{:?}", kp.public.to_bytes());

        let bp = BasicPrefix {
            derivation: BasicCode::Ed25519,
            public_key: Key::new(&kp.public.to_bytes()),
        };

        let string = bp.to_str();

        let from_str = BasicPrefix::from_str(&string);

        assert!(from_str.is_ok());
        let deser = from_str.unwrap();
        assert_eq!(bp, deser);
    }
}