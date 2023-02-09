use cesride::counter::Codex;
use cesride::{Counter, Matter};

use crate::error::CesrResult;
use crate::message::groups::ControllerIdxSigs;

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroups {
    pub value: Vec<TransLastIdxSigGroup>,
}

impl TransLastIdxSigGroups {
    pub const CODE: Codex = Codex::TransLastIdxSigGroups;

    pub fn new(value: Vec<TransLastIdxSigGroup>) -> Self {
        Self { value }
    }

    pub fn counter(&self) -> Counter {
        Counter::new(&Self::CODE.code(), self.count())
    }

    pub fn count(&self) -> u32 {
        self.value.len() as u32
    }

    pub fn qb64(&self) -> CesrResult<String> {
        let mut out = self.counter().qb64()?;
        for couple in self.value.iter() {
            out.push_str(&couple.prefixer.qb64()?);
            out.push_str(&couple.isigers.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.prefixer.qb64b()?);
            out.extend_from_slice(&couple.isigers.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.prefixer.qb2()?);
            out.extend_from_slice(&couple.isigers.qb2()?);
        }
        Ok(out)
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
