// vec 3 class

use std::ops;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e : [x, y, z] }
    }
    pub fn zero() -> Vec3 {
        Vec3 { e : [0.0, 0.0, 0.0] }
    }

    // Getters
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn r(&self) -> f64 { self.e[0] }
    pub fn g(&self) -> f64 { self.e[1] }
    pub fn b(&self) -> f64 { self.e[2] }

    // Math functions
    pub fn dot(&self, rhs: &Self) -> f64 {
        {
            self.e[0] * rhs.e[0] +
            self.e[1] * rhs.e[1] +
            self.e[2] * rhs.e[2]
        }
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Vec3 {
            e:[
                self.e[1] * rhs.e[2] + self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] + self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]
            ]
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 0.00000001_f64;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0_f64 * self.dot(&n) * n
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            e: [self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2]]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
                    e: [self.e[0] + _rhs.e[0],
                        self.e[1] + _rhs.e[1],
                        self.e[2] + _rhs.e[2]]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            e: [self.e[0] - _rhs.e[0],
                self.e[1] - _rhs.e[1],
                self.e[2] - _rhs.e[2]]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, _rhs: Self) {
        *self = Self {
                    e: [self.e[0] - _rhs.e[0],
                        self.e[1] - _rhs.e[1],
                        self.e[2] - _rhs.e[2]]
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            e: [-self.e[0],
                -self.e[1],
                -self.e[2]]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        *self = Self {
                    e: [self.e[0] * _rhs,
                        self.e[1] * _rhs,
                        self.e[2] * _rhs]
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        let v1: Vec3 = Vec3::new(1f64, 2f64, 3f64);
        let v2: Vec3 = Vec3::new(2f64, 4f64, 6f64);
        let v3: Vec3 = Vec3::new(3f64, 6f64, 9f64);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_vec3_sub() {
        let v1: Vec3 = Vec3::new(1f64, 2f64, 3f64);
        let v2: Vec3 = Vec3::new(2f64, 4f64, 6f64);
        let v3: Vec3 = Vec3::new(-1f64, -2f64, -3f64);
        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn test_vec3_neg() {
        let v1: Vec3 = Vec3::new(1f64, 2f64, 3f64);
        let v2: Vec3 = Vec3::new(-1f64, -2f64, -3f64);
        assert_eq!(-v1, v2);
    }

    #[test]
    fn test_vec3_mul() {
        let v1: Vec3 = Vec3::new(1f64, 2f64, 3f64);
        let t: f64 = 2.0;
        let v2: Vec3 = Vec3::new(2f64, 4f64, 6f64);
        assert_eq!(v1 * t, v2);
        assert_eq!(t * v1, v2);

        let v3: Vec3 = Vec3::new(2f64, 8f64, 18f64);
        assert_eq!(v1 * v2, v3);
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut v1: Vec3 = Vec3::new(1f64, 2f64, 3f64);
        let t: f64 = -2.0;
        let v2: Vec3 = Vec3::new(-2f64, -4f64, -6f64);
        v1 *= t;
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_vec3_div() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let t: f64 = 2.0;
        let v2: Vec3 = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(v1 / t, v2);
    }

    #[test]
    fn test_vec3_dot() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1.dot(&v2), 6.0);
    }

    #[test]
    fn test_vec3_cross() {
        let v1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let v2: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.cross(&v2), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vec3_length_square() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length_squared(), 14.0);
    }

    #[test]
    fn test_vec3_length() {
        let v1: Vec3 = Vec3::new(0f64, 4f64, 3f64);
        assert_eq!(v1.length(), 5.0);
    }
}
