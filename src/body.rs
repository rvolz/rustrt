use crate::ray::Ray;
use crate::matrix::Matrix;

/// Shared behaviour for 3D things
pub trait Body {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)>;
  fn set_transform(&mut self, m: Matrix);
  fn transform(&self) -> &Matrix;
}
