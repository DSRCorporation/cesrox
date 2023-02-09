use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::{Counter, Matter};

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouples {
    pub value: Vec<NonTransReceiptCouple>,
}

impl NonTransReceiptCouples {
    pub const CODE: Codex = Codex::NonTransReceiptCouples;

    pub fn new(value: Vec<NonTransReceiptCouple>) -> Self {
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
            out.push_str(&couple.verfer.qb64()?);
            out.push_str(&couple.cigar.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.verfer.qb64b()?);
            out.extend_from_slice(&couple.cigar.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.verfer.qb2()?);
            out.extend_from_slice(&couple.cigar.qb2()?);
        }
        Ok(out)
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
