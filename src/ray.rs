use crate::tuple::*;
use crate::matrix::Matrix;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  origin: Tuple,
  direction: Tuple
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
  Ray { origin: origin, direction: direction }
}

impl Ray {
  pub fn origin(&self) -> &Tuple { &self.origin }
  pub fn direction(&self) -> &Tuple { &self.direction }
  pub fn position(&self, t: f32) -> Tuple {
    self.origin + self.direction * t
  }
  /// set_transform a ray with a transformation matrix
  pub fn set_transform(&self, m: &Matrix) -> Ray {
    Ray {
      origin: m * &self.origin,
      direction: m * &self.direction
    }
  }
}
