use nalgebra_glm::{Vec3, vec3};

use crate::color::Color;
use crate::cube::Cube;
use crate::material::Material;

const STONE: Color = Color {
    r: 0.25,
    g: 0.27,
    b: 0.32,
};
const GRASS: Color = Color {
    r: 0.22,
    g: 0.55,
    b: 0.18,
};
const TRUNK: Color = Color {
    r: 0.35,
    g: 0.16,
    b: 0.07,
};
const LEAVES: Color = Color {
    r: 0.10,
    g: 0.38,
    b: 0.16,
};
const CUBE_MAIN: Color = Color {
    r: 0.42,
    g: 0.08,
    b: 0.65,
};
const CUBE_DETAIL: Color = Color {
    r: 0.85,
    g: 0.20,
    b: 1.00,
};

const MAIN_CUBE_HALF_SIZE: f32 = 1.2;
const MAIN_CUBE_MIN_Y: f32 = 0.0;
const MAIN_CUBE_MAX_Y: f32 = MAIN_CUBE_MIN_Y + MAIN_CUBE_HALF_SIZE * 2.0;
const FRONT_FACE_Z: f32 = -MAIN_CUBE_HALF_SIZE;

fn cube(min: Vec3, max: Vec3, color: Color) -> Cube {
    Cube::new(min, max, Material::new(color))
}

fn tree(cubes: &mut Vec<Cube>, base: Vec3) {
    cubes.push(cube(
        base + vec3(-0.15, 0.0, -0.15),
        base + vec3(0.15, 1.2, 0.15),
        TRUNK,
    ));
    cubes.push(cube(
        base + vec3(-0.5, 1.2, -0.5),
        base + vec3(0.5, 1.9, 0.5),
        LEAVES,
    ));
}

fn rock(cubes: &mut Vec<Cube>, center: Vec3, half_size: f32) {
    cubes.push(cube(
        center - vec3(half_size, half_size, half_size),
        center + vec3(half_size, half_size, half_size),
        STONE,
    ));
}

fn front_face_rune_grid(cubes: &mut Vec<Cube>, center_y: f32) {
    let bar_depth = (FRONT_FACE_Z - 0.02, FRONT_FACE_Z - 0.006);
    let half = MAIN_CUBE_HALF_SIZE - 0.15;
    let thin = 0.035;
    let thick = 0.06;

    let border_thickness = 0.05;
    cubes.push(cube(
        vec3(-half, center_y - half, bar_depth.0),
        vec3(half, -half + border_thickness + center_y, bar_depth.1),
        CUBE_DETAIL,
    ));
    cubes.push(cube(
        vec3(-half, half - border_thickness + center_y, bar_depth.0),
        vec3(half, half + center_y, bar_depth.1),
        CUBE_DETAIL,
    ));
    cubes.push(cube(
        vec3(-half, center_y - half, bar_depth.0),
        vec3(-half + border_thickness, center_y + half, bar_depth.1),
        CUBE_DETAIL,
    ));
    cubes.push(cube(
        vec3(half - border_thickness, center_y - half, bar_depth.0),
        vec3(half, center_y + half, bar_depth.1),
        CUBE_DETAIL,
    ));

    let verticals = [-0.65, -0.32, 0.0, 0.32, 0.65];
    for (i, x) in verticals.iter().enumerate() {
        let width = if i == 2 { thick } else { thin };
        cubes.push(cube(
            vec3(x - width, center_y - half, bar_depth.0),
            vec3(x + width, center_y + half, bar_depth.1),
            CUBE_DETAIL,
        ));
    }

    let horizontals = [-0.65, -0.32, 0.0, 0.32, 0.65];
    for (i, y) in horizontals.iter().enumerate() {
        let width = if i == 2 { thick } else { thin };
        cubes.push(cube(
            vec3(-half, center_y + y - width, bar_depth.0),
            vec3(half, center_y + y + width, bar_depth.1),
            CUBE_DETAIL,
        ));
    }

    let corner_dot = 0.09;
    for x in [-0.65, 0.65] {
        for y in [-0.65, 0.65] {
            cubes.push(cube(
                vec3(x - corner_dot, center_y + y - corner_dot, bar_depth.0),
                vec3(x + corner_dot, center_y + y + corner_dot, bar_depth.1),
                CUBE_DETAIL,
            ));
        }
    }
}

pub fn build_scene() -> Vec<Cube> {
    let mut cubes = vec![
        cube(vec3(-60.0, -0.5, -60.0), vec3(60.0, 0.0, 60.0), GRASS),
        cube(
            vec3(-MAIN_CUBE_HALF_SIZE, MAIN_CUBE_MIN_Y, -MAIN_CUBE_HALF_SIZE),
            vec3(MAIN_CUBE_HALF_SIZE, MAIN_CUBE_MAX_Y, MAIN_CUBE_HALF_SIZE),
            CUBE_MAIN,
        ),
    ];

    front_face_rune_grid(&mut cubes, MAIN_CUBE_MIN_Y + MAIN_CUBE_HALF_SIZE);

    let base_rock_positions = [
        (vec3(1.55, 0.0, -1.35), 0.26),
        (vec3(-1.5, 0.0, -1.4), 0.24),
        (vec3(1.65, 0.0, 1.15), 0.28),
        (vec3(-1.55, 0.0, 1.2), 0.22),
        (vec3(0.15, 0.0, -1.65), 0.20),
    ];
    for (position, half_size) in base_rock_positions {
        rock(&mut cubes, position + vec3(0.0, half_size, 0.0), half_size);
    }

    let distant_rock_positions = [
        vec3(3.6, 0.15, 2.8),
        vec3(-3.8, 0.15, -2.4),
        vec3(4.2, 0.15, -3.0),
    ];
    for position in distant_rock_positions {
        rock(&mut cubes, position, 0.18);
    }

    tree(&mut cubes, vec3(-5.0, 0.0, -3.0));
    tree(&mut cubes, vec3(5.0, 0.0, -4.0));

    cubes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_contains_the_main_cube_and_its_landscape() {
        let cubes = build_scene();
        assert_eq!(cubes.len(), 1 + 1 + 18 + 5 + 3 + 4);
    }

    #[test]
    fn main_cube_sits_directly_on_the_ground() {
        let cubes = build_scene();
        let main_cube = &cubes[1];
        assert!((main_cube.min.y - MAIN_CUBE_MIN_Y).abs() < 1e-5);
    }
}
