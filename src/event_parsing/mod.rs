use serde::Deserialize;

use crate::event::sections::seal::{EventSeal, SourceSeal};
use crate::prefix::{AttachedSignaturePrefix, BasicPrefix, IdentifierPrefix, SelfSigningPrefix};

pub mod attachment;
pub mod payload_size;
pub mod prefix;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Attachment {
    // Count codes
    SealSourceCouplets(Vec<SourceSeal>),
    AttachedSignatures(Vec<AttachedSignaturePrefix>),
    ReceiptCouplets(Vec<(BasicPrefix, SelfSigningPrefix)>),
    // Group codes
    SealSignaturesGroups(Vec<(EventSeal, Vec<AttachedSignaturePrefix>)>),
    // List of signatures made using keys from last establishment event od identifier of prefix
    LastEstSignaturesGroups(Vec<(IdentifierPrefix, Vec<AttachedSignaturePrefix>)>),
    // Frame codes
    Frame(Vec<Attachment>),
}
