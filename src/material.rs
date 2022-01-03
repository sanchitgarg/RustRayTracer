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
    fuzz: f64,
}

impl MetalMaterial {
    pub fn metal(a: Vec3, f: f64) -> MetalMaterial {
        MetalMaterial { albedo: a, fuzz: f }
    }
}

impl Scatter for MetalMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = Utils::unit_vector(&r_in.direction()).reflect(rec.normal);
        *scattered = Ray::ray(rec.p, reflected + self.fuzz * Utils::random_in_unit_shpere());
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0_f64
    }
}
// -----------------------------------------

// -------- Metal material -----------------
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DielectricMaterial {
    ir: f64,
}

impl DielectricMaterial {
    pub fn dielectric(index_of_refraction: f64) -> DielectricMaterial {
        DielectricMaterial { ir: index_of_refraction }
    }
}

impl DielectricMaterial {
    fn refractance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0: f64 = (1.0_f64 - ref_idx) / (1.0_f64 + ref_idx);
        r0 = r0*r0;
        r0 + (1.0_f64 - r0) * (1.0_f64 - cosine).powi(5)
    }
}
impl Scatter for DielectricMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::one();

        let mut refraction_ratio: f64 = self.ir;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        }

        let unit_direction: Vec3 = Utils::unit_vector(&r_in.direction());
        let cos_theta: f64 = Utils::dot(&-unit_direction, &rec.normal).min(1.0_f64);
        let sin_theta: f64 = (1.0_f64 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0_f64;
        let mut direction: Vec3 = Vec3::zero();

        if cannot_refract || DielectricMaterial::refractance(cos_theta, refraction_ratio) > Utils::random_double() {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, refraction_ratio);
        }

        *scattered = Ray::ray(rec.p, direction);

        true
    }
}
// -----------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub enum Material {
    Lambertian { lambertian: LambertianMaterial },
    Metal { metal: MetalMaterial },
    Dielectric { dielectric: DielectricMaterial },
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
            Material::Dielectric { dielectric } => {
                dielectric.scatter(r_in, rec, attenuation, scattered)
            },
            Material::Default => {
                // Should be unreachable given the assert at the start of the function,
                // added mostly for completeness
                false
            }
        }
    }
}
