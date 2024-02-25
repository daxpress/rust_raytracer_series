use std::ops::{Add, Div, Mul, Sub, Neg, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    fields: [f64; 3]
}

impl Vec3 {
    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn new(x: f64, y:f64, z:f64) -> Self {
        Vec3 {
            fields: [x, y, z]
        }
    }

    pub fn x(&self) -> &f64 {
        &self.fields[0]
    }

    pub fn y(&self) -> &f64 {
        &self.fields[1]
    }

    pub fn z(&self) -> &f64 {
        &self.fields[2]
    }

    pub fn len(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs.x() * rhs.x() + 
        lhs.y() * rhs.y() + 
        lhs.z() * rhs.z()
    }

    pub fn cross(lhs: &Self, rhs: &Self) -> Self {
        Vec3 {
            fields: [
                lhs.y() * rhs.z() - lhs.z() * rhs.y(),
                lhs.z() * rhs.x() - lhs.x() * rhs.z(),
                lhs.x() * rhs.y() - lhs.y() * rhs.x()
            ]
        }
    }

    pub fn normalize(&mut self) {
        *self /= self.len()
    }

    pub fn normalized(&self) -> Self {
        let temp = self.clone() / self.len();
        temp
    }

}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            fields: [
                -self.fields[0],
                -self.fields[1],
                -self.fields[2]
            ]
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.fields[0], self.fields[1], self.fields[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            fields: [
                self.fields[0] + rhs.fields[0],
                self.fields[1] + rhs.fields[1],
                self.fields[2] + rhs.fields[2],
            ]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.fields[0] += rhs.fields[0];
        self.fields[1] += rhs.fields[1];
        self.fields[2] += rhs.fields[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            fields: [
                self.fields[0] - rhs.fields[0],
                self.fields[1] - rhs.fields[1],
                self.fields[2] - rhs.fields[2],
            ]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.fields[0] -= rhs.fields[0];
        self.fields[1] -= rhs.fields[1];
        self.fields[2] -= rhs.fields[2];
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            fields: [
                self.fields[0] * rhs.fields[0],
                self.fields[1] * rhs.fields[1],
                self.fields[2] * rhs.fields[2],
            ]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            fields: [
                self.fields[0] * rhs,
                self.fields[1] * rhs,
                self.fields[2] * rhs,
            ]
        }
    }
}
    
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            rhs.x() * self,
            rhs.y() * self,
            rhs.z() * self
        )
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.fields[0] *= rhs;
        self.fields[1] *= rhs;
        self.fields[2] *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    
    fn div(self, rhs: f64) -> Self::Output {
        (1.0f64/rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.fields[0] *= 1.0f64/rhs;
        self.fields[1] *= 1.0f64/rhs;
        self.fields[2] *= 1.0f64/rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields[index]
    }
}