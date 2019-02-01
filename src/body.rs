use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::tuple::Tuple;
use crate::material::Material;

/// Shared behaviour for 3D things
pub trait Body {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)>;
  fn material(&self) -> &Material;
  fn normal_at(&self, world_point: Tuple) -> Tuple;
  fn set_material(&mut self, m: Material);
  fn set_transform(&mut self, m: Matrix);
  fn transform(&self) -> &Matrix;
}
