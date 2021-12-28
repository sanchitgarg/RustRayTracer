// vec 3 class

use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    // Constructor
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            e : [x, y, z]
        }
    }

    // Getters
    pub fn x(self) -> f32 { self.e[0] }
    pub fn y(self) -> f32 { self.e[1] }
    pub fn z(self) -> f32 { self.e[2] }
    pub fn r(self) -> f32 { self.e[0] }
    pub fn g(self) -> f32 { self.e[1] }
    pub fn b(self) -> f32 { self.e[2] }

    // Math functions
    pub fn dot(self, rhs: Self) -> f32 {
        {
            self.e[0] * rhs.e[0] +
            self.e[1] * rhs.e[1] +
            self.e[2] * rhs.e[2]
        }
    }

    pub fn cross(self, rhs: Self) -> Vec3 {
        Vec3 {
            e:[
                self.e[1] * rhs.e[2] + self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] + self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]
            ]
        }
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
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

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
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
        let v1: Vec3 = Vec3::new(1f32, 2f32, 3f32);
        let v2: Vec3 = Vec3::new(2f32, 4f32, 6f32);
        let v3: Vec3 = Vec3::new(3f32, 6f32, 9f32);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_vec3_sub() {
        let v1: Vec3 = Vec3::new(1f32, 2f32, 3f32);
        let v2: Vec3 = Vec3::new(2f32, 4f32, 6f32);
        let v3: Vec3 = Vec3::new(-1f32, -2f32, -3f32);
        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn test_vec3_neg() {
        let v1: Vec3 = Vec3::new(1f32, 2f32, 3f32);
        let v2: Vec3 = Vec3::new(-1f32, -2f32, -3f32);
        assert_eq!(-v1, v2);
    }

    #[test]
    fn test_vec3_mul() {
        let v1: Vec3 = Vec3::new(1f32, 2f32, 3f32);
        let t: f32 = 2.0;
        let v2: Vec3 = Vec3::new(2f32, 4f32, 6f32);
        assert_eq!(v1 * t, v2);
    }

    #[test]
    fn test_vec3_div() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let t: f32 = 2.0;
        let v2: Vec3 = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(v1 / t, v2);
    }

    #[test]
    fn test_vec3_dot() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1.dot(v2), 6.0);
    }

    #[test]
    fn test_vec3_cross() {
        let v1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let v2: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.cross(v2), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vec3_length_square() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length_squared(), 14.0);
    }

    #[test]
    fn test_vec3_length() {
        let v1: Vec3 = Vec3::new(0f32, 4f32, 3f32);
        assert_eq!(v1.length(), 5.0);
    }
}
