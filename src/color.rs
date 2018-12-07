pub use std::f64::{EPSILON};
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color{r: r, g: g, b: b}
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        (self.r - other.r).abs() <= EPSILON &&
            (self.g - other.g).abs() <= EPSILON &&
            (self.b - other.b).abs() <= EPSILON
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, c: Color) -> Color {
        Color::new(self.r + c.r,
                     self.g + c.g,
                     self.b + c.b)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, c: Color) -> Color {
        Color::new(self.r - c.r,
                     self.g - c.g,
                     self.b - c.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, f: f64) -> Color {
        Color::new(self.r * f, self.g * f, self.b * f)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, c: Color) -> Color {
        Color::new(self.r * c.r, self.g * c.g, self.b * c.b)
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_equal() {
        let c1 = Color::new(0.5, 0.25, 0.1);
        let c2 = Color::new(0.5, 0.25, 0.1);
        assert_eq!(c1, c2);
    }

    #[test]
    fn color_plus_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let nc = Color::new(1.6, 0.7, 1.0);
        assert_eq!(nc, c1 + c2);
    }

    #[test]
    fn color_sub_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let nc = Color::new(0.2, 0.5, 0.5);
        assert_eq!(nc, c1 - c2);
    }

    #[test]
    fn color_mul_scaler() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let nc = Color::new(0.4, 0.6, 0.8);
        assert_eq!(nc, c1 * 2.0);
    }

    #[test]
    fn color_mul_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 1.0);
        let nc = Color::new(0.9, 0.2, 0.4);
        assert_eq!(nc, c1 * c2);
    }
}
