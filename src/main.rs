use common::{
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    vec3::{Color, Point3},
};
use hittable::{hittable_list::HittableList, sphere::Sphere, Hittable};
use rand::{thread_rng, Rng};
use std::{f64::INFINITY, fs::File, io::Write, rc::Rc};
use view::Camera;

mod common;
mod hittable;
mod view;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::zeros();
    }
    if let Some(rec) = world.hit(ray, 0.001, INFINITY) {
        if let Some((attentuation, scattered)) = rec.material.scatter(ray, &rec) {
            return ray_color(&scattered, world, depth - 1) * attentuation;
        }
        return Color::zeros();
    }
    let unit_dir = ray.direction().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut image = File::create("image.ppm").unwrap();
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    //let material_center = Rc::new(Dielectric::new(1.5));
    //let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let camera = Camera::default();

    write!(image, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    let mut rng = thread_rng();
    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}  ", j);
        for i in 0..image_width {
            let mut pixel = Color::zeros();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                pixel = pixel + ray_color(&r, &world, max_depth);
            }

            pixel.write_color(&mut image, 100).unwrap();
        }
    }
}
