use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::ControllerIdxSigs;
use crate::message::groups::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroups {
    pub value: Vec<TransIdxSigGroup>,
}

impl Group<TransIdxSigGroup> for TransIdxSigGroups {
    const CODE: Codex = Codex::TransIdxSigGroups;

    fn new(value: Vec<TransIdxSigGroup>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<TransIdxSigGroup> {
        &self.value
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroup {
    pub prefixer: Matter,
    pub seqner: Matter,
    pub saider: Matter,
    pub isigers: ControllerIdxSigs,
}

impl TransIdxSigGroup {
    pub fn new(
        prefixer: Matter,
        seqner: Matter,
        saider: Matter,
        isigers: ControllerIdxSigs,
    ) -> Self {
        Self {
            prefixer,
            seqner,
            saider,
            isigers,
        }
    }
}

impl GroupItem for TransIdxSigGroup {
    fn qb64(&self) -> CesrResult<String> {
        let mut out = String::new();
        out.push_str(&self.prefixer.qb64()?);
        out.push_str(&self.seqner.qb64()?);
        out.push_str(&self.saider.qb64()?);
        out.push_str(&self.isigers.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb64b()?);
        out.extend_from_slice(&self.seqner.qb64b()?);
        out.extend_from_slice(&self.saider.qb64b()?);
        out.extend_from_slice(&self.isigers.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb2()?);
        out.extend_from_slice(&self.seqner.qb2()?);
        out.extend_from_slice(&self.saider.qb2()?);
        out.extend_from_slice(&self.isigers.qb2()?);
        Ok(out)
    }
}
