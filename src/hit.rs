use nalgebra_glm::Vec3;

use crate::material::Material;

pub struct HitRecord {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}
