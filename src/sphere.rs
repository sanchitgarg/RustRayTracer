use crate::vec3::Vec3;
use crate::hittable::*;
use crate::ray::Ray;
use crate::utils::Utils;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn sphere(cen: Vec3, r: f32) -> Self {
        Sphere{ center: cen, radius: r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = Utils::dot(oc, r.direction());
        let c: f32 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f32 = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f32 = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root: f32 = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.point_at_parameter(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}
