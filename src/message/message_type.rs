use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::error::{CesrError, CesrResult};

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum ColdCodes {
    // not taken
    Free = 0b000,
    // CountCode Base64
    CtB64 = 0b001,
    // OpCode Base64
    OpB64 = 0b010,
    // JSON Map Event Start
    JSON = 0b011,
    // MGPK Fixed Map Event Start
    MGPK1 = 0b100,
    // CBOR Map Event Start
    CBOR = 0b101,
    // MGPK Big 16 or 32 Map Event Start
    MGPK2 = 0b110,
    // CountCode or OpCode Base2
    CtOpB2 = 0b111,
}

#[derive(Debug)]
pub enum CesrMessageType {
    CESR,
    JSON,
    CBOR,
    MGPK,
}

#[derive(Debug)]
pub enum CesrMessageFormat {
    CtB64,
    OpB64,
    CtOpB2,
}

impl TryFrom<u8> for CesrMessageType {
    type Error = CesrError;

    fn try_from(byte: u8) -> CesrResult<Self> {
        let tritet = byte >> 5;
        let code = FromPrimitive::from_u8(tritet).ok_or_else(|| {
            CesrError::DeserializeError("Unable to parse Message cold start code".to_string())
        })?;
        match code {
            ColdCodes::CtB64 | ColdCodes::OpB64 => Ok(CesrMessageType::CESR),
            ColdCodes::CtOpB2 => Ok(CesrMessageType::CESR),
            ColdCodes::JSON => Ok(CesrMessageType::JSON),
            ColdCodes::MGPK1 | ColdCodes::MGPK2 => Ok(CesrMessageType::MGPK),
            ColdCodes::CBOR => Ok(CesrMessageType::CBOR),
            code => Err(CesrError::DeserializeError(format!(
                "Unsupported Message cold start code: {:?}",
                code
            ))),
        }
    }
}

impl TryFrom<u8> for CesrMessageFormat {
    type Error = CesrError;

    fn try_from(byte: u8) -> CesrResult<Self> {
        let tritet = byte >> 5;
        let code = FromPrimitive::from_u8(tritet).ok_or_else(|| {
            CesrError::DeserializeError("Unable to parse Message cold start code".to_string())
        })?;
        match code {
            ColdCodes::CtB64 => Ok(CesrMessageFormat::CtB64),
            ColdCodes::OpB64 => Ok(CesrMessageFormat::OpB64),
            ColdCodes::CtOpB2 => Ok(CesrMessageFormat::CtOpB2),
            code => Err(CesrError::DeserializeError(format!(
                "Unsupported Message cold start code: {:?}",
                code
            ))),
        }
    }
}
