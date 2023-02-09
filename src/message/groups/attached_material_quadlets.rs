use cesride::counter::Codex;
use cesride::Matter;
use crate::error::CesrResult;
use crate::Group;

#[derive(Debug, Clone, Default)]
pub struct AttachedMaterialQuadlets {
    pub value: Vec<Matter>,
}

impl AttachedMaterialQuadlets {
    pub fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }
}

impl Group for AttachedMaterialQuadlets {
    const CODE: Codex = Codex::AttachedMaterialQuadlets;

    fn count(&self) -> u32 {
        self.value.len() as u32
    }

    fn to_string(&self) -> CesrResult<String> {
        let mut string = self.counter().qb64()?;
        for matter in self.value.iter() {
            string.push_str(&matter.qb64()?);
        }
        Ok(string)
    }

    fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        self.to_string().map(|str| str.as_bytes().to_vec())
    }
}
