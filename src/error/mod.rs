use base64::DecodeError;
use core::num::ParseIntError;
use ed25519_dalek;
use nom::error::ErrorKind;
use rmp_serde as serde_mgpk;
use serde_cbor;
use serde_json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CesrError {
    #[error("Error during Serialization: {0}")]
    SerializationError(String),

    #[error("JSON Serialization error")]
    JSONSerializationError {
        #[from]
        source: serde_json::Error,
    },

    #[error("CBOR Serialization error")]
    CBORSerializationError {
        #[from]
        source: serde_cbor::Error,
    },

    #[error("MessagePack Serialization error")]
    MsgPackSerializationError {
        #[from]
        source: serde_mgpk::encode::Error,
    },

    #[error("Error parsing numerical value: {source}")]
    IntegerParseValue {
        #[from]
        source: ParseIntError,
    },

    #[error("Error while applying event: {0}")]
    SemanticError(String),

    #[error("Event signature verification faulty")]
    FaultySignatureVerification,

    #[error("Error while applying event: out of order event")]
    EventOutOfOrderError,

    #[error("Error while applying event: duplicate event")]
    EventDuplicateError,

    #[error("Not enough signatures while verifying")]
    NotEnoughSigsError,

    #[error("Signature verification failed")]
    SignatureVerificationError,

    #[error("Deserialize error: {0}")]
    DeserializeError(String),

    #[error("Identifier is not indexed into the DB")]
    NotIndexedError,

    #[error("Identifier ID is already present in the DB")]
    IdentifierPresentError,

    #[error("Base64 Decoding error")]
    Base64DecodingError {
        #[from]
        source: DecodeError,
    },

    #[error("Improper Prefix Type")]
    ImproperPrefixType,

    #[error("Storage error")]
    StorageError,

    #[error("Invalid identifier state")]
    InvalidIdentifierStat,

    #[cfg(feature = "async")]
    #[error("Zero send error")]
    ZeroSendError,

    #[error("Failed to obtain mutable ref to Ark of KeyManager")]
    MutArcKeyVaultError,

    #[error(transparent)]
    Ed25519DalekSignatureError(#[from] ed25519_dalek::SignatureError),

    // #[error(transparent)]
    // SledError(#[from] sled::Error),
    //
    // #[error(transparent)]
    // SerdeSerError(#[from] serializer_error::Error),
    #[cfg(feature = "wallet")]
    #[error(transparent)]
    WalletError(#[from] universal_wallet::Error),

    #[error("mutex is poisoned")]
    MutexPoisoned,

    #[error("Incorrect event digest")]
    IncorrectDigest,

    #[cfg(feature = "query")]
    #[error(transparent)]
    QueryError(#[from] crate::query::QueryError),

    #[error("Nom error")]
    StreamDeserializationError(nom::error::ErrorKind),

    #[error("Data type support is not implemented yet")]
    NotImplementedError,

    #[error("Empty bytes stream passed for parsing")]
    EmptyBytesStream,

    #[error("Requested variant does not exists")]
    NotExist,

    #[error("Invalid library state")]
    InvalidState,

    #[error("Incomplete")]
    Incomplete(usize),

    #[error("Too much data")]
    TooMuch,
}

impl<I> From<(I, nom::error::ErrorKind)> for CesrError {
    fn from((_, kind): (I, nom::error::ErrorKind)) -> CesrError {
        CesrError::StreamDeserializationError(kind)
    }
}

impl<E> From<nom::Err<E>> for CesrError {
    fn from(_: nom::Err<E>) -> CesrError {
        CesrError::StreamDeserializationError(ErrorKind::IsNot)
    }
}

impl From<std::str::Utf8Error> for CesrError {
    fn from(err: std::str::Utf8Error) -> CesrError {
        CesrError::DeserializeError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for CesrError {
    fn from(err: std::string::FromUtf8Error) -> CesrError {
        CesrError::DeserializeError(err.to_string())
    }
}

pub type CesrResult<T> = std::result::Result<T, CesrError>;

pub trait ToResult<T> {
    fn to_cesr(self) -> CesrResult<T>;
}

impl<T> ToResult<T> for serde_json::Result<T> {
    fn to_cesr(self) -> CesrResult<T> {
        self.map_err(|e| e.into())
    }
}
