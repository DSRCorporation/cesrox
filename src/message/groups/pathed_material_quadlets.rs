use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::{Counter, Matter};

#[derive(Debug, Clone, Default)]
pub struct PathedMaterialQuadlets {
    pub value: Vec<Matter>,
}

impl PathedMaterialQuadlets {
    pub const CODE: Codex = Codex::PathedMaterialQuadlets;

    pub fn new(value: Vec<Matter>) -> Self {
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
        for matter in self.value.iter() {
            out.push_str(&matter.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for matter in self.value.iter() {
            out.extend_from_slice(&matter.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for matter in self.value.iter() {
            out.extend_from_slice(&matter.qb2()?);
        }
        Ok(out)
    }
}
