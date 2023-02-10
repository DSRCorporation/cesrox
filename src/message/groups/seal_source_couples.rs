use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouples {
    pub value: Vec<SealSourceCouple>,
}

impl Group<SealSourceCouple> for SealSourceCouples {
    const CODE: Codex = Codex::SealSourceCouples;

    fn new(value: Vec<SealSourceCouple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<SealSourceCouple> {
        &self.value
    }
}

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouple {
    pub seqner: Matter,
    pub saider: Matter,
}

impl SealSourceCouple {
    pub fn new(seqner: Matter, saider: Matter) -> Self {
        Self { seqner, saider }
    }
}

impl GroupItem for SealSourceCouple {
    fn qb64(&self) -> CesrResult<String> {
        let mut out = String::new();
        out.push_str(&self.seqner.qb64()?);
        out.push_str(&self.saider.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.seqner.qb64b()?);
        out.extend_from_slice(&self.saider.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.seqner.qb2()?);
        out.extend_from_slice(&self.saider.qb2()?);
        Ok(out)
    }
}