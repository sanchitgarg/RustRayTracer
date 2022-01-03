use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::Utils;

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        false
    }
}

// ----------- Lambertian material -----------------
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LambertianMaterial {
    albedo: Vec3,
}

impl LambertianMaterial {
    pub fn lambertian(a: Vec3) -> LambertianMaterial {
        LambertianMaterial { albedo: a }
    }
}

impl Scatter for LambertianMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: & mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + Utils::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::ray(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
// -----------------------------------------


// -------- Metal material -----------------
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MetalMaterial {
    albedo: Vec3,
}

impl MetalMaterial {
    pub fn metal(a: Vec3) -> MetalMaterial {
        MetalMaterial { albedo: a }
    }
}

impl Scatter for MetalMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = Utils::unit_vector(&r_in.direction()).reflect(rec.normal);
        *scattered = Ray::ray(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0_f64
    }
}
// -----------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub enum Material {
    Lambertian { lambertian: LambertianMaterial },
    Metal { metal: MetalMaterial },
    Default,
}

impl Default for Material {
    fn default() -> Self {
        Material::Default
    }
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        // Material should not be default
        assert_ne!(*self, Material::Default);

        match self {
            Material::Lambertian { lambertian } => {
                lambertian.scatter(r_in, rec, attenuation, scattered)
            },
            Material::Metal { metal } => {
                metal.scatter(r_in, rec, attenuation, scattered)
            },
            Material::Default => {
                // Should be unreachable given the assert at the start of the function,
                // added mostly for completeness
                false
            }
        }
    }
}
