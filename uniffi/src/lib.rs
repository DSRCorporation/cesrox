use core::str::FromStr;

pub use cesrox_core::error::CesrError;
pub use cesrox_core::error::CesrResult;
pub use cesrox_core::primitives::prefix::{
    Prefix,
    BasicPrefix,
    SelfAddressingPrefix,
    SelfSigningPrefix,
    IdentifierPrefix,
    AttachedSignaturePrefix,
};
pub use cesrox_core::primitives::key::Key;
pub use cesrox_core::primitives::derivation::{
    BasicCode,
    SelfSigningCode,
    SelfAddressingCode,
};
pub use cesrox_core::groups::{
    CesrGroup,
    IndexedControllerSignatures,
    IndexedWitnessSignatures,
    NontransferableIdentifierReceiptCouples,
    NontransferableIdentifierReceiptCouple,
    TransferableIndexedSignaturesGroups,
    TransferableIndexedSignaturesGroup,
    LastEstSignaturesGroups,
    LastEstSignaturesGroup,
    SealSourceCouplets,
    SourceSeal,
    EventSeal,
    Frame
};
pub use cesrox_core::message::{
    CustomMessage,
    Message,
    MessageList,
};

/*
    Message
*/
pub struct MessageFromStreamResult {
    pub rest: Vec<u8>,
    pub message: Message
}

pub fn message_from_bytes(bytes: &[u8]) -> CesrResult<Message> {
    Message::from_bytes(bytes)
}

pub fn message_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageFromStreamResult> {
    let (rest, message) = Message::from_stream_bytes(bytes)?;
    Ok(MessageFromStreamResult {
        rest: rest.to_vec(),
        message
    })
}

pub fn message_to_bytes(message: &Message) -> CesrResult<Vec<u8>> {
    message.to_bytes()
}

pub fn message_to_str(message: &Message) -> CesrResult<String> {
    message.to_str()
}

pub fn message_from_str(value: &str) -> CesrResult<Message> {
    Message::from_str(value)
}

/*
    MessageList
*/
pub struct MessageListFromStreamResult {
    pub rest: Vec<u8>,
    pub messages: MessageList
}

pub fn message_list_from_bytes(bytes: &[u8]) -> CesrResult<MessageList> {
    MessageList::from_bytes(bytes)
}

pub fn message_list_from_stream_bytes(bytes: &[u8]) -> CesrResult<MessageListFromStreamResult> {
    let (rest, messages) = MessageList::from_stream_bytes(bytes)?;
    Ok(MessageListFromStreamResult {
        rest: rest.to_vec(),
        messages
    })
}

pub fn message_list_to_bytes(message_list: &MessageList) -> CesrResult<Vec<u8>> {
    message_list.to_bytes()
}

pub fn message_list_to_str(message_list: &MessageList) -> CesrResult<String> {
    message_list.to_str()
}

pub fn message_list_from_str(value: &str) -> CesrResult<MessageList> {
    MessageList::from_str(value)
}

/*
    Key
*/
pub fn key_create(value: &[u8]) -> Key {
    Key::new(value)
}

/*
    BasicPrefix
*/
pub struct BasicPrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: BasicPrefix
}

pub fn basic_prefix_create(code: BasicCode, public_key: Key) -> BasicPrefix {
    BasicPrefix::new(code, public_key)
}

pub fn basic_prefix_to_str(basic_prefix: &BasicPrefix) -> String {
    basic_prefix.to_str()
}

pub fn basic_prefix_from_str(str: &str) -> CesrResult<BasicPrefix> {
    BasicPrefix::from_str(str)
}

pub fn basic_prefix_to_bytes(basic_prefix: &BasicPrefix) -> Vec<u8> {
    basic_prefix.to_bytes()
}

pub fn basic_prefix_from_bytes(bytes: &[u8]) -> CesrResult<BasicPrefix> {
    BasicPrefix::from_bytes(bytes)
}

pub fn basic_prefix_from_stream_bytes(bytes: &[u8]) -> CesrResult<BasicPrefixFromStreamResult> {
    let (res, message) = BasicPrefix::from_stream_bytes(bytes)?;
    Ok(BasicPrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    SelfSigningPrefix
*/
pub struct SelfSigningPrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: SelfSigningPrefix
}

pub fn self_signing_prefix_create(code: SelfSigningCode, signature: Vec<u8>) -> SelfSigningPrefix {
    SelfSigningPrefix::new(code, signature)
}

pub fn self_signing_prefix_to_str(self_signing_prefix: &SelfSigningPrefix) -> String {
    self_signing_prefix.to_str()
}

pub fn self_signing_prefix_from_str(str: &str) -> CesrResult<SelfSigningPrefix> {
    SelfSigningPrefix::from_str(str)
}

pub fn self_signing_prefix_to_bytes(self_signing_prefix: &SelfSigningPrefix) -> Vec<u8> {
    self_signing_prefix.to_bytes()
}

pub fn self_signing_prefix_from_bytes(bytes: &[u8]) -> CesrResult<SelfSigningPrefix> {
    SelfSigningPrefix::from_bytes(bytes)
}

pub fn self_signing_prefix_from_stream_bytes(str: &[u8]) -> CesrResult<SelfSigningPrefixFromStreamResult> {
    let (res, message) = SelfSigningPrefix::from_stream_bytes(str)?;
    Ok(SelfSigningPrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    SelfAddressingPrefix
*/
pub struct SelfAddressingPrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: SelfAddressingPrefix
}

pub fn self_addressing_prefix_create(code: SelfAddressingCode, digest: Vec<u8>) -> SelfAddressingPrefix {
    SelfAddressingPrefix::new(code, digest)
}

pub fn self_addressing_prefix_to_str(self_addressing_prefix: &SelfAddressingPrefix) -> String {
    self_addressing_prefix.to_str()
}

pub fn self_addressing_prefix_from_str(str: &str) -> CesrResult<SelfAddressingPrefix> {
    SelfAddressingPrefix::from_str(str)
}

pub fn self_addressing_prefix_to_bytes(attached_signature_prefix: &SelfAddressingPrefix) -> Vec<u8> {
    attached_signature_prefix.to_bytes()
}

pub fn self_addressing_prefix_from_bytes(bytes: &[u8]) -> CesrResult<SelfAddressingPrefix> {
    SelfAddressingPrefix::from_bytes(bytes)
}

pub fn self_addressing_prefix_from_stream_bytes(str: &[u8]) -> CesrResult<SelfAddressingPrefixFromStreamResult> {
    let (res, message) = SelfAddressingPrefix::from_stream_bytes(str)?;
    Ok(SelfAddressingPrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    AttachedSignaturePrefix
*/
pub struct AttachedSignaturePrefixFromStreamResult {
    pub rest: Vec<u8>,
    pub message: AttachedSignaturePrefix
}

pub fn attached_signature_prefix_create(signature: SelfSigningPrefix, index: u16) -> AttachedSignaturePrefix {
    AttachedSignaturePrefix::new(signature, index)
}

pub fn attached_signature_prefix_to_str(attached_signature_prefix: &AttachedSignaturePrefix) -> String {
    attached_signature_prefix.to_str()
}

pub fn attached_signature_prefix_from_str(str: &str) -> CesrResult<AttachedSignaturePrefix> {
    AttachedSignaturePrefix::from_str(str)
}

pub fn attached_signature_prefix_to_bytes(attached_signature_prefix: &AttachedSignaturePrefix) -> Vec<u8> {
    attached_signature_prefix.to_bytes()
}

pub fn attached_signature_prefix_from_bytes(bytes: &[u8]) -> CesrResult<AttachedSignaturePrefix> {
    AttachedSignaturePrefix::from_bytes(bytes)
}

pub fn attached_signature_prefix_from_stream_bytes(str: &[u8]) -> CesrResult<AttachedSignaturePrefixFromStreamResult> {
    let (res, message) = AttachedSignaturePrefix::from_stream_bytes(str)?;
    Ok(AttachedSignaturePrefixFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    IndexedControllerSignatures
*/
pub struct IndexedControllerSignaturesFromStreamResult {
    pub rest: Vec<u8>,
    pub message: IndexedControllerSignatures
}

pub fn indexed_controller_signatures_create(value: Vec<AttachedSignaturePrefix>) -> IndexedControllerSignatures {
    IndexedControllerSignatures::new(value)
}

pub fn indexed_controller_signatures_to_str(indexed_controller_signatures: &IndexedControllerSignatures) -> String {
    indexed_controller_signatures.to_str()
}

pub fn indexed_controller_signatures_from_str(str: &str) -> CesrResult<IndexedControllerSignatures> {
    IndexedControllerSignatures::from_str(str)
}

pub fn indexed_controller_signatures_to_bytes(indexed_controller_signatures: &IndexedControllerSignatures) -> Vec<u8> {
    indexed_controller_signatures.to_bytes()
}

pub fn indexed_controller_signatures_from_bytes(bytes: &[u8]) -> CesrResult<IndexedControllerSignatures> {
    IndexedControllerSignatures::from_bytes(bytes)
}

pub fn indexed_controller_signatures_from_stream_bytes(str: &[u8]) -> CesrResult<IndexedControllerSignaturesFromStreamResult> {
    let (res, message) = IndexedControllerSignatures::from_stream_bytes(str)?;
    Ok(IndexedControllerSignaturesFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    IndexedWitnessSignatures
*/
pub struct IndexedWitnessSignaturesFromStreamResult {
    pub rest: Vec<u8>,
    pub message: IndexedWitnessSignatures
}

pub fn witness_controller_signatures_create(value: Vec<AttachedSignaturePrefix>) -> IndexedWitnessSignatures {
    IndexedWitnessSignatures::new(value)
}

pub fn witness_controller_signatures_to_str(witness_controller_signatures: &IndexedWitnessSignatures) -> String {
    witness_controller_signatures.to_str()
}

pub fn witness_controller_signatures_from_str(str: &str) -> CesrResult<IndexedWitnessSignatures> {
    IndexedWitnessSignatures::from_str(str)
}

pub fn witness_controller_signatures_to_bytes(witness_controller_signatures: &IndexedWitnessSignatures) -> Vec<u8> {
    witness_controller_signatures.to_bytes()
}

pub fn witness_controller_signatures_from_bytes(bytes: &[u8]) -> CesrResult<IndexedWitnessSignatures> {
    IndexedWitnessSignatures::from_bytes(bytes)
}

pub fn witness_controller_signatures_from_stream_bytes(str: &[u8]) -> CesrResult<IndexedWitnessSignaturesFromStreamResult> {
    let (res, message) = IndexedWitnessSignatures::from_stream_bytes(str)?;
    Ok(IndexedWitnessSignaturesFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    NontransferableIdentifierReceiptCouple
*/
pub fn non_transferable_identifier_receipt_couple_create(basic: BasicPrefix, self_signing: SelfSigningPrefix) -> NontransferableIdentifierReceiptCouple {
    NontransferableIdentifierReceiptCouple::new(basic, self_signing)
}

/*
    NontransferableIdentifierReceiptCouples
*/
pub struct NontransferableIdentifierReceiptCouplesFromStreamResult {
    pub rest: Vec<u8>,
    pub message: NontransferableIdentifierReceiptCouples
}

pub fn non_transferable_identifier_receipt_couples_create(value: Vec<NontransferableIdentifierReceiptCouple>) -> NontransferableIdentifierReceiptCouples {
    NontransferableIdentifierReceiptCouples::new(value)
}

pub fn non_transferable_identifier_receipt_couples_to_str(non_transferable_identifier_receipt_couples: &NontransferableIdentifierReceiptCouples) -> String {
    non_transferable_identifier_receipt_couples.to_str()
}

pub fn non_transferable_identifier_receipt_couples_from_str(str: &str) -> CesrResult<NontransferableIdentifierReceiptCouples> {
    NontransferableIdentifierReceiptCouples::from_str(str)
}

pub fn non_transferable_identifier_receipt_couples_to_bytes(non_transferable_identifier_receipt_couples: &NontransferableIdentifierReceiptCouples) -> Vec<u8> {
    non_transferable_identifier_receipt_couples.to_bytes()
}

pub fn non_transferable_identifier_receipt_couples_from_bytes(bytes: &[u8]) -> CesrResult<NontransferableIdentifierReceiptCouples> {
    NontransferableIdentifierReceiptCouples::from_bytes(bytes)
}

pub fn non_transferable_identifier_receipt_couples_from_stream_bytes(str: &[u8]) -> CesrResult<NontransferableIdentifierReceiptCouplesFromStreamResult> {
    let (res, message) = NontransferableIdentifierReceiptCouples::from_stream_bytes(str)?;
    Ok(NontransferableIdentifierReceiptCouplesFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    SourceSeal
*/
pub fn source_seal_create(sn: u64, digest: SelfAddressingPrefix) -> SourceSeal {
    SourceSeal::new(sn, digest)
}

/*
    SealSourceCouplets
*/
pub struct SealSourceCoupletsFromStreamResult {
    pub rest: Vec<u8>,
    pub message: SealSourceCouplets
}

pub fn seal_source_couplets_create(value: Vec<SourceSeal>) -> SealSourceCouplets {
    SealSourceCouplets::new(value)
}

pub fn seal_source_couplets_to_str(seal_source_couplets: &SealSourceCouplets) -> String {
    seal_source_couplets.to_str()
}

pub fn seal_source_couplets_from_str(str: &str) -> CesrResult<SealSourceCouplets> {
    SealSourceCouplets::from_str(str)
}

pub fn seal_source_couplets_to_bytes(transferable_indexed_signatures_groups: &SealSourceCouplets) -> Vec<u8> {
    transferable_indexed_signatures_groups.to_bytes()
}

pub fn seal_source_couplets_from_bytes(bytes: &[u8]) -> CesrResult<SealSourceCouplets> {
    SealSourceCouplets::from_bytes(bytes)
}

pub fn seal_source_couplets_from_stream_bytes(str: &[u8]) -> CesrResult<SealSourceCoupletsFromStreamResult> {
    let (res, message) = SealSourceCouplets::from_stream_bytes(str)?;
    Ok(SealSourceCoupletsFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    EventSeal
*/
pub fn event_seal_create(prefix: IdentifierPrefix, sn: u64, event_digest: SelfAddressingPrefix) -> EventSeal {
    EventSeal::new(prefix, sn, event_digest)
}

/*
    TransferableIndexedSignatureGroup
*/
pub fn transferable_indexed_signatures_group_create(event_seal: EventSeal, signature_prefixes: Vec<AttachedSignaturePrefix>) -> TransferableIndexedSignaturesGroup {
    TransferableIndexedSignaturesGroup::new(event_seal, signature_prefixes)
}

/*
    TransferableIndexedSignaturesGroups
*/
pub struct TransferableIndexedSignaturesGroupsFromStreamResult {
    pub rest: Vec<u8>,
    pub message: TransferableIndexedSignaturesGroups
}

pub fn transferable_indexed_signatures_groups_create(value: Vec<TransferableIndexedSignaturesGroup>) -> TransferableIndexedSignaturesGroups {
    TransferableIndexedSignaturesGroups::new(value)
}

pub fn transferable_indexed_signatures_groups_to_str(transferable_indexed_signatures_groups: &TransferableIndexedSignaturesGroups) -> String {
    transferable_indexed_signatures_groups.to_str()
}

pub fn transferable_indexed_signatures_groups_from_str(str: &str) -> CesrResult<TransferableIndexedSignaturesGroups> {
    TransferableIndexedSignaturesGroups::from_str(str)
}

pub fn transferable_indexed_signatures_groups_to_bytes(transferable_indexed_signatures_groups: &TransferableIndexedSignaturesGroups) -> Vec<u8> {
    transferable_indexed_signatures_groups.to_bytes()
}

pub fn transferable_indexed_signatures_groups_from_bytes(bytes: &[u8]) -> CesrResult<TransferableIndexedSignaturesGroups> {
    TransferableIndexedSignaturesGroups::from_bytes(bytes)
}

pub fn transferable_indexed_signatures_groups_from_stream_bytes(str: &[u8]) -> CesrResult<TransferableIndexedSignaturesGroupsFromStreamResult> {
    let (res, message) = TransferableIndexedSignaturesGroups::from_stream_bytes(str)?;
    Ok(TransferableIndexedSignaturesGroupsFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    LastEstSignaturesGroup
*/
pub fn last_est_signatures_group_create(identifier_prefix: IdentifierPrefix, attached_signature_prefixes: Vec<AttachedSignaturePrefix>) -> LastEstSignaturesGroup {
    LastEstSignaturesGroup::new(identifier_prefix, attached_signature_prefixes)
}

/*
    LastEstSignaturesGroups
*/
pub struct LastEstSignaturesGroupsFromStreamResult {
    pub rest: Vec<u8>,
    pub message: LastEstSignaturesGroups
}

pub fn last_est_signatures_groups_create(value: Vec<LastEstSignaturesGroup>) -> LastEstSignaturesGroups {
    LastEstSignaturesGroups::new(value)
}

pub fn last_est_signatures_groups_to_str(last_est_signatures_group: &LastEstSignaturesGroups) -> String {
    last_est_signatures_group.to_str()
}

pub fn last_est_signatures_groups_from_str(str: &str) -> CesrResult<LastEstSignaturesGroups> {
    LastEstSignaturesGroups::from_str(str)
}

pub fn last_est_signatures_groups_to_bytes(last_est_signatures_group: &LastEstSignaturesGroups) -> Vec<u8> {
    last_est_signatures_group.to_bytes()
}

pub fn last_est_signatures_groups_from_bytes(bytes: &[u8]) -> CesrResult<LastEstSignaturesGroups> {
    LastEstSignaturesGroups::from_bytes(bytes)
}

pub fn last_est_signatures_groups_from_stream_bytes(str: &[u8]) -> CesrResult<LastEstSignaturesGroupsFromStreamResult> {
    let (res, message) = LastEstSignaturesGroups::from_stream_bytes(str)?;
    Ok(LastEstSignaturesGroupsFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

/*
    Frame
*/
pub struct FrameFromStreamResult {
    pub rest: Vec<u8>,
    pub message: Frame
}

pub fn frame_create(value: Vec<CesrGroup>) -> Frame {
    Frame::new(value)
}

pub fn frame_to_str(frame: &Frame) -> String {
    frame.to_str()
}

pub fn frame_from_str(str: &str) -> CesrResult<Frame> {
    Frame::from_str(str)
}

pub fn frame_to_bytes(frame: &Frame) -> Vec<u8> {
    frame.to_bytes()
}

pub fn frame_from_bytes(bytes: &[u8]) -> CesrResult<Frame> {
    Frame::from_bytes(bytes)
}

pub fn frame_from_stream_bytes(str: &[u8]) -> CesrResult<FrameFromStreamResult> {
    let (res, message) = Frame::from_stream_bytes(str)?;
    Ok(FrameFromStreamResult {
        rest: res.to_vec(),
        message
    })
}

// We use `JsonValue` in our UDL. It moves to and from Uniffi bindings via a string.
pub type JsonValue = serde_json::Value;

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
