use nalgebra_glm::{Vec3, normalize};

use crate::ray::Ray;

pub const MIN_PITCH_DEGREES: f32 = -85.0;
pub const MAX_PITCH_DEGREES: f32 = 85.0;
pub const MIN_RADIUS: f32 = 4.0;
pub const MAX_RADIUS: f32 = 20.0;

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new(target: Vec3, yaw: f32, pitch: f32, radius: f32, fov: f32) -> Self {
        let mut camera = Camera {
            eye: target,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            yaw,
            pitch,
            radius,
            fov,
        };
        camera.update_eye();
        camera
    }

    fn update_eye(&mut self) {
        self.eye = self.target
            + Vec3::new(
                self.radius * self.pitch.cos() * self.yaw.cos(),
                self.radius * self.pitch.sin(),
                self.radius * self.pitch.cos() * self.yaw.sin(),
            );
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(
            MIN_PITCH_DEGREES.to_radians(),
            MAX_PITCH_DEGREES.to_radians(),
        );
        self.update_eye();
    }

    pub fn zoom(&mut self, delta: f32) {
        self.radius = (self.radius + delta).clamp(MIN_RADIUS, MAX_RADIUS);
        self.update_eye();
    }

    pub fn generate_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let forward = normalize(&(self.target - self.eye));
        let right = normalize(&forward.cross(&self.up));
        let camera_up = normalize(&right.cross(&forward));

        let aspect = width as f32 / height as f32;
        let tan_half_fov = (self.fov.to_radians() * 0.5).tan();

        let ndc_x = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * aspect * tan_half_fov;
        let ndc_y = (1.0 - 2.0 * (y as f32 + 0.5) / height as f32) * tan_half_fov;

        let direction = normalize(&(forward + right * ndc_x + camera_up * ndc_y));
        Ray::new(self.eye, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::vec3;

    #[test]
    fn eye_position_matches_spherical_coordinates() {
        let camera = Camera::new(vec3(0.0, 0.0, 0.0), 0.0, 0.0, 5.0, 60.0);
        assert!((camera.eye.x - 5.0).abs() < 1e-5);
        assert!(camera.eye.y.abs() < 1e-5);
        assert!(camera.eye.z.abs() < 1e-5);
    }

    #[test]
    fn orbit_updates_eye_position() {
        let mut camera = Camera::new(vec3(0.0, 0.0, 0.0), 0.0, 0.0, 5.0, 60.0);
        camera.orbit(std::f32::consts::FRAC_PI_2, 0.0);
        assert!(camera.eye.x.abs() < 1e-4);
        assert!((camera.eye.z - 5.0).abs() < 1e-4);
    }

    #[test]
    fn pitch_is_clamped_within_limits() {
        let mut camera = Camera::new(vec3(0.0, 0.0, 0.0), 0.0, 0.0, 5.0, 60.0);
        camera.orbit(0.0, std::f32::consts::PI);
        assert!((camera.pitch - MAX_PITCH_DEGREES.to_radians()).abs() < 1e-5);

        camera.orbit(0.0, -std::f32::consts::TAU);
        assert!((camera.pitch - MIN_PITCH_DEGREES.to_radians()).abs() < 1e-5);
    }

    #[test]
    fn radius_is_clamped_within_limits() {
        let mut camera = Camera::new(vec3(0.0, 0.0, 0.0), 0.0, 0.0, 5.0, 60.0);
        camera.zoom(100.0);
        assert!((camera.radius - MAX_RADIUS).abs() < 1e-5);

        camera.zoom(-100.0);
        assert!((camera.radius - MIN_RADIUS).abs() < 1e-5);
    }

    #[test]
    fn center_pixel_ray_points_toward_forward_direction() {
        let camera = Camera::new(vec3(0.0, 0.0, 0.0), 0.0, 0.0, 5.0, 60.0);
        let ray = camera.generate_ray(320, 180, 641, 361);
        let forward = normalize(&(camera.target - camera.eye));
        assert!((ray.direction - forward).norm() < 1e-4);
    }
}
