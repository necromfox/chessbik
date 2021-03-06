use bevy::prelude::{Handle, Mesh};

pub mod material_ty;
pub use material_ty::*;

pub mod planes_materials;
pub use planes_materials::*;

pub mod pieces_materials;
pub use pieces_materials::*;

pub mod pieces_meshes;
pub use pieces_meshes::*;

pub mod impls;
pub use impls::*;

pub struct AppAssets {
    pub planes: PlanesMaterials,
    pub pieces: PiecesMaterials,
    pub selected: MaterialTy,
    pub plane_mesh: Handle<Mesh>,
    pub pieces_meshes: PiecesMeshes,
    pub move_indicator_material: MaterialTy,
    pub mage_move_mesh: Handle<Mesh>,
    pub mage_move_material: MaterialTy,
    pub rotator_mesh: Handle<Mesh>,
    pub rotator_available_material: MaterialTy,
    pub rotator_available_highlighted_material: MaterialTy,
    pub rotator_active_material: MaterialTy,
    pub rotator_unavailable_material: MaterialTy,
}
