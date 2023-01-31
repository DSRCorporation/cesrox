pub use cesrox_core::error::{CesrError, CesrResult};

pub mod groups;
pub mod message;
pub mod primitives;

pub use self::groups::*;
pub use self::message::*;
pub use self::primitives::*;

// We must implement the UniffiCustomTypeWrapper trait.
impl UniffiCustomTypeConverter for JsonValue {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val).expect("unable wrap json value"))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serde_json::to_string(&obj).expect("unable unwrap json value")
    }
}

uniffi_macros::include_scaffolding!("cesrox");
