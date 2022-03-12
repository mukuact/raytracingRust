use super::vec::{Point3, Vec3};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};
use super::material::Scatter;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Scatter>
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<dyn Scatter>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: m
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            material: self.material.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_narmal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_narmal);
        Some(rec)
        
    }
}
