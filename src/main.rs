mod vec3;
mod ray;
mod utils;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;

use vec3::Vec3;
use ray::Ray;
use utils::Utils;
use hittable::*;
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;
use material::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    // Object intersection
    let mut rec: HitRecord = HitRecord::default();

    if world.hit(r, 0.001, Utils::infinity(), &mut rec) {
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Vec3 = Vec3::zero();

        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Vec3::zero();
    }

    // Environment
    let unit_direction: Vec3 = Utils::unit_vector(&r.direction());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let debug: bool = false;

    let aspect_ratio : f64 = 16.0 / 9.0;

    // debug/testing configs
    let mut image_witdh : u32 = 200;
    let mut samples_per_pixel : u32 = 5;
    let mut max_depth = 5;

    // final render configs (not debug)
    if !debug {
        image_witdh = 400;
        samples_per_pixel = 100;
        max_depth = 10;
    }

    let image_heigth : u32 = (image_witdh as f64 / aspect_ratio) as u32;

    // World
    let material_ground: Box<Material>
        = Box::new(
            Material::Lambertian{
                lambertian: LambertianMaterial::lambertian(Vec3::new(0.8, 0.8, 0.0))});
    let material_center: Box<Material>
        = Box::new(
            Material::Lambertian{
                lambertian: LambertianMaterial::lambertian(Vec3::new(0.1, 0.2, 0.5))});
    let material_left: Box<Material>
        = Box::new(
            Material::Dielectric{
                dielectric: DielectricMaterial::dielectric(1.5)});
    let material_left_2: Box<Material>
        = Box::new(
            Material::Dielectric{
                dielectric: DielectricMaterial::dielectric(1.5)});
    let material_right: Box<Material>
        = Box::new(
            Material::Metal{
                metal: MetalMaterial::metal(Vec3::new(0.8, 0.6, 0.2), 1.0)});

    let mut world: HittableList = HittableList::default();
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    // world.add(
    //     Box::new(
    //         Sphere::sphere(
    //             Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left_2)));
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let cam: Camera = Camera::camera();

    // Render
    println!("P3\n{} {}\n{}", image_witdh, image_heigth, 255);

    for j in (0..image_heigth).rev() {
        if j % 10 == 0 {
            eprintln!("\rScanlines remaining: {}", j);
        }

        for i in 0..image_witdh {
            let mut pixel_color = Vec3::new(0.0,0.0,0.0);

            for s in 0..samples_per_pixel {
                let u: f64 =
                    (i as f64 + Utils::random_double()) / image_witdh as f64;
                let v: f64 =
                    (j as f64 + Utils::random_double()) / image_heigth as f64;

                let r : Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            Utils::write_color(&pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.\n");
}
