pub mod attached_signature;
pub mod basic;
pub mod self_addressing;
pub mod self_signing;
pub mod serial_number;
pub mod timestamp;

pub use basic::BasicCode;
pub use self_addressing::SelfAddressingCode;
pub use self_signing::SelfSigningCode;
pub use attached_signature::AttachedSignatureCode;
pub use serial_number::SerialNumberCode;
pub use timestamp::TimestampCode;

pub trait DerivationCode {
    fn code_len(&self) -> usize;
    fn derivative_b64_len(&self) -> usize;
    fn prefix_b64_len(&self) -> usize {
        self.code_len() + self.derivative_b64_len()
    }
    fn to_str(&self) -> String;
}
