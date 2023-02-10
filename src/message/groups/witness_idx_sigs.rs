use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::Group;

#[derive(Debug, Clone, Default)]
pub struct WitnessIdxSigs {
    pub value: Vec<Matter>,
}

impl Group<Matter> for WitnessIdxSigs {
    const CODE: Codex = Codex::WitnessIdxSigs;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}
