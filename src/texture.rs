use crate::color::Color;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

fn hash2(x: u32, y: u32) -> f32 {
    let mut h = x
        .wrapping_mul(374_761_393)
        .wrapping_add(y.wrapping_mul(668_265_263));
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^= h >> 16;
    (h as f32) / (u32::MAX as f32)
}

impl Texture {
    pub fn from_pixels(width: usize, height: usize, pixels: Vec<Color>) -> Self {
        Texture {
            width,
            height,
            pixels,
        }
    }

    pub fn generate_rune_panel(size: usize) -> Self {
        const CELL_SIZE: usize = 32;
        const BORDER: usize = 3;

        let grout = Color::new(0.10, 0.02, 0.16);
        let dim_tile = Color::new(0.30, 0.06, 0.42);
        let bright_tile = Color::new(0.92, 0.35, 1.00);

        let mut pixels = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                let cell_x = (x / CELL_SIZE) as u32;
                let cell_y = (y / CELL_SIZE) as u32;
                let local_x = x % CELL_SIZE;
                let local_y = y % CELL_SIZE;

                let on_border = !(BORDER..CELL_SIZE - BORDER).contains(&local_x)
                    || !(BORDER..CELL_SIZE - BORDER).contains(&local_y);

                let color = if on_border {
                    grout
                } else {
                    let brightness = hash2(cell_x, cell_y);
                    let lit = hash2(cell_x.wrapping_add(97), cell_y.wrapping_add(59)) > 0.35;
                    if lit {
                        dim_tile
                            .scale(1.0 - brightness)
                            .add(bright_tile.scale(brightness))
                    } else {
                        dim_tile.scale(0.5 + brightness * 0.2)
                    }
                };
                pixels.push(color);
            }
        }

        Texture::from_pixels(size, size, pixels)
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let fx = u * (self.width - 1) as f32;
        let fy = v * (self.height - 1) as f32;

        let x0 = fx.floor() as usize;
        let y0 = fy.floor() as usize;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let tx = fx - x0 as f32;
        let ty = fy - y0 as f32;

        let c00 = self.pixels[y0 * self.width + x0];
        let c10 = self.pixels[y0 * self.width + x1];
        let c01 = self.pixels[y1 * self.width + x0];
        let c11 = self.pixels[y1 * self.width + x1];

        let top = c00.scale(1.0 - tx).add(c10.scale(tx));
        let bottom = c01.scale(1.0 - tx).add(c11.scale(tx));
        top.scale(1.0 - ty).add(bottom.scale(ty))
    }

    pub fn save_png(&self, path: &std::path::Path) {
        let mut image = image::RgbImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[y * self.width + x].clamp();
                image.put_pixel(
                    x as u32,
                    y as u32,
                    image::Rgb([
                        (color.r * 255.0).round() as u8,
                        (color.g * 255.0).round() as u8,
                        (color.b * 255.0).round() as u8,
                    ]),
                );
            }
        }
        image.save(path).expect("no se pudo guardar la textura");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_clamps_out_of_range_uv() {
        let texture = Texture::from_pixels(
            2,
            2,
            vec![
                Color::new(1.0, 0.0, 0.0),
                Color::new(0.0, 1.0, 0.0),
                Color::new(0.0, 0.0, 1.0),
                Color::new(1.0, 1.0, 1.0),
            ],
        );
        let inside = texture.sample(0.0, 0.0);
        let outside = texture.sample(-5.0, -5.0);
        assert_eq!(
            (inside.r, inside.g, inside.b),
            (outside.r, outside.g, outside.b)
        );
    }

    #[test]
    fn sample_interpolates_between_neighboring_texels() {
        let texture = Texture::from_pixels(
            2,
            1,
            vec![Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0)],
        );
        let mid = texture.sample(0.5, 0.0);
        assert!((mid.r - 0.5).abs() < 1e-5);
    }

    #[test]
    fn generate_rune_panel_produces_requested_size() {
        let texture = Texture::generate_rune_panel(64);
        assert_eq!(texture.width, 64);
        assert_eq!(texture.height, 64);
    }
}
