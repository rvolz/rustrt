use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::body::Body;
use crate::tuple::Tuple;
use crate::sphere::{Sphere,sphere};
use crate::intersection::{Intersections,intersection,intersections};
use crate::material::Material;

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

  pub fn material(&self) -> &Material {
    match self {
      Shape::Sphere(sphere) =>  sphere.material()
    }
  }

  pub fn normal_at(&self, point: Tuple) -> Tuple {
    match self {
      Shape::Sphere(sphere) =>  sphere.normal_at(point)
    }
  }

  pub fn set_material(&mut self, m: Material) {
    match self {
      Shape::Sphere(sphere) =>  sphere.set_material(m)
    }
  }

  pub fn set_transform(&mut self, m: Matrix) {
    match self {
      Shape::Sphere(sphere) =>  sphere.set_transform(m)
    }
  }
}

impl Default for Shape {
    fn default() -> Self { Shape::Sphere(sphere()) }
}
