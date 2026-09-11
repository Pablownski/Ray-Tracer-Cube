use std::rc::Rc;

use crate::color::Color;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Material {
    pub diffuse: Color,
    pub texture: Option<Rc<Texture>>,
    pub emissive_strength: f32,
}

impl Material {
    pub fn new(diffuse: Color) -> Self {
        Material {
            diffuse,
            texture: None,
            emissive_strength: 0.0,
        }
    }

    pub fn textured(texture: Rc<Texture>, emissive_strength: f32) -> Self {
        Material {
            diffuse: Color::new(1.0, 1.0, 1.0),
            texture: Some(texture),
            emissive_strength,
        }
    }
}
