use crate::ray::Ray;
use ordered_float::*;

/// Shared behaviour for 3D things
pub trait Body {
  fn intersect(&self, ray: Ray) -> Option<(f32,f32)>;
}

/// Intersection of a ray with a 3d body
/// Contains the location (t) and the body
#[derive(Debug, Clone, Copy, Eq)]
pub struct Intersection<T> where T: Body {
  t: OrderedFloat<f32>,
  object: T
}

/// Factory function
pub fn intersection<T: Body>(t: f32, object: T) -> Intersection<T> {
  Intersection {
    t: OrderedFloat(t),
    object: object
  }
}

impl<T> Intersection<T> where T: Body {
  pub fn t(&self) -> &f32 { &self.t }
  pub fn object(&self) -> &T { &self.object }
}

impl<T> PartialEq for Intersection<T> where T: Body+PartialEq {
  fn eq(&self, other: &Intersection<T>) -> bool {
    self.t == other.t && self.object == other.object
  }
}

/// A list of intersections 
#[derive(Debug)]
pub struct Intersections<T> where T: Body {
  count: usize,
  intersections: Vec<Intersection<T>>
}

pub fn intersections<T: Body>(args: Vec<Intersection<T>>) -> Intersections<T> {
  Intersections {
    count: args.len(),
    intersections: args
  }
}

impl<T> Intersections<T> where T: Body {
  pub fn count(&self) -> &usize { &self.count }
  pub fn intersections(&self) -> &Vec<Intersection<T>> { &self.intersections }
  pub fn hit(&self) -> Option<&Intersection<T>> {
    let positives: Vec<&Intersection<T>> = self.intersections.iter().filter(|i| i.t >= OrderedFloat(0.0f32)).collect();
    if positives.len() == 0 {
      None
    } else {
      let min_t = positives.iter().min_by(|i,j| i.t.cmp(&j.t));
      Some(min_t.unwrap())
    }
  }
}
