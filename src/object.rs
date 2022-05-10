use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    /// Returns a [Hit], if Self has been hit by the ray.
    fn hit_by(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit_by(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().len_sqr();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_sqr() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant >= 0.0 {
            let disc_sqrt = discriminant.sqrt();
            let mut root = (-half_b - disc_sqrt) / a;
            if root < t_min || root >= t_max {
                root = (-half_b + disc_sqrt) / a;
                if root < t_min || root >= t_max {
                    return None;
                }
            }
            let t = root;
            let point = ray.at(t);
            let normal = (point - self.center) / self.radius;
            Some(Hit { t, point, normal })
        } else {
            None
        }
    }
}
