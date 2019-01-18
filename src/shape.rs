use crate::ray::Ray;
use crate::tuple::{Tuple, point, vector};
use crate::body::Body;
use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
  Sphere(Sphere)
}

impl Body for Shape {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)> {
    match self {
      Sphere =>  {
        let s2r = *ray.origin() - point(0.0,0.0,0.0);
        let a = ray.direction().dot(*ray.direction());
        let b = 2.0 * ray.direction().dot(s2r);
        let c = s2r.dot(s2r) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
          None
        } else {
          let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
          let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
          Some((t1,t2))
        }
      }
    }
  }
}
