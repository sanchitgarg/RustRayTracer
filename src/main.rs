mod vec3;
mod ray;
mod utils;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;

use vec3::Vec3;
use ray::Ray;
use utils::Utils;
use hittable::*;
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, 0.001, Utils::infinity(), &mut rec) {
        let target: Vec3 = rec.p + rec.normal + Utils::random_unit_vector();
        return 0.5 * ray_color(&Ray::ray(rec.p, target - rec.p), world, depth - 1);
    }

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
    let mut world: HittableList = HittableList::default();
    world.add(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
