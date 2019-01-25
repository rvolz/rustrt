use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

/// Shared behaviour for 3D things
pub trait Body {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)>;
  fn normal_at(&self, world_point: Tuple) -> Tuple;
  fn set_transform(&mut self, m: Matrix);
  fn transform(&self) -> &Matrix;
}
