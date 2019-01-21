use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::body::Body;
use crate::sphere::{Sphere,sphere};
use crate::intersection::{Intersections,intersection,intersections};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
  Sphere(Sphere)
}

impl Shape {
  pub fn intersect(&self, ray: Ray) -> Intersections {
    match self {
      Shape::Sphere(sphere) =>  {
        let oi = sphere.intersect(ray);
        match oi {
          Some((t1,t2)) => {
            let i1 = intersection(t1, self.clone());
            let i2 = intersection(t2, self.clone());
            intersections(vec![i1,i2])
        }
          None => intersections(vec![])
        }
      }
    }
  }

  pub fn transform(&mut self, m: Matrix) {
    match self {
      Shape::Sphere(sphere) =>  sphere.transform(m)
    }
  }
}

impl Default for Shape {
    fn default() -> Self { Shape::Sphere(sphere()) }
}
