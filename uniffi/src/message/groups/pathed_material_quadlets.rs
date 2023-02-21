use cesrox::error::ParsideResult;
use cesrox::Group;
use cesrox::Siger;
pub use cesrox::message::groups::{
    PathedMaterialQuadlets,
    PathedMaterialQuadlet,
};

pub fn pathed_material_quadlet_create(siger: Siger) -> PathedMaterialQuadlet {
    PathedMaterialQuadlet::new(siger)
}

pub fn pathed_material_quadlets_create(value: Vec<PathedMaterialQuadlet>) -> PathedMaterialQuadlets {
    PathedMaterialQuadlets::new(value)
}

pub fn pathed_material_quadlets_qb64(pathed_material_quadlets: &PathedMaterialQuadlets) -> ParsideResult<String> {
    pathed_material_quadlets.qb64()
}

pub fn pathed_material_quadlets_qb64b(pathed_material_quadlets: &PathedMaterialQuadlets) -> ParsideResult<Vec<u8>> {
    pathed_material_quadlets.qb64b()
}

pub fn pathed_material_quadlets_qb2(pathed_material_quadlets: &PathedMaterialQuadlets) -> ParsideResult<Vec<u8>> {
    pathed_material_quadlets.qb2()
}