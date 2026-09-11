mod camera;
mod color;
mod cube;
mod framebuffer;
mod hit;
mod light;
mod material;
mod ray;
mod renderer;
mod scene;

use camera::Camera;
use color::Color;
use cube::Cube;
use framebuffer::Framebuffer;
use light::Light;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const FOV_DEGREES: f32 = 60.0;
const INITIAL_YAW: f32 = -std::f32::consts::FRAC_PI_2 + 0.35;
const INITIAL_PITCH: f32 = 0.15;
const INITIAL_RADIUS: f32 = 6.5;
const ORBIT_SPEED: f32 = 0.03;
const ZOOM_SPEED: f32 = 0.15;
const CAMERA_TARGET_HEIGHT: f32 = 1.2;

fn new_camera() -> Camera {
    Camera::new(
        vec3(0.0, CAMERA_TARGET_HEIGHT, 0.0),
        INITIAL_YAW,
        INITIAL_PITCH,
        INITIAL_RADIUS,
        FOV_DEGREES,
    )
}

fn build_light() -> Light {
    Light::new(vec3(2.5, 5.0, -6.0), Color::new(1.0, 0.97, 0.9), 1.4)
}

fn render(framebuffer: &mut Framebuffer, camera: &Camera, cubes: &[Cube], light: &Light) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let ray = camera.generate_ray(x, y, framebuffer.width, framebuffer.height);
            let color = renderer::trace(&ray, cubes, light);
            framebuffer.set_pixel(x, y, color.to_u32());
        }
    }
}

fn write_ppm(framebuffer: &Framebuffer, path: &std::path::Path) {
    use std::io::Write;

    let mut file = std::fs::File::create(path).expect("no se pudo crear el archivo del frame");
    write!(
        file,
        "P6\n{} {}\n255\n",
        framebuffer.width, framebuffer.height
    )
    .expect("no se pudo escribir el encabezado PPM");

    let mut bytes = Vec::with_capacity(framebuffer.width * framebuffer.height * 3);
    for &pixel in &framebuffer.buffer {
        bytes.push(((pixel >> 16) & 0xFF) as u8);
        bytes.push(((pixel >> 8) & 0xFF) as u8);
        bytes.push((pixel & 0xFF) as u8);
    }
    file.write_all(&bytes)
        .expect("no se pudo escribir los pixeles del frame");
}

const GIF_ORBIT_AMPLITUDE_DEGREES: f32 = 22.0;

fn capture_orbit_frames(output_dir: &std::path::Path, frame_count: usize) {
    std::fs::create_dir_all(output_dir).expect("no se pudo crear el directorio de salida");

    let cubes = scene::build_scene();
    let light = build_light();
    let amplitude = GIF_ORBIT_AMPLITUDE_DEGREES.to_radians();

    for i in 0..frame_count {
        let phase = (i as f32 / frame_count as f32) * std::f32::consts::TAU;
        let yaw = INITIAL_YAW + amplitude * phase.sin();
        let camera = Camera::new(
            vec3(0.0, CAMERA_TARGET_HEIGHT, 0.0),
            yaw,
            INITIAL_PITCH,
            INITIAL_RADIUS,
            FOV_DEGREES,
        );

        let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
        render(&mut framebuffer, &camera, &cubes, &light);

        let path = output_dir.join(format!("frame_{i:04}.ppm"));
        write_ppm(&framebuffer, &path);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(gif_index) = args.iter().position(|a| a == "--gif") {
        let output_dir = args
            .get(gif_index + 1)
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from("target/gif_frames"));
        let frame_count = args
            .get(gif_index + 2)
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);

        capture_orbit_frames(&output_dir, frame_count);
        println!(
            "{} frames escritos en {}",
            frame_count,
            output_dir.display()
        );
        return;
    }

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let cubes = scene::build_scene();
    let light = build_light();

    let mut camera = new_camera();
    let mut dirty = true;

    let mut window = Window::new(
        "Cubo con Raytracing",
        framebuffer.width,
        framebuffer.height,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            camera.orbit(-ORBIT_SPEED, 0.0);
            dirty = true;
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(ORBIT_SPEED, 0.0);
            dirty = true;
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, ORBIT_SPEED);
            dirty = true;
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, -ORBIT_SPEED);
            dirty = true;
        }
        if window.is_key_down(Key::Q) {
            camera.zoom(-ZOOM_SPEED);
            dirty = true;
        }
        if window.is_key_down(Key::E) {
            camera.zoom(ZOOM_SPEED);
            dirty = true;
        }
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            camera = new_camera();
            dirty = true;
        }

        if dirty {
            render(&mut framebuffer, &camera, &cubes, &light);
            dirty = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .expect("No se pudo actualizar la ventana");
    }
}
