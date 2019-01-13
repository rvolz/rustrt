use crate::body::*;
//use crate::matrix::{dot};
use crate::ray::Ray;
use crate::tuple::{Tuple, point, vector};

/// A 3D sphere
#[derive(Debug, Clone, Copy)]
pub struct Sphere {
  origin: Tuple,
  direction: Tuple
}

/// Factory function
pub fn sphere() -> Sphere {
  Sphere {
    origin: point(0.0,0.0,0.0),
    direction: vector(0.0,0.0,0.0),
  }
}

impl Body for Sphere {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)> {
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

impl PartialEq for Sphere {
  fn eq(&self, other: &Sphere) -> bool {
    self.origin == other.origin && self.direction == other.direction
  }
}
