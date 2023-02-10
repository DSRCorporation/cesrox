use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct FirstSeenReplayCouples {
    pub value: Vec<FirstSeenReplayCouple>,
}

impl Group<FirstSeenReplayCouple> for FirstSeenReplayCouples {
    const CODE: Codex = Codex::FirstSeenReplayCouples;

    fn new(value: Vec<FirstSeenReplayCouple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<FirstSeenReplayCouple> {
        &self.value
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


impl GroupItem for FirstSeenReplayCouple {
    fn qb64(&self) -> CesrResult<String> {
        let mut out = String::new();
        out.push_str(&self.dater.qb64()?);
        out.push_str(&self.firner.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.dater.qb64b()?);
        out.extend_from_slice(&self.firner.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.dater.qb2()?);
        out.extend_from_slice(&self.firner.qb2()?);
        Ok(out)
    }
}
