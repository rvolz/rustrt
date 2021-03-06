use derive_builder::Builder;
use crate::body::*;
use crate::matrix::{Matrix,identity};
use crate::ray::Ray;
use crate::tuple::{Tuple, point, vector};
use crate::material::{Material,material};

/// A 3D sphere
#[derive(Default, Builder, Debug, Clone)]
pub struct Sphere {
  origin: Tuple,
  direction: Tuple,
  transform: Matrix,
  material: Material
}

/// Factory function
pub fn sphere() -> Sphere {
  Sphere {
    origin: point(0.0,0.0,0.0),
    direction: vector(0.0,0.0,0.0),
    transform: identity(),
    material: material()
  }
}

impl Body for Sphere {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)> {
    if self.transform.is_invertible() {
      let nray = ray.set_transform(&self.transform.inverse());
      let s2r = *nray.origin() - point(0.0,0.0,0.0);
      let a = nray.direction().dot(*nray.direction());
      let b = 2.0 * nray.direction().dot(s2r);
      let c = s2r.dot(s2r) - 1.0;
      let discriminant = b.powi(2) - 4.0 * a * c;
      if discriminant < 0.0 {
        None
      } else {
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some((t1,t2))
      }
    } else {
      None
    }
  }
  fn material(&self) -> &Material { &self.material }
  fn normal_at(&self, world_point: Tuple) -> Tuple {
    let object_point = &self.transform.inverse() * &world_point;
    let object_normal = object_point - point(0.0,0.0,0.0);
    let mut world_normal = &self.transform.inverse().transpose() * &object_normal;
    world_normal.set_w(0.0);
    world_normal.normalize()
  }
  fn set_material(&mut self, m: Material) {
    self.material = m;
  }
  fn set_transform(&mut self, m: Matrix) {
    self.transform = m;
  }
  fn transform(&self) -> &Matrix {
    &self.transform
  }
}

impl PartialEq for Sphere {
  fn eq(&self, other: &Sphere) -> bool {
    self.origin == other.origin && self.direction == other.direction
  }
}
