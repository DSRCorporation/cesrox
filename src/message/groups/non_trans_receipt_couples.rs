use crate::error::CesrResult;
use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouples {
    pub value: Vec<NonTransReceiptCouple>,
}

impl Group<NonTransReceiptCouple> for NonTransReceiptCouples {
    const CODE: Codex = Codex::NonTransReceiptCouples;

    fn new(value: Vec<NonTransReceiptCouple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<NonTransReceiptCouple> {
        &self.value
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

impl GroupItem for NonTransReceiptCouple {
    fn qb64(&self) -> CesrResult<String> {
        let mut out = String::new();
        out.push_str(&self.verfer.qb64()?);
        out.push_str(&self.cigar.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.verfer.qb64b()?);
        out.extend_from_slice(&self.cigar.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> CesrResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.verfer.qb2()?);
        out.extend_from_slice(&self.cigar.qb2()?);
        Ok(out)
    }
}
