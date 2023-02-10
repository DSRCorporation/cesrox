use cesride::counter::Codex;
use cesride::Matter;
use crate::message::groups::Group;

#[derive(Debug, Clone, Default)]
pub struct AttachedMaterialQuadlets {
    pub value: Vec<Matter>,
}

impl Group<Matter> for AttachedMaterialQuadlets {
    const CODE: Codex = Codex::AttachedMaterialQuadlets;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}
