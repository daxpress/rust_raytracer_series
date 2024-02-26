pub use super::vec3::Vec3 as Color;

impl Color {
    #[inline(always)]
    pub fn r(&self) -> &f64 {
        self.x()
    }

    #[inline(always)]
    pub fn g(&self) -> &f64 {
        self.y()
    }

    #[inline(always)]
    pub fn b(&self) -> &f64 {
        self.z()
    }

    #[inline(always)]
    pub fn r_mut(&mut self) -> &mut f64 {
        self.x_mut()
    }

    #[inline(always)]
    pub fn g_mut(&mut self) -> &mut f64 {
        self.y_mut()
    }

    #[inline(always)]
    pub fn b_mut(&mut self) -> &mut f64 {
        self.z_mut()
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

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    #[inline(always)]
    pub fn linear_to_gamma(color: &Color) -> Color {
        Color::new(color.x().sqrt(), color.y().sqrt(), color.z().sqrt())
    }

    #[inline(always)]
    pub fn gamma_to_linear(color: &Color) -> Color {
        Color::new(color.x().powi(2), color.y().powi(2), color.z().powi(2))
    }
}
