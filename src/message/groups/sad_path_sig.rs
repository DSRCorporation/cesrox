use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::Group;

#[derive(Debug, Clone, Default)]
pub struct SadPathSig {
    pub value: Vec<Matter>,
}

impl Group<Matter> for SadPathSig {
    const CODE: Codex = Codex::SadPathSig;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}
