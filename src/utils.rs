use crate::vec3::Vec3;

pub struct Utils {

}

impl Utils {
    pub fn unit_direction(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        v1.cross(v2)
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.dot(v2)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_utils_unit_direction() {
        let v : Vec3 = Vec3::new(4.0, 0.0, 0.0);
        assert_eq!(Utils::unit_direction(v), Vec3::new(1.0,0.0, 0.0))
    }

    #[test]
    fn test_utils_dot() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(Utils::dot(v1, v2), 6.0);
    }

    #[test]
    fn test_uitls_cross() {
        let v1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let v2: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Utils::cross(v1, v2), Vec3::new(0.0, 0.0, 1.0));
    }
}
