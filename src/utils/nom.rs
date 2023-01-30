use nom::{bytes::complete::take, IResult, ToUsize};
use crate::error::CesrResult;

// Macro to map function returning Cesr result to nom compatible
#[macro_export]
macro_rules! nomify {
    ($func:expr) => {
        |bytes: &'a [u8]| {
            $func(bytes).map_err(|_| nom::Err::Error((bytes, nom::error::ErrorKind::IsNot)))
        }
    };
}

pub fn take_bytes<C: ToUsize>(from: &[u8], count: C) -> CesrResult<(&[u8], &[u8])> {
    let nom_result: IResult<&[u8], &[u8]> = take(count)(from);
    nom_result.map_err(|err| err.into())
}
