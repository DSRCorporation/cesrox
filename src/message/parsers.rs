use rmp_serde as serde_mgpk;
use serde::Deserialize;
use std::io::Cursor;

use crate::event_parsing::attachment::attachment;
use crate::{
    error::{CesrError, CesrResult},
    message::message::CesrMessage,
};

pub fn cesr_message(s: &[u8]) -> CesrResult<(&[u8], CesrMessage)> {
    attachment(s)
}

pub fn json_message<'a, D: Deserialize<'a>>(s: &'a [u8]) -> CesrResult<(D, &'a [u8], &'a [u8])> {
    let mut stream = serde_json::Deserializer::from_slice(s).into_iter::<D>();
    match stream.next() {
        Some(Ok(value)) => Ok((
            value,
            &s[..stream.byte_offset()],
            &s[stream.byte_offset()..],
        )),
        Some(Err(err)) => Err(CesrError::DeserializeError(err.to_string())),
        None => Err(CesrError::DeserializeError("End of stream".to_string())),
    }
}

pub fn cbor_message<'a, D: Deserialize<'a>>(s: &'a [u8]) -> CesrResult<(D, &'a [u8], &'a [u8])> {
    let mut stream = serde_cbor::Deserializer::from_slice(s).into_iter::<D>();
    match stream.next() {
        Some(Ok(value)) => Ok((
            value,
            &s[..stream.byte_offset()],
            &s[stream.byte_offset()..],
        )),
        Some(Err(err)) => Err(CesrError::DeserializeError(err.to_string())),
        None => Err(CesrError::DeserializeError("End of stream".to_string())),
    }
}

pub fn mgpk_message<'a, D: Deserialize<'a>>(s: &'a [u8]) -> CesrResult<(D, &'a [u8], &'a [u8])> {
    let mut deser = serde_mgpk::Deserializer::new(Cursor::new(s));
    match Deserialize::deserialize(&mut deser) {
        Ok(event) => Ok((
            event,
            &s[..deser.get_ref().position() as usize],
            &s[deser.get_ref().position() as usize..],
        )),
        Err(err) => Err(CesrError::DeserializeError(err.to_string())),
    }
}
