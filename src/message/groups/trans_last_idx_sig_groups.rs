use cesride::counter::Codex;
use cesride::Matter;

use crate::error::CesrResult;
use crate::message::groups::{ControllerIdxSigs, Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroups {
    pub value: Vec<TransLastIdxSigGroup>,
}

impl Group<TransLastIdxSigGroup> for TransLastIdxSigGroups {
    const CODE: Codex = Codex::TransLastIdxSigGroups;

    fn new(value: Vec<TransLastIdxSigGroup>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<TransLastIdxSigGroup> {
        &self.value
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroup {
    pub prefixer: Matter,
    pub isigers: ControllerIdxSigs,
}

impl TransLastIdxSigGroup {
    pub fn new(prefixer: Matter, isigers: ControllerIdxSigs) -> Self {
        Self { prefixer, isigers }
    }
}

impl GroupItem for TransLastIdxSigGroup {
    fn qb64(&self) -> CesrResult<String> {
        let mut out = String::new();
        out.push_str(&self.prefixer.qb64()?);
        out.push_str(&self.isigers.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb64b()?);
        out.extend_from_slice(&self.isigers.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb2()?);
        out.extend_from_slice(&self.isigers.qb2()?);
        Ok(out)
    }
}
