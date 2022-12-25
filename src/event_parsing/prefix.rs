#![allow(non_upper_case_globals)]

use crate::error::CesrError;
use crate::{
    derivation::{
        attached_signature_code::b64_to_num, basic::Basic, self_addressing::SelfAddressing,
        self_signing::SelfSigning, DerivationCode,
    },
    error::CesrResult,
    keys::PublicKey,
    prefix::{
        AttachedSignaturePrefix, BasicPrefix, IdentifierPrefix, SelfAddressingPrefix,
        SelfSigningPrefix,
    },
};
use base64::URL_SAFE;
use nom::{bytes::complete::take, error::ErrorKind, IResult, ToUsize};
use std::str::from_utf8;

// TODO this could be a lot nicer, but is currently written to be careful and "easy" to follow
pub fn attached_signature(s: &[u8]) -> CesrResult<(&[u8], AttachedSignaturePrefix)> {
    let (more, type_c) = take_bytes(s, 1u8)?;

    const a: &[u8] = "A".as_bytes();
    const b: &[u8] = "B".as_bytes();
    const z: &[u8] = "0".as_bytes();

    match type_c {
        a => {
            let (maybe_sig, index_c) = take_bytes(more, 1u8)?;

            let index =
                b64_to_num(index_c).map_err(|_| nom::Err::Error((index_c, ErrorKind::IsNot)))?;

            let (rest, sig_s) = take_bytes(maybe_sig, 86u8)?;

            let sig = base64::decode_config(sig_s, base64::URL_SAFE)
                .map_err(|_| nom::Err::Error((index_c, ErrorKind::IsNot)))?;

            Ok((
                rest,
                AttachedSignaturePrefix::new(SelfSigning::Ed25519Sha512, sig, index),
            ))
        }
        b => {
            let (maybe_sig, index_c) = take_bytes(more, 1u8)?;

            let index =
                b64_to_num(index_c).map_err(|_| nom::Err::Error((index_c, ErrorKind::IsNot)))?;

            let (rest, sig_s) = take_bytes(maybe_sig, 86u8)?;

            let sig = base64::decode_config(sig_s, base64::URL_SAFE)
                .map_err(|_| nom::Err::Error((index_c, ErrorKind::IsNot)))?;

            Ok((
                rest,
                AttachedSignaturePrefix::new(SelfSigning::ECDSAsecp256k1Sha256, sig, index),
            ))
        }
        z => {
            let (maybe_count, type_c_2) = take_bytes(more, 1u8)?;
            match type_c_2 {
                a => {
                    let (maybe_sig, index_c) = take_bytes(maybe_count, 2u8)?;

                    let index = b64_to_num(index_c)
                        .map_err(|_| nom::Err::Error((index_c, ErrorKind::IsNot)))?;

                    let (rest, sig_s) = take_bytes(maybe_sig, 152u8)?;

                    let sig = base64::decode_config(sig_s, base64::URL_SAFE)
                        .map_err(|_| nom::Err::Error((index_c, ErrorKind::IsNot)))?;

                    Ok((
                        rest,
                        AttachedSignaturePrefix::new(SelfSigning::Ed448, sig, index),
                    ))
                }
                _ => Err(CesrError::InvalidState),
            }
        }
        _ => Err(CesrError::InvalidState),
    }
}

pub fn basic_prefix(s: &[u8]) -> CesrResult<(&[u8], BasicPrefix)> {
    const EXT: &[u8] = "1".as_bytes();

    let (_, type_c) = take_bytes(s, 1u8)?;

    let count_bytes = match type_c {
        EXT => 4u8,
        _ => 1u8,
    };
    let (rest, code_str) = take_bytes(s, count_bytes)?;

    let code: Basic = String::from_utf8(code_str.to_vec())?.parse()?;

    let (extra, b) = take_bytes(rest, code.derivative_b64_len())?;

    let pk = PublicKey::new(
        base64::decode_config(b.to_vec(), URL_SAFE)
            .map_err(|_| nom::Err::Error((s, ErrorKind::IsNot)))?,
    );
    Ok((extra, code.derive(pk)))
}

pub fn self_addressing_prefix(s: &[u8]) -> CesrResult<(&[u8], SelfAddressingPrefix)> {
    const EXT: &[u8] = "0".as_bytes();
    let (_, type_c) = take_bytes(s, 1u8)?;

    let count_bytes = match type_c {
        EXT => 2u8,
        _ => 1u8,
    };
    let (rest, code_str) = take_bytes(s, count_bytes)?;

    let code: SelfAddressing = String::from_utf8(code_str.to_vec())?.parse()?;

    let (extra, b) = take_bytes(rest, code.derivative_b64_len())?;

    let pref: SelfAddressingPrefix = from_utf8(&[code_str, b].concat())?.parse()?;
    Ok((extra, pref))
}

pub fn self_signing_prefix(s: &[u8]) -> CesrResult<(&[u8], SelfSigningPrefix)> {
    const EXT: &[u8] = "1".as_bytes();

    let (_, type_c) = take_bytes(s, 1u8)?;

    let count_bytes = match type_c {
        EXT => 4u8,
        _ => 2u8,
    };
    let (rest, code_str) = take_bytes(s, count_bytes)?;

    let code: SelfSigning = String::from_utf8(code_str.to_vec())?.parse()?;

    let (extra, b) = take_bytes(rest, code.derivative_b64_len())?;

    let sig =
        base64::decode_config(b, URL_SAFE).map_err(|_| nom::Err::Error((s, ErrorKind::IsNot)))?;
    Ok((extra, code.derive(sig)))
}

pub fn attached_sn(s: &[u8]) -> CesrResult<(&[u8], u64)> {
    let (more, type_c) = take_bytes(s, 2u8)?;

    const a: &[u8] = "0A".as_bytes();

    match type_c {
        a => {
            let (rest, parsed_sn) = take_bytes(more, 22u8)?;

            let sn = {
                let b64decode = base64::decode_config(parsed_sn, URL_SAFE)?;
                let mut sn_array: [u8; 8] = [0; 8];
                sn_array.copy_from_slice(&b64decode[8..]);
                u64::from_be_bytes(sn_array)
            };

            Ok((rest, sn))
        }
        _ => Err(CesrError::InvalidState),
    }
}

/// extracts Identifier prefix
pub fn prefix(s: &[u8]) -> CesrResult<(&[u8], IdentifierPrefix)> {
    let (rest, identifier) = match self_addressing_prefix(s) {
        Ok(sap) => Ok((sap.0, IdentifierPrefix::SelfAddressing(sap.1))),
        Err(_) => match basic_prefix(s) {
            Ok(bp) => Ok((bp.0, IdentifierPrefix::Basic(bp.1))),
            Err(e) => Err(e),
        },
    }?;
    Ok((rest, identifier))
}

pub fn take_bytes<C: ToUsize>(from: &[u8], count: C) -> CesrResult<(&[u8], &[u8])> {
    let nom_result: IResult<&[u8], &[u8]> = take(count)(from);
    nom_result.map_err(|err| err.into())
}

#[test]
fn test() {
    assert_eq!(
        attached_signature("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes()).unwrap(),
        ("".as_bytes(), AttachedSignaturePrefix::new(SelfSigning::Ed25519Sha512, vec![0u8; 64], 0))
    );

    assert_eq!(
        attached_signature("BCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes()).unwrap(),
        ("AA".as_bytes(), AttachedSignaturePrefix::new(SelfSigning::ECDSAsecp256k1Sha256, vec![0u8; 64], 2))
    );
}

#[test]
fn test_basic_prefix() {
    use crate::prefix::Prefix;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    let kp = Keypair::generate(&mut OsRng);

    let bp = BasicPrefix {
        derivation: Basic::Ed25519,
        public_key: PublicKey::new(kp.public.to_bytes().to_vec()),
    };
    let bp_str = [&bp.to_str(), "more"].join("");
    let parsed = basic_prefix(bp_str.as_bytes()).unwrap();
    assert_eq!(parsed, ("more".as_bytes(), bp))
}

#[test]
fn test_self_adressing() {
    use crate::prefix::Prefix;

    let sap: SelfAddressingPrefix = "EJJR2nmwyYAfSVPzhzS6b5CMZAoTNZH3ULvaU6Z-i0d8"
        .parse()
        .unwrap();
    let str_to_parse = [&sap.to_str(), "more"].join("");
    assert_eq!(
        self_addressing_prefix(str_to_parse.as_bytes()).unwrap(),
        ("more".as_bytes(), sap)
    );
}

#[test]
fn test_self_signing() {
    use crate::prefix::Prefix;

    let sig_prefix: SelfSigningPrefix =
        "0Bq1UBr1QD5TokdcnO_FmnoYsd8rB4_-oaQtk0dfFSSXPcxAu7pSaQIVfkhzckCVmTIgrdxyXS21uZgs7NxoyZAQ"
            .parse()
            .unwrap();
    let string_to_parse = [&sig_prefix.to_str(), "more"].join("");

    assert_eq!(
        self_signing_prefix(string_to_parse.as_bytes()).unwrap(),
        ("more".as_bytes(), sig_prefix.clone())
    );
}

#[test]
fn test_sn_parse() {
    let sn = attached_sn("0AAAAAAAAAAAAAAAAAAAAAAw".as_bytes()).unwrap();
    assert_eq!(sn, ("".as_bytes(), 3));
}
