pub use super::vec3::Vec3 as Color;

impl Color {
    
    pub fn r(&self) -> &f64 {
        self.x()
    }

    pub fn g(&self) -> &f64 {
        self.y()
    }

    pub fn b(&self) -> &f64 {
        self.z()
    }

    pub fn black() -> Color {
        Color::zero()
    }
    pub fn white() -> Color {
        Color::one()
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }
}