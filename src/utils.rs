use crate::vec3::Vec3;
use rand::*;

pub struct Utils {

}

impl Utils {
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        v1.cross(v2)
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.dot(&v2)
    }

    pub fn write_color(pixel_color: &Vec3, samples_per_pixel: u32) {
        // Write the translated [0,255] value of each color component.
        let mut r: f64 = pixel_color.r();
        let mut g: f64 = pixel_color.g();
        let mut b: f64 = pixel_color.b();

        let scale: f64 = 1f64 / samples_per_pixel as f64;

        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ir: u32 = (255.999 * Utils::clamp(r, 0.0, 0.999)) as u32;
        let ig: u32 = (255.999 * Utils::clamp(g, 0.0, 0.999)) as u32;
        let ib: u32 = (255.999 * Utils::clamp(b, 0.0, 0.999)) as u32;

        println!("{} {} {}", ir, ig, ib);
    }

    pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            return min;
        } else if x > max {
            return max;
        }

        x
    }

    pub fn infinity() -> f64 {
        std::f64::MAX
    }

    pub fn pi() -> f64 {
        3.1415926535897932385
    }

    pub fn degree_to_radians(degree: f64) -> f64 {
        degree * Utils::pi() / 180.0
    }

    pub fn random_double() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

    pub fn random_double_min_max(min: f64, max: f64) -> f64 {
        min + (max - min) * Utils::random_double()
    }

    pub fn random_vec3() -> Vec3 {
        Vec3::new(
            Utils::random_double(),
            Utils::random_double(),
            Utils::random_double())
    }

    pub fn random_vec3_min_max(min: f64, max:f64) -> Vec3 {
        Vec3::new(
            Utils::random_double_min_max(min, max),
            Utils::random_double_min_max(min, max),
            Utils::random_double_min_max(min, max))
    }

    pub fn random_in_unit_shpere() -> Vec3 {
        loop {
            let p: Vec3 = Utils::random_vec3_min_max(-1f64, 1f64);
            if p.length_squared() < 1f64 {
                return p;
            }
        }

        // unreachable code
        Utils::random_vec3()
    }

    pub fn random_unit_vector() -> Vec3 {
        Utils::unit_vector(&Utils::random_in_unit_shpere())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_utils_unit_direction() {
        let v : Vec3 = Vec3::new(4.0, 0.0, 0.0);
        assert_eq!(Utils::unit_vector(&v), Vec3::new(1.0,0.0, 0.0))
    }

    #[test]
    fn test_utils_dot() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(Utils::dot(&v1, &v2), 6.0);
    }

    #[test]
    fn test_uitls_cross() {
        let v1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let v2: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Utils::cross(&v1, &v2), Vec3::new(0.0, 0.0, 1.0));
    }
}
