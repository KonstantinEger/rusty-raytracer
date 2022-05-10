use crate::vec3::Vec3;
use crate::ray::Ray;

pub enum Hit {
    Color(Vec3),
}

pub trait Hittable {
    /// Returns a [Hit], if Self has been hit by the ray.
    fn hit_by(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit_by(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            Some(Hit::Color(Vec3::from((1.0, 0.0, 0.0))))
        } else {
            None
        }
    }
}
