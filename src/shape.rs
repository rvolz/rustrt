use crate::ray::Ray;
use crate::body::Body;
use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
  Sphere(Sphere)
}

impl Body for Shape {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)> {
    match *self {
      Shape::Sphere(sphere) =>  {
        sphere.intersect(ray)
      }
    }
  }
}
