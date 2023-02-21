use crate::message::group::CesrGroup;
use cesrox::error::ParsideResult;
use cesrox::message::groups::AttachedMaterialQuadlets as ParsideAttachedMaterialQuadlets;
use cesrox::Group;

#[derive(Clone)]
pub struct AttachedMaterialQuadlets {
    pub value: Vec<CesrGroup>,
}

pub fn attached_material_quadlets_create(
    value: Vec<CesrGroup>,
) -> AttachedMaterialQuadlets {
    AttachedMaterialQuadlets { value: value.into_iter().map(|val| val.into()).collect() }
}

// FIXME: Try to avoid cloning
pub fn attached_material_quadlets_qb64(
    attached_material_quadlets: &AttachedMaterialQuadlets,
) -> ParsideResult<String> {
    let attached_material_quadlets: ParsideAttachedMaterialQuadlets = attached_material_quadlets.clone().into();
    attached_material_quadlets.qb64()
}

pub fn attached_material_quadlets_qb64b(
    attached_material_quadlets: &AttachedMaterialQuadlets,
) -> ParsideResult<Vec<u8>> {
    let attached_material_quadlets: ParsideAttachedMaterialQuadlets = attached_material_quadlets.clone().into();
    attached_material_quadlets.qb64b()
}

pub fn attached_material_quadlets_qb2(
    attached_material_quadlets: &AttachedMaterialQuadlets,
) -> ParsideResult<Vec<u8>> {
    let attached_material_quadlets: ParsideAttachedMaterialQuadlets = attached_material_quadlets.clone().into();
    attached_material_quadlets.qb2()
}

impl From<ParsideAttachedMaterialQuadlets> for AttachedMaterialQuadlets {
    fn from(group: ParsideAttachedMaterialQuadlets) -> AttachedMaterialQuadlets {
        AttachedMaterialQuadlets {
            value: group.value.into_iter().map(CesrGroup::from).collect()
        }
    }
}

impl Into<ParsideAttachedMaterialQuadlets> for AttachedMaterialQuadlets {
    fn into(self) -> ParsideAttachedMaterialQuadlets {
        ParsideAttachedMaterialQuadlets {
            value: self.value.into_iter().map(|value| value.into()).collect()
        }
    }
}
