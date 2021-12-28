mod vec3;
mod ray;
mod utils;

use vec3::Vec3;
use ray::Ray;
use utils::Utils;

fn color(r: &Ray) -> Vec3 {
    let unit_direction: Vec3 = Utils::unit_direction(r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main()
{
    let debug: bool = false;

    let mut nx : u32 = 800;
    let mut ny : u32 = 400;

    if debug {
        nx = 200;
        ny = 100;
    }

    let max_value : u32 = 255;

    println!("P3\n{} {}\n{}", nx, ny, max_value);

    let lower_left_corner : Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal : Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical : Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin : Vec3 = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        if j % 10 == 0 {
            eprintln!("\rScanlines remaining: {}", j);
        }

        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;

            let r : Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vec3 = color(&r);

            let ir: u32 = (255.999 * col.r()) as u32;
            let ig: u32 = (255.999 * col.g()) as u32;
            let ib: u32 = (255.999 * col.b()) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.\n");
}
