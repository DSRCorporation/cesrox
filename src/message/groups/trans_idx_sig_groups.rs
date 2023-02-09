use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::Group;
use crate::groups::ControllerIdxSigs;

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroups {
    pub value: Vec<TransIdxSigGroup>,
}

impl TransIdxSigGroups {
    pub fn new(value: Vec<TransIdxSigGroup>) -> Self {
        Self { value }
    }
}

impl Group for TransIdxSigGroups {
    const CODE: Codex = Codex::TransIdxSigGroups;

    fn count(&self) -> u32 {
        self.value.len() as u32
    }

     fn to_string(&self) -> CesrResult<String> {
         let mut string = self.counter().qb64()?;
        for couple in self.value.iter() {
            string.push_str(&couple.seqner.qb64()?);
            string.push_str(&couple.saider.qb64()?);
            string.push_str(&couple.saider.qb64()?);
            string.push_str(&couple.isigers.to_string()?);
        }
        Ok(string)
    }

     fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        self.to_string().map(|str| str.as_bytes().to_vec())
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
