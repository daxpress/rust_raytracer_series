use std::f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && self.max >= value
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && self.max > value
    }

    pub fn clamp(&self, value: f64) -> f64 {
        match value {
            _ if value < self.min => self.min,
            _ if value > self.max => self.max,
            n => n
        }
    }

    #[inline(always)]
    pub fn min(&self) -> f64 {
        self.min
    }

    #[inline(always)]
    pub fn max(&self) -> f64 {
        self.max
    }

    #[inline(always)]
    pub const fn empty() -> Self {
        Interval {
            min: INFINITY,
            max: NEG_INFINITY
        }
    }

    #[inline(always)]
    pub const fn universe() -> Self {
        Interval {
            min: NEG_INFINITY,
            max: INFINITY
        }
    }

}