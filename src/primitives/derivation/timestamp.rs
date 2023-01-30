use super::DerivationCode;
use crate::error::CesrError;
use core::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TimestampCode;

impl DerivationCode for TimestampCode {
    fn code_len(&self) -> usize {
        4
    }

    fn derivative_b64_len(&self) -> usize {
        32
    }

    fn to_str(&self) -> String {
        "1AAG".into()
    }
}

impl FromStr for TimestampCode {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.get(..4).ok_or(CesrError::IdentifierPresentError)?;

        match code {
            "1AAG" => Ok(TimestampCode),
            _ => Err(CesrError::NotImplementedError),
        }
    }
}
