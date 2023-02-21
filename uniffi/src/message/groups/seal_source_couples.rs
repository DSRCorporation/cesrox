use cesrox::error::ParsideResult;
use cesrox::Group;
use cesrox::{Saider, Seqner};
pub use cesrox::message::groups::{
    SealSourceCouples,
    SealSourceCouple,
};

pub fn seal_source_couple_create(seqner: Seqner, saider: Saider) -> SealSourceCouple {
    SealSourceCouple::new(seqner, saider)
}

pub fn seal_source_couples_create(value: Vec<SealSourceCouple>) -> SealSourceCouples {
    SealSourceCouples::new(value)
}

pub fn seal_source_couples_qb64(seal_source_couples: &SealSourceCouples) -> ParsideResult<String> {
    seal_source_couples.qb64()
}

pub fn seal_source_couples_qb64b(seal_source_couples: &SealSourceCouples) -> ParsideResult<Vec<u8>> {
    seal_source_couples.qb64b()
}

pub fn seal_source_couples_qb2(seal_source_couples: &SealSourceCouples) -> ParsideResult<Vec<u8>> {
    seal_source_couples.qb2()
}
