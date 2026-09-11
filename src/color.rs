#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn scale(&self, factor: f32) -> Color {
        Color::new(self.r * factor, self.g * factor, self.b * factor)
    }

    pub fn mul(&self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }

    pub fn add(&self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }

    pub fn clamp(&self) -> Color {
        Color::new(
            self.r.clamp(0.0, 1.0),
            self.g.clamp(0.0, 1.0),
            self.b.clamp(0.0, 1.0),
        )
    }

    pub fn to_u32(self) -> u32 {
        let clamped = self.clamp();
        let r = (clamped.r * 255.0).round() as u32;
        let g = (clamped.g * 255.0).round() as u32;
        let b = (clamped.b * 255.0).round() as u32;
        (r << 16) | (g << 8) | b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_multiplies_each_channel() {
        let c = Color::new(0.2, 0.4, 0.6).scale(2.0);
        assert_eq!((c.r, c.g, c.b), (0.4, 0.8, 1.2));
    }

    #[test]
    fn mul_is_component_wise() {
        let a = Color::new(0.5, 1.0, 0.0);
        let b = Color::new(0.5, 0.5, 0.5);
        let c = a.mul(b);
        assert_eq!((c.r, c.g, c.b), (0.25, 0.5, 0.0));
    }

    #[test]
    fn add_sums_each_channel() {
        let a = Color::new(0.1, 0.2, 0.3);
        let b = Color::new(0.1, 0.1, 0.1);
        let c = a.add(b);
        assert!((c.r - 0.2).abs() < 1e-6);
        assert!((c.g - 0.3).abs() < 1e-6);
        assert!((c.b - 0.4).abs() < 1e-6);
    }

    #[test]
    fn clamp_limits_channels_to_unit_range() {
        let c = Color::new(-0.5, 0.5, 1.5).clamp();
        assert_eq!((c.r, c.g, c.b), (0.0, 0.5, 1.0));
    }

    #[test]
    fn to_u32_packs_as_0xrrggbb() {
        let c = Color::new(1.0, 0.0, 0.0);
        assert_eq!(c.to_u32(), 0x00FF0000);

        let c = Color::new(0.0, 1.0, 0.0);
        assert_eq!(c.to_u32(), 0x0000FF00);

        let c = Color::new(0.0, 0.0, 1.0);
        assert_eq!(c.to_u32(), 0x000000FF);
    }

    #[test]
    fn to_u32_clamps_out_of_range_values() {
        let c = Color::new(2.0, -1.0, 0.5);
        assert_eq!(c.to_u32(), 0x00FF0080);
    }
}
