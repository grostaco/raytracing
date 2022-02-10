use crate::common::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{hit_record::HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut record = HitRecord {
            t: root,
            p: ray.at(root),
            normal: Vec3::zeros(),
            front_face: false,
        };
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        Some(record)
    }
}