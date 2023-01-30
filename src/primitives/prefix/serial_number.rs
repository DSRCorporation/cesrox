use base64::{URL_SAFE, URL_SAFE_NO_PAD};
use std::str::FromStr;

use crate::error::{CesrError, CesrResult};
use crate::primitives::derivation::SerialNumberCode;
use crate::primitives::codes::Codes;
use crate::primitives::prefix::Prefix;
use crate::utils::nom::take_bytes;

#[derive(PartialEq, Eq, Debug)]
pub struct SerialNumberPrefix {
    pub derivation: SerialNumberCode,
    pub value: u64,
}

impl Prefix for SerialNumberPrefix {
    fn derivative(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
    fn derivation_code(&self) -> String {
        "0A".to_string()
    }
}

impl FromStr for SerialNumberPrefix {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let derivation = SerialNumberCode::from_str(s)?;

        let mut sn_array: [u8; 8] = [0; 8];
        sn_array.copy_from_slice(&s[8..].as_bytes());
        let value = u64::from_be_bytes(sn_array);

        Ok(SerialNumberPrefix {
            derivation,
            value,
        })
    }
}

impl SerialNumberPrefix {
    pub fn to_str(sn: u64) -> String {
        let payload_type = Codes::OA;
        let sn_raw: Vec<u8> = sn.to_be_bytes().into();
        // Calculate how many zeros are missing to achieve expected base64 string
        // length. Master code size is expected padding size.
        let missing_zeros =
            payload_type.size() / 4 * 3 - payload_type.master_code_size(false) - sn_raw.len();
        let sn_vec: Vec<u8> = std::iter::repeat(0)
            .take(missing_zeros)
            .chain(sn_raw)
            .collect();
        [
            payload_type.to_string(),
            base64::encode_config(sn_vec, URL_SAFE_NO_PAD),
        ]
            .join("")
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

    pub fn from_stream_bytes(s: &[u8]) -> CesrResult<(&[u8], Self)> {
        let (more, type_c) = take_bytes(s, 2u8)?;

        const A: &[u8] = "0A".as_bytes();

        match type_c {
            A => {
                let (rest, parsed_sn) = take_bytes(more, 22u8)?;

                let sn = {
                    let b64decode = base64::decode_config(parsed_sn, URL_SAFE)?;
                    let mut sn_array: [u8; 8] = [0; 8];
                    sn_array.copy_from_slice(&b64decode[8..]);
                    u64::from_be_bytes(sn_array)
                };

                Ok((rest, Self { derivation: SerialNumberCode, value: sn }))
            }
            _ => Err(CesrError::InvalidState),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_serial_number_prefix() {
        let sn = SerialNumberPrefix::from_stream_bytes("0AAAAAAAAAAAAAAAAAAAAAAw".as_bytes()).unwrap();
        assert_eq!(sn, ("".as_bytes(), SerialNumberPrefix { derivation: SerialNumberCode, value: 3 }));
    }
}