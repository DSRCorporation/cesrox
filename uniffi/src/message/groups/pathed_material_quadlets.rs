use cesrox_core::error::CesrResult;
pub use cesrox_core::Matter;
pub use cesrox_core::message::groups::{
    PathedMaterialQuadlets,
};

pub fn pathed_material_quadlets_create(value: Vec<Matter>) -> PathedMaterialQuadlets {
    PathedMaterialQuadlets::new(value)
}

pub fn pathed_material_quadlets_qb64(pathed_material_quadlets: &PathedMaterialQuadlets) -> CesrResult<String> {
    pathed_material_quadlets.qb64()
}

pub fn pathed_material_quadlets_qb64b(pathed_material_quadlets: &PathedMaterialQuadlets) -> CesrResult<Vec<u8>> {
    pathed_material_quadlets.qb64b()
}

pub fn pathed_material_quadlets_qb2(pathed_material_quadlets: &PathedMaterialQuadlets) -> CesrResult<Vec<u8>> {
    pathed_material_quadlets.qb2()
}