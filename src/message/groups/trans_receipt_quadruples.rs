use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::Group;

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruples {
    pub value: Vec<TransReceiptQuadruple>,
}

impl TransReceiptQuadruples {
    pub fn new(value: Vec<TransReceiptQuadruple>) -> Self {
        Self { value }
    }
}

impl Group for TransReceiptQuadruples {
    const CODE: Codex = Codex::TransReceiptQuadruples;

    fn count(&self) -> u32 {
        self.value.len() as u32
    }

    fn to_string(&self) -> CesrResult<String> {
        let mut string = self.counter().qb64()?;
        for couple in self.value.iter() {
            string.push_str(&couple.prefixer.qb64()?);
            string.push_str(&couple.seqner.qb64()?);
            string.push_str(&couple.saider.qb64()?);
            string.push_str(&couple.siger.qb64()?);
        }
        Ok(string)
    }

    fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        self.to_string().map(|str| str.as_bytes().to_vec())
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruple {
    pub prefixer: Matter,
    pub seqner: Matter,
    pub saider: Matter,
    pub siger: Matter,
}

impl TransReceiptQuadruple {
    pub fn new(prefixer: Matter, seqner: Matter, saider: Matter, siger: Matter) -> Self {
        Self {
            prefixer,
            seqner,
            saider,
            siger,
        }
    }
}
