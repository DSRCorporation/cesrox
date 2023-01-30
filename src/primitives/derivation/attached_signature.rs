use crate::error::{CesrError, CesrResult};
use base64::{decode_config, encode_config};
use core::str::FromStr;
use crate::primitives::derivation::{DerivationCode, SelfSigningCode};
use crate::utils::nom::take_bytes;

/// Attached Signature Derivation Codes
///
/// A self signing prefix primitives.derivation outputs a signature as its derivative (2.3.5)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AttachedSignatureCode {
    pub index: u16,
    pub code: SelfSigningCode,
}

impl AttachedSignatureCode {
    pub fn new(code: SelfSigningCode, index: u16) -> Self {
        Self { index, code }
    }
}

impl DerivationCode for AttachedSignatureCode {
    fn code_len(&self) -> usize {
        match self.code {
            SelfSigningCode::Ed25519Sha512 | SelfSigningCode::ECDSAsecp256k1Sha256 => 2,
            SelfSigningCode::Ed448 => 4,
        }
    }

    fn derivative_b64_len(&self) -> usize {
        match self.code {
            SelfSigningCode::Ed25519Sha512 | SelfSigningCode::ECDSAsecp256k1Sha256 => 86,
            SelfSigningCode::Ed448 => 152,
        }
    }

    // TODO, this will only work with indicies up to 63
    fn to_str(&self) -> String {
        [
            match self.code {
                SelfSigningCode::Ed25519Sha512 => "A",
                SelfSigningCode::ECDSAsecp256k1Sha256 => "B",
                SelfSigningCode::Ed448 => "0AA",
            },
            &num_to_b64(self.index),
        ]
        .join("")
    }
}

impl FromStr for AttachedSignatureCode {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..1] {
            "A" => Ok(Self::new(
                SelfSigningCode::Ed25519Sha512,
                b64_to_num(&s.as_bytes()[1..2])?,
            )),
            "B" => Ok(Self::new(
                SelfSigningCode::ECDSAsecp256k1Sha256,
                b64_to_num(&s.as_bytes()[1..2])?,
            )),
            "0" => match &s[1..3] {
                "AA" => Ok(Self::new(
                    SelfSigningCode::Ed448,
                    b64_to_num(&s.as_bytes()[3..4])?,
                )),
                _ => Err(CesrError::DeserializeError("Unknows signature code".into())),
            },
            _ => Err(CesrError::DeserializeError(
                "Unknown attachment code".into(),
            )),
        }
    }
}

// returns the u16 from the lowest 2 bytes of the b64 string
// currently only works for strings 4 chars or less
pub fn b64_to_num(b64: &[u8]) -> CesrResult<u16> {
    let slice = decode_config(
        match b64.len() {
            1 => [r"AAA".as_bytes(), b64].concat(),
            2 => [r"AA".as_bytes(), b64].concat(),
            _ => b64.to_owned(),
        },
        base64::URL_SAFE,
    )
    .map_err(|e| CesrError::Base64DecodingError { source: e })?;
    let len = slice.len();

    Ok(u16::from_be_bytes(match len {
        0 => [0u8; 2],
        1 => [0, slice[0]],
        _ => [slice[len - 2], slice[len - 1]],
    }))
}

pub(crate) fn b64_count<'a>(s: &[u8]) -> CesrResult<(&[u8], u16)> {
    let (more, rest) = take_bytes(s, 2u8)?;
    let bytes = b64_to_num(rest)?;
    Ok((more, bytes))
}

pub fn num_to_b64(num: u16) -> String {
    match num {
        n if n < 63 => {
            encode_config([num.to_be_bytes()[1] << 2], base64::URL_SAFE_NO_PAD)[..1].to_string()
        }
        n if n < 4095 => encode_config(num.to_be_bytes(), base64::URL_SAFE_NO_PAD)[..2].to_string(),
        _ => encode_config(num.to_be_bytes(), base64::URL_SAFE_NO_PAD),
    }
}

#[test]
fn num_to_b64_test() {
    assert_eq!("A", num_to_b64(0));
    assert_eq!("B", num_to_b64(1));
    assert_eq!("C", num_to_b64(2));
    assert_eq!("D", num_to_b64(3));
    assert_eq!("b", num_to_b64(27));
    assert_eq!("AE", num_to_b64(64));
    assert_eq!("EAA", num_to_b64(4096));
}

#[test]
fn test_b64_count() {
    assert_eq!(b64_count("AA".as_bytes()).unwrap(), ("".as_bytes(), 0u16));
    assert_eq!(b64_count("BA".as_bytes()).unwrap(), ("".as_bytes(), 64u16));
    assert_eq!(
        b64_count("ABextra data and stuff".as_bytes()).unwrap(),
        ("extra data and stuff".as_bytes(), 1u16)
    );
}