use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utils::Utils;

pub struct Camera {
    origin : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
    lower_left_corner : Vec3,
    u : Vec3,
    v : Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn camera(
            lookfrom: Vec3,
            lookat: Vec3,
            vup: Vec3,
            vfov: f64,
            aspect_ratio: f64,
            aperture: f64,
            focus_dist: f64) -> Camera {

        let theta = Utils::degree_to_radians(vfov);
        let viewport_height_half: f64 = (theta / 2.0).tan();
        // let viewport_height_half: f64 = 2.0 * h;
        let viewport_width_half: f64 = aspect_ratio * viewport_height_half;

        let temp_w = Utils::unit_vector(&(lookfrom - lookat));
        let temp_u = Utils::unit_vector(&Utils::cross(&vup, &temp_w));
        let temp_v = Utils::cross(&temp_w, &temp_u);

        let hori = focus_dist * viewport_width_half * temp_u;
        let vert = focus_dist * viewport_height_half * temp_v;

        Camera {
            origin : lookfrom,
            horizontal : hori * 2.0,
            vertical : vert * 2.0,
            lower_left_corner :
                lookfrom - hori - vert - focus_dist * temp_w,
            u: temp_u,
            v: temp_v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = self.lens_radius * Utils::random_in_unit_disk();
        let offset : Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray {
            _origin : self.origin + offset,
            _direction :
                self.lower_left_corner
                + s * self.horizontal + t * self.vertical
                - self.origin - offset,
        }
    }
}
