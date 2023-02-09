use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::Group;

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouples {
    pub value: Vec<SealSourceCouple>,
}

impl SealSourceCouples {
    pub fn new(value: Vec<SealSourceCouple>) -> Self {
        Self { value }
    }
}

impl Group for SealSourceCouples {
    const CODE: Codex = Codex::SealSourceCouples;

    fn count(&self) -> u32 {
        self.value.len() as u32
    }

    fn to_string(&self) -> CesrResult<String> {
        let mut string = self.counter().qb64()?;
        for couple in self.value.iter() {
            string.push_str(&couple.seqner.qb64()?);
            string.push_str(&couple.saider.qb64()?);
        }
        Ok(string)
    }

    fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        self.to_string().map(|str| str.as_bytes().to_vec())
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
