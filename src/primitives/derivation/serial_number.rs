use super::DerivationCode;
use crate::error::CesrError;
use core::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SerialNumberCode;

impl DerivationCode for SerialNumberCode {
    fn code_len(&self) -> usize {
        2
    }

    fn derivative_b64_len(&self) -> usize {
        22
    }

    fn to_str(&self) -> String {
        "0A".into()
    }
}

impl FromStr for SerialNumberCode {
    type Err = CesrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.get(..2).ok_or(CesrError::IdentifierPresentError)?;

        match code {
            "0A" => Ok(SerialNumberCode),
            _ => Err(CesrError::NotImplementedError),
        }
    }
}
