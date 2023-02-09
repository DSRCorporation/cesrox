use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::{Counter, Matter};

#[derive(Debug, Clone, Default)]
pub struct FirstSeenReplayCouples {
    pub value: Vec<FirstSeenReplayCouple>,
}

impl FirstSeenReplayCouples {
    pub const CODE: Codex = Codex::FirstSeenReplayCouples;

    pub fn new(value: Vec<FirstSeenReplayCouple>) -> Self {
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
            out.push_str(&couple.dater.qb64()?);
            out.push_str(&couple.firner.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.dater.qb64b()?);
            out.extend_from_slice(&couple.firner.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.dater.qb2()?);
            out.extend_from_slice(&couple.firner.qb2()?);
        }
        Ok(out)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FirstSeenReplayCouple {
    pub firner: Matter,
    pub dater: Matter,
}

impl FirstSeenReplayCouple {
    pub fn new(firner: Matter, dater: Matter) -> Self {
        Self { firner, dater }
    }
}
