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

fn test_scene() {
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
                metal: MetalMaterial::metal(Vec3::new(0.8, 0.6, 0.2), 0.0)});

    let mut world: HittableList = HittableList::default();
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(-1.0, 0.0, -1.0), -0.45, material_left_2)));
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let lookfrom : Vec3 = Vec3::new(3.0, 3.0, 2.0);
    let lookat : Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let vup : Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (lookfrom - lookat).length();
    let aperture: f64 = 2.0;
    let cam: Camera = Camera::camera(
        lookfrom,
        lookat,
        vup,
        60.0,
        aspect_ratio,
        aperture,
        dist_to_focus);

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

fn random_world() -> HittableList {
    let mut world: HittableList = HittableList::default();

    let ground_material: Box<Material>
        = Box::new(
            Material::Lambertian{
                lambertian: LambertianMaterial::lambertian(Vec3::one() / 2.0)});
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = Utils::random_double();
            let center: Vec3 = Vec3::new(a as f64 + 0.9*Utils::random_double(), 0.2, b as f64 + 0.9 * Utils::random_double());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Vec3 = Utils::random_vec3() * Utils::random_vec3();
                    sphere_material = Box::new(
                        Material::Lambertian{
                            lambertian: LambertianMaterial::lambertian(albedo)});
                    world.add(
                        Box::new(
                            Sphere::sphere(
                                center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Vec3 = Utils::random_vec3_min_max(0.5, 1.0);
                    let fuzz: f64 = Utils::random_double_min_max(0.0, 0.5);
                    sphere_material = Box::new(
                        Material::Metal{
                            metal: MetalMaterial::metal(albedo, fuzz)});
                    world.add(
                        Box::new(
                            Sphere::sphere(
                                center, 0.2, sphere_material)));
                } else {
                    sphere_material = Box::new(
                        Material::Dielectric{
                            dielectric: DielectricMaterial::dielectric(1.5)});
                    world.add(
                        Box::new(
                            Sphere::sphere(
                                center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1: Box<Material>
        = Box::new(
            Material::Dielectric{
                dielectric: DielectricMaterial::dielectric(1.5)});
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2: Box<Material>
        = Box::new(
            Material::Lambertian{
                lambertian: LambertianMaterial::lambertian(
                    Vec3::new(0.4, 0.2, 0.1))});
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3: Box<Material>
        = Box::new(
            Material::Metal{
                metal: MetalMaterial::metal(
                    Vec3::new(0.7, 0.6, 0.5), 0.0)});
    world.add(
        Box::new(
            Sphere::sphere(
                Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // return the random world
    world
}

fn final_scene() {
    // Image
    let debug: bool = false;

    let aspect_ratio : f64 = 3.0 / 2.0;

    // debug/testing configs
    let mut image_witdh : u32 = 200;
    let mut samples_per_pixel : u32 = 5;
    let mut max_depth = 5;

    // final render configs (not debug)
    if !debug {
        image_witdh = 1200;
        samples_per_pixel = 500;
        max_depth = 50;
    }

    let image_heigth : u32 = (image_witdh as f64 / aspect_ratio) as u32;

    // World
    let world = random_world();

    // Camera
    let lookfrom : Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let lookat : Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let vup : Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;
    let cam: Camera = Camera::camera(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus);

    // Render
    println!("P3\n{} {}\n{}", image_witdh, image_heigth, 255);

    for j in (0..image_heigth) {
        if j % 10 == 0 {
            eprintln!("\rScanlines remaining: {}", image_heigth - j);
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

fn main() {
    // test_scene();
    final_scene();
}
