use std::str::{from_utf8, FromStr};

use chrono::{DateTime, FixedOffset, SecondsFormat};

use crate::error::{CesrError, CesrResult};
use crate::primitives::derivation::TimestampCode;
use crate::primitives::prefix::Prefix;
use crate::utils::nom::take_bytes;

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Debug, PartialEq, Eq)]
pub struct TimestampPrefix {
    pub derivation: TimestampCode,
    pub value: Timestamp,
}

impl Prefix for TimestampPrefix {
    fn derivative(&self) -> Vec<u8> {
        self.value.to_string().as_bytes().to_vec()
    }
    fn derivation_code(&self) -> String {
        "1AAG".to_string()
    }
}

impl FromStr for TimestampPrefix {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let derivation = TimestampCode::from_str(s)?;

        let value = Timestamp::from_str(&s[4..])
            .map_err(|_| CesrError::DeserializeError(format!("Unable to parse timestamp")))?;

        Ok(TimestampPrefix {
            derivation,
            value,
        })
    }
}

impl TimestampPrefix {
    pub fn to_str(&self) -> String {
        [
            self.derivation_code(),
            self.value
                .to_rfc3339_opts(SecondsFormat::Micros, false)
                .replace(':', "c")
                .replace('.', "d")
                .replace('+', "p"),
        ]
            .concat()
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
        let (rest, bytes) = take_bytes(s, 4u8)?;

        const A: &[u8] = "1AAG".as_bytes();

        match bytes {
            A => {
                let (rest, parsed) = take_bytes(rest, 32u8)?;

                let timestamp =
                    &from_utf8(parsed)
                        .unwrap()
                        .replace('c', ":")
                        .replace('d', ".")
                        .replace('p', "+");

                let value = Timestamp::parse_from_rfc3339(&timestamp)
                    .map_err(|_| CesrError::DeserializeError(format!("Unable to parse timestamp")))?;

                Ok((
                    rest,
                    TimestampPrefix {
                        derivation: TimestampCode,
                        value,
                    }
                ))
            }
            _ => Err(CesrError::InvalidState),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_serialize_deserialize_timestamp() {
        let expected_str = "1AAG2023-12-25T12c12c12d000000p00c00";
        let timestamp = "2023-12-25T12:12:12.000000+00:00".parse().unwrap();
        let timestamp_prefix = TimestampPrefix {
            derivation: TimestampCode,
            value: timestamp,
        };
        assert_eq!(expected_str, timestamp_prefix.to_str());

        let timestamp_prefix = TimestampPrefix::from_bytes(&expected_str.as_bytes()).unwrap();
        assert_eq!(timestamp_prefix, TimestampPrefix { derivation: TimestampCode, value: timestamp });
    }
}
