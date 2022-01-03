use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
    lower_left_corner : Vec3,
}

impl Camera {
    pub fn camera() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let o = Vec3::zero();
        let h = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            origin : o,
            horizontal : h,
            vertical : v,
            lower_left_corner :
                o - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            _origin : self.origin,
            _direction :
                self.lower_left_corner
                + u * self.horizontal
                + v * self.vertical
                - self.origin,
        }
    }
}
