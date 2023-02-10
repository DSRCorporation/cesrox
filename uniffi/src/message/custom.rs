// We use `JsonValue` in our UDL. It moves to and from Uniffi bindings via a string.
pub type JsonValue = serde_json::Value;

pub use cesrox_core::CustomPayload;