use crate::vec3::Vec3;

pub struct Utils {

}

impl Utils {
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        v1.cross(v2)
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.dot(v2)
    }

    pub fn write_color(pixel_color: Vec3) {
        // Write the translated [0,255] value of each color component.
        let ir: u32 = (255.999 * pixel_color.r()) as u32;
        let ig: u32 = (255.999 * pixel_color.g()) as u32;
        let ib: u32 = (255.999 * pixel_color.b()) as u32;

        println!("{} {} {}", ir, ig, ib);
    }

    pub fn infinity() -> f32 {
        std::f32::MAX
    }

    pub fn pi() -> f32 {
        3.1415926535897932385
    }

    pub fn degree_to_radians(degree: f32) -> f32 {
        degree * Utils::pi() / 180.0
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_utils_unit_direction() {
        let v : Vec3 = Vec3::new(4.0, 0.0, 0.0);
        assert_eq!(Utils::unit_vector(v), Vec3::new(1.0,0.0, 0.0))
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
