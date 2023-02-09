use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::Group;

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouples {
    pub value: Vec<NonTransReceiptCouple>,
}

impl NonTransReceiptCouples {
    pub fn new(value: Vec<NonTransReceiptCouple>) -> Self {
        Self { value }
    }
}

impl Group for NonTransReceiptCouples {
    const CODE: Codex = Codex::NonTransReceiptCouples;

    fn count(&self) -> u32 {
        self.value.len() as u32
    }

    fn to_string(&self) -> CesrResult<String> {
        let mut string = self.counter().qb64()?;
        for couple in self.value.iter() {
            string.push_str(&couple.verfer.qb64()?);
            string.push_str(&couple.cigar.qb64()?);
        }
        Ok(string)
    }

    fn to_bytes(&self) -> CesrResult<Vec<u8>> {
        self.to_string().map(|str| str.as_bytes().to_vec())
    }
}

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouple {
    pub verfer: Matter,
    pub cigar: Matter,
}

impl NonTransReceiptCouple {
    pub fn new(verfer: Matter, cigar: Matter) -> Self {
        Self { verfer, cigar }
    }
}
