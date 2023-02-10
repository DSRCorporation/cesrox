use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::Group;

#[derive(Debug, Clone, Default)]
pub struct SadPathSigGroup {
    pub value: Vec<Matter>,
}

impl Group<Matter> for SadPathSigGroup {
    const CODE: Codex = Codex::SadPathSigGroup;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}
