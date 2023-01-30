pub use cesrox_core::error::CesrError;
pub use cesrox_core::error::CesrResult;
pub use cesrox_core::primitives::prefix::{
    BasicPrefix,
    SelfAddressingPrefix,
    SelfSigningPrefix,
    IdentifierPrefix,
    AttachedSignaturePrefix
};
pub use cesrox_core::primitives::key::Key;
pub use cesrox_core::primitives::derivation::{
    BasicCode,
    SelfSigningCode,
    SelfAddressingCode
};
pub use cesrox_core::groups::{
    IndexedControllerSignatures,
    IndexedWitnessSignatures,
    NontransferableIdentifierReceiptCouples,
    NontransferableIdentifierReceiptCouple,
    TransferableIndexedSignatureGroups,
    TransferableIndexedSignatureGroup,
    LastEstSignaturesGroups,
    LastEstSignaturesGroup,
    SealSourceCouplets,
    SourceSeal,
    EventSeal
};

// #[derive(Debug, Clone, Default, PartialEq)]
// pub struct SealSourceCouplets {
//     pub value: Vec<Arc<SourceSeal>>,
// }
//
// impl SealSourceCouplets {
//     pub fn new(value: Vec<Arc<SourceSeal>>) -> SealSourceCouplets {
//         Self {
//             value,
//         }
//     }
//
//     pub fn to_str(&self) -> String {
//         let serialized_sources = self.value.iter().fold("".into(), |acc, s| {
//             [acc, pack_sn(s.sn), s.digest.to_str()].join("")
//         });
//         pack_counter(PayloadType::MG, self.value.len(), serialized_sources)
//     }
// }

//
// pub struct FFIMessageListReturn {
//     pub rest: Vec<u8>,
//     pub list: Vec<Message>,
// }
//
// pub enum Message {
//     CustomMessage {
//         value: CustomMessage
//     },
//     CesrMessage{
//         value: CesrMessage
//     },
// }

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
// impl UniffiCustomTypeConverter for CustomMessage {
//     type Builtin = String;
//
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(serde_json::from_str(&val).unwrap())
//     }
//
//     fn from_custom(obj: Self) -> Self::Builtin {
//         serde_json::to_string(&obj.value()).expect("unable serialize json value")
//     }
// }
//
// pub fn parse_message_list(bytes: &Vec<u8>) -> CesrResult<FFIMessageListReturn> {
//     let (rest, messages) = MessageList::from_stream(bytes).unwrap();
//     return Ok(FFIMessageListReturn {
//         rest: rest.to_vec(),
//         list: messages,
//     });
// }
//
// pub struct TypeOneType {
//     value: u8,
// }
//
// pub struct TypeTwoType {
//     pub one: u8,
//     pub two: String,
// }
//
// pub enum TestEnum {
//     TypeOne {
//         value: TypeOneType
//     },
//     TypeTwo {
//         value: TypeTwoType,
//     },
// }

uniffi_macros::include_scaffolding!("cesrox");
