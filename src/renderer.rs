use nalgebra_glm::{Vec3, normalize};

use crate::color::Color;
use crate::cube::{Cube, cast_ray};
use crate::hit::HitRecord;
use crate::light::Light;
use crate::ray::Ray;

const SUNSET_HORIZON: Color = Color {
    r: 0.95,
    g: 0.55,
    b: 0.40,
};
const SKY_BLUE: Color = Color {
    r: 0.30,
    g: 0.45,
    b: 0.75,
};

pub fn lambert_factor(normal: Vec3, point: Vec3, light: &Light) -> f32 {
    let light_dir = normalize(&(light.position - point));
    normal.dot(&light_dir).max(0.0)
}

pub fn shade(hit: &HitRecord, light: &Light) -> Color {
    let factor = lambert_factor(hit.normal, hit.point, light);
    let surface_color = hit.material.diffuse.mul(light.color);
    surface_color.scale(factor * light.intensity).clamp()
}

pub fn background_color(direction: Vec3) -> Color {
    let t = 0.5 * (direction.y + 1.0);
    SUNSET_HORIZON.scale(1.0 - t).add(SKY_BLUE.scale(t))
}

pub fn trace(ray: &Ray, cubes: &[Cube], light: &Light) -> Color {
    match cast_ray(ray, cubes) {
        Some(hit) => shade(&hit, light),
        None => background_color(ray.direction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::vec3;

    fn white_light(position: Vec3) -> Light {
        Light::new(position, Color::new(1.0, 1.0, 1.0), 1.0)
    }

    #[test]
    fn lambert_factor_is_maximal_when_light_is_aligned_with_normal() {
        let normal = vec3(0.0, 1.0, 0.0);
        let point = vec3(0.0, 0.0, 0.0);
        let light = white_light(vec3(0.0, 5.0, 0.0));
        let factor = lambert_factor(normal, point, &light);
        assert!((factor - 1.0).abs() < 1e-5);
    }

    #[test]
    fn lambert_factor_is_zero_when_light_is_behind_surface() {
        let normal = vec3(0.0, 1.0, 0.0);
        let point = vec3(0.0, 0.0, 0.0);
        let light = white_light(vec3(0.0, -5.0, 0.0));
        let factor = lambert_factor(normal, point, &light);
        assert_eq!(factor, 0.0);
    }

    #[test]
    fn lambert_factor_scales_with_incidence_angle() {
        let normal = vec3(0.0, 1.0, 0.0);
        let point = vec3(0.0, 0.0, 0.0);
        let straight_light = white_light(vec3(0.0, 5.0, 0.0));
        let angled_light = white_light(vec3(3.0, 5.0, 0.0));

        let straight_factor = lambert_factor(normal, point, &straight_light);
        let angled_factor = lambert_factor(normal, point, &angled_light);

        assert!(straight_factor > angled_factor);
    }

    #[test]
    fn shade_combines_material_light_and_intensity() {
        let hit = HitRecord {
            distance: 1.0,
            point: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 1.0, 0.0),
            material: crate::material::Material::new(Color::new(0.5, 0.5, 0.5)),
        };
        let light = Light::new(vec3(0.0, 5.0, 0.0), Color::new(1.0, 1.0, 1.0), 0.5);

        let color = shade(&hit, &light);

        assert!((color.r - 0.25).abs() < 1e-5);
        assert!((color.g - 0.25).abs() < 1e-5);
        assert!((color.b - 0.25).abs() < 1e-5);
    }

    #[test]
    fn shade_result_is_clamped_to_unit_range() {
        let hit = HitRecord {
            distance: 1.0,
            point: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 1.0, 0.0),
            material: crate::material::Material::new(Color::new(1.0, 1.0, 1.0)),
        };
        let light = Light::new(vec3(0.0, 5.0, 0.0), Color::new(1.0, 1.0, 1.0), 5.0);

        let color = shade(&hit, &light);

        assert_eq!((color.r, color.g, color.b), (1.0, 1.0, 1.0));
    }

    #[test]
    fn background_color_is_sunset_when_looking_down() {
        let color = background_color(vec3(0.0, -1.0, 0.0));
        assert_eq!((color.r, color.g, color.b), (0.95, 0.55, 0.40));
    }

    #[test]
    fn background_color_is_sky_blue_when_looking_up() {
        let color = background_color(vec3(0.0, 1.0, 0.0));
        assert_eq!((color.r, color.g, color.b), (0.30, 0.45, 0.75));
    }

    #[test]
    fn trace_returns_procedural_background_when_nothing_is_hit() {
        let cubes: Vec<Cube> = vec![];
        let light = white_light(vec3(0.0, 5.0, 0.0));
        let ray = Ray::new(vec3(0.0, 0.0, -5.0), vec3(0.0, 0.0, 1.0));

        let traced = trace(&ray, &cubes, &light);
        let expected = background_color(ray.direction);

        assert_eq!(
            (traced.r, traced.g, traced.b),
            (expected.r, expected.g, expected.b)
        );
    }
}
