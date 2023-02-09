use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::Group;

#[derive(Debug, Clone, Default)]
pub struct FirstSeenReplayCouples {
    pub value: Vec<FirstSeenReplayCouple>,
}

impl FirstSeenReplayCouples {
    pub fn new(value: Vec<FirstSeenReplayCouple>) -> Self {
        Self { value }
    }
}

impl Group for FirstSeenReplayCouples {
    const CODE: Codex = Codex::FirstSeenReplayCouples;

    fn count(&self) -> u32 {
        self.value.len() as u32
    }

    fn to_string(&self) -> CesrResult<String> {
        let mut string = self.counter().qb64()?;
        for couple in self.value.iter() {
            string.push_str(&couple.dater.qb64()?);
            string.push_str(&couple.firner.qb64()?);
        }
        Ok(string)
    }

    fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        self.to_string().map(|str| str.as_bytes().to_vec())
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
