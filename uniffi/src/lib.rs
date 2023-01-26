pub use cesrox_core::error::CesrError;
pub use cesrox_core::error::CesrResult;
pub use cesrox_core::message::message::{CesrMessage, CustomMessage, MessageList};
pub use cesrox_core::prefix::BasicPrefix;
pub use cesrox_core::prefix::Prefix;
pub use cesrox_core::prefix::SelfAddressingPrefix;
pub use cesrox_core::prefix::SelfSigningPrefix;

use cesrox_core::error::ToResult;
use std::str::FromStr;

pub struct FFIMessageListReturn {
    pub rest: Vec<u8>,
    pub list: Vec<Message>,
}

pub enum Message {
    CustomMessage {
        value: CustomMessage
    },
    CesrMessage{
        value: CesrMessage
    },
}

// pub struct CustomMessage {
//     value: serde_json::Value;
// }

// // We use `JsonValue` in our UDL. It moves to and from Uniffi bindings via a string.
// pub type JsonValue = serde_json::Value;

// // We must implement the UniffiCustomTypeWrapper trait.
// impl UniffiCustomTypeWrapper for JsonValue {
//     type Wrapped = String;

//     fn wrap(val: Self::Wrapped) -> uniffi::Result<Self> {
//         Ok(serde_json::from_str(&val).to_didcomm("Invalid json value")?)
//     }

//     fn unwrap(obj: Self) -> Self::Wrapped {
//         serde_json::to_string(&obj).expect("unable serialize json value")
//     }
// }

// We must implement the UniffiCustomTypeConverter trait.
impl UniffiCustomTypeConverter for CustomMessage {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val).unwrap())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serde_json::to_string(&obj.value()).expect("unable serialize json value")
    }
}

pub fn parse_message_list(bytes: &Vec<u8>) -> CesrResult<FFIMessageListReturn> {
    let (rest, messages) = MessageList::from_stream(bytes).unwrap();
    return Ok(FFIMessageListReturn {
        rest: rest.to_vec(),
        list: messages,
    });
}

pub struct TypeOneType {
    value: u8,
}

pub struct TypeTwoType {
    pub one: u8,
    pub two: String,
}

pub enum TestEnum {
    TypeOne {
        value: TypeOneType
    },
    TypeTwo {
        value: TypeTwoType,
    },
}

uniffi_macros::include_scaffolding!("cesrox");
