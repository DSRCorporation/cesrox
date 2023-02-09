use cesride::counter::Codex;
use cesride::Matter;

use crate::error::CesrResult;
use crate::Group;
use crate::groups::ControllerIdxSigs;

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroups {
    pub value: Vec<TransLastIdxSigGroup>,
}

impl TransLastIdxSigGroups {
    pub fn new(value: Vec<TransLastIdxSigGroup>) -> Self {
        Self { value }
    }
}

impl Group for TransLastIdxSigGroups {
     const CODE: Codex = Codex::TransLastIdxSigGroups;

    fn count(&self) -> u32 {
        self.value.len() as u32
    }

     fn to_string(&self) -> CesrResult<String> {
         let mut string = self.counter().qb64()?;
        for couple in self.value.iter() {
            string.push_str(&couple.prefixer.qb64()?);
            string.push_str(&couple.isigers.to_string()?);
        }
        Ok(string)
    }

     fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        self.to_string().map(|str| str.as_bytes().to_vec())
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
