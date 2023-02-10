use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::Group;

#[derive(Debug, Clone, Default)]
pub struct ControllerIdxSigs {
    pub value: Vec<Matter>,
}

impl Group<Matter> for ControllerIdxSigs {
    const CODE: Codex = Codex::ControllerIdxSigs;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}

