use rmp_serde as serde_mgpk;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::io::Cursor;

use crate::error::{CesrError, CesrResult};

#[derive(Debug)]
pub struct CustomMessage {
    pub value: JsonValue,
}

impl CustomMessage {
    pub fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        serde_json::to_vec(&self.value).map_err(CesrError::from)
    }

    pub fn to_str(&self) -> CesrResult<String> {
        serde_json::to_string(&self.value).map_err(CesrError::from)
    }

    pub fn to_typed_message<'de, D: DeserializeOwned>(&self) -> CesrResult<D> {
        serde_json::from_value::<D>(self.value.to_owned()).map_err(CesrError::from)
    }

    pub fn from_json(s: &[u8]) -> CesrResult<CustomMessage> {
        let value = serde_json::from_slice(s)
            .map_err(|_| CesrError::SerializationError(format!("Unable to deserialize message from json")))?;
        Ok(CustomMessage { value })
    }

    pub fn from_json_stream(s: &[u8]) -> CesrResult<(CustomMessage, &[u8], &[u8])> {
        let mut stream = serde_json::Deserializer::from_slice(s).into_iter::<JsonValue>();
        match stream.next() {
            Some(Ok(value)) => Ok((
                CustomMessage { value },
                &s[..stream.byte_offset()],
                &s[stream.byte_offset()..],
            )),
            Some(Err(err)) => Err(CesrError::DeserializeError(err.to_string())),
            None => Err(CesrError::DeserializeError("End of stream".to_string())),
        }
    }

    pub fn from_cbor(s: &[u8]) -> CesrResult<CustomMessage> {
        let value: JsonValue = serde_cbor::from_slice(s)
            .map_err(|_| CesrError::SerializationError(format!("Unable to deserialize message from cbor")))?;
        Ok(CustomMessage { value })
    }

    pub fn from_cbor_stream(s: &[u8]) -> CesrResult<(CustomMessage, &[u8], &[u8])> {
        let mut stream = serde_cbor::Deserializer::from_slice(s).into_iter::<JsonValue>();
        match stream.next() {
            Some(Ok(value)) => Ok((
                CustomMessage { value },
                &s[..stream.byte_offset()],
                &s[stream.byte_offset()..],
            )),
            Some(Err(err)) => Err(CesrError::DeserializeError(err.to_string())),
            None => Err(CesrError::DeserializeError("End of stream".to_string())),
        }
    }

    pub fn from_mgpk(s: &[u8]) -> CesrResult<CustomMessage> {
        let value = serde_mgpk::from_slice(s)
            .map_err(|_| CesrError::SerializationError(format!("Unable to deserialize message from mgpk")))?;
        Ok(CustomMessage { value })
    }

    pub fn from_mgpk_stream(s: &[u8]) -> CesrResult<(CustomMessage, &[u8], &[u8])> {
        let mut deser = serde_mgpk::Deserializer::new(Cursor::new(s));
        match Deserialize::deserialize(&mut deser) {
            Ok(value) => Ok((
                CustomMessage { value },
                &s[..deser.get_ref().position() as usize],
                &s[deser.get_ref().position() as usize..],
            )),
            Err(err) => Err(CesrError::DeserializeError(err.to_string())),
        }
    }
}
