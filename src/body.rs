use crate::ray::Ray;

/// Shared behaviour for 3D things
pub trait Body {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)>;
}
