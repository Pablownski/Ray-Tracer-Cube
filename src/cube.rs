use nalgebra_glm::{Vec3, vec3};

use crate::hit::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

pub const EPSILON: f32 = 0.001;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Cube { min, max, material }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;

        for axis in 0..3 {
            let origin = ray.origin[axis];
            let direction = ray.direction[axis];
            let min = self.min[axis];
            let max = self.max[axis];

            let mut t0 = (min - origin) / direction;
            let mut t1 = (max - origin) / direction;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_near = t_near.max(t0);
            t_far = t_far.min(t1);
        }

        if t_far < t_near || t_far <= 0.0 {
            return None;
        }

        let distance = if t_near > EPSILON { t_near } else { t_far };
        let point = ray.at(distance);
        let normal = self.normal_at(point);

        Some(HitRecord {
            distance,
            point,
            normal,
            material: self.material,
        })
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        if (point.x - self.min.x).abs() < EPSILON {
            vec3(-1.0, 0.0, 0.0)
        } else if (point.x - self.max.x).abs() < EPSILON {
            vec3(1.0, 0.0, 0.0)
        } else if (point.y - self.min.y).abs() < EPSILON {
            vec3(0.0, -1.0, 0.0)
        } else if (point.y - self.max.y).abs() < EPSILON {
            vec3(0.0, 1.0, 0.0)
        } else if (point.z - self.min.z).abs() < EPSILON {
            vec3(0.0, 0.0, -1.0)
        } else {
            vec3(0.0, 0.0, 1.0)
        }
    }
}

pub fn cast_ray(ray: &Ray, cubes: &[Cube]) -> Option<HitRecord> {
    let mut closest: Option<HitRecord> = None;

    for cube in cubes {
        if let Some(hit) = cube.intersect(ray) {
            let is_closer = match &closest {
                Some(current) => hit.distance < current.distance,
                None => true,
            };
            if is_closer {
                closest = Some(hit);
            }
        }
    }

    closest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    fn test_cube() -> Cube {
        Cube::new(
            vec3(-1.0, -1.0, -1.0),
            vec3(1.0, 1.0, 1.0),
            Material::new(Color::new(1.0, 1.0, 1.0)),
        )
    }

    #[test]
    fn frontal_ray_hits_cube() {
        let cube = test_cube();
        let ray = Ray::new(vec3(0.0, 0.0, -5.0), vec3(0.0, 0.0, 1.0));
        let hit = cube.intersect(&ray).expect("debe golpear");
        assert!((hit.distance - 4.0).abs() < EPSILON);
        assert_eq!(hit.normal, vec3(0.0, 0.0, -1.0));
    }

    #[test]
    fn lateral_ray_hits_cube() {
        let cube = test_cube();
        let ray = Ray::new(vec3(-5.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0));
        let hit = cube.intersect(&ray).expect("debe golpear");
        assert!((hit.distance - 4.0).abs() < EPSILON);
        assert_eq!(hit.normal, vec3(-1.0, 0.0, 0.0));
    }

    #[test]
    fn top_ray_hits_cube() {
        let cube = test_cube();
        let ray = Ray::new(vec3(0.0, 5.0, 0.0), vec3(0.0, -1.0, 0.0));
        let hit = cube.intersect(&ray).expect("debe golpear");
        assert!((hit.distance - 4.0).abs() < EPSILON);
        assert_eq!(hit.normal, vec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_missing_cube_returns_none() {
        let cube = test_cube();
        let ray = Ray::new(vec3(0.0, 5.0, -5.0), vec3(0.0, 0.0, 1.0));
        assert!(cube.intersect(&ray).is_none());
    }

    #[test]
    fn cube_behind_ray_origin_returns_none() {
        let cube = test_cube();
        let ray = Ray::new(vec3(0.0, 0.0, -5.0), vec3(0.0, 0.0, -1.0));
        assert!(cube.intersect(&ray).is_none());
    }

    #[test]
    fn ray_starting_inside_cube_hits_exit_face() {
        let cube = test_cube();
        let ray = Ray::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0));
        let hit = cube
            .intersect(&ray)
            .expect("debe golpear la cara de salida");
        assert!((hit.distance - 1.0).abs() < EPSILON);
        assert_eq!(hit.normal, vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn cast_ray_returns_none_without_cubes() {
        let ray = Ray::new(vec3(0.0, 0.0, -5.0), vec3(0.0, 0.0, 1.0));
        assert!(cast_ray(&ray, &[]).is_none());
    }

    #[test]
    fn cast_ray_picks_the_closest_of_multiple_cubes() {
        let far_cube = Cube::new(
            vec3(-1.0, -1.0, 4.0),
            vec3(1.0, 1.0, 6.0),
            Material::new(Color::new(0.0, 1.0, 0.0)),
        );
        let near_cube = Cube::new(
            vec3(-1.0, -1.0, -1.0),
            vec3(1.0, 1.0, 1.0),
            Material::new(Color::new(1.0, 0.0, 0.0)),
        );
        let ray = Ray::new(vec3(0.0, 0.0, -5.0), vec3(0.0, 0.0, 1.0));
        let cubes = vec![far_cube, near_cube];
        let hit = cast_ray(&ray, &cubes).expect("debe golpear el cubo mas cercano");
        assert!((hit.distance - 4.0).abs() < EPSILON);
        assert_eq!(hit.material.diffuse.r, 1.0);
    }
}
