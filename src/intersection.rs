use derive_builder::Builder;
use ordered_float::*;
use crate::shape::Shape;

/// Intersection of a ray with a 3d body
/// Contains the location (t) and the body
#[derive(Builder, Debug, Clone)]
pub struct Intersection where {
  t: OrderedFloat<f32>,
  object: Shape
}

/// Factory function
pub fn intersection(t: f32, object: Shape) -> Intersection {
  Intersection {
    t: OrderedFloat(t),
    object: object
  }
}

impl Intersection {
  pub fn t(&self) -> &f32 { &self.t }
  pub fn object(&self) -> &Shape { &self.object }
}

impl PartialEq for Intersection  {
  fn eq(&self, other: &Intersection) -> bool {
    self.t == other.t && self.object == other.object
  }
}

/// A list of intersections
#[derive(Default, Builder, Debug, Clone)]
pub struct Intersections {
  count: usize,
  intersections: Vec<Intersection>
}

pub fn intersections(args: Vec<Intersection>) -> Intersections {
  Intersections {
    count: args.len(),
    intersections: args
  }
}

impl Intersections {
  pub fn count(&self) -> &usize { &self.count }
  pub fn intersections(&self) -> &Vec<Intersection> { &self.intersections }
  pub fn hit(&self) -> Option<&Intersection> {
    let positives: Vec<&Intersection> = self.intersections.iter()
      .filter(|i| i.t >= OrderedFloat(0.0f32)).collect();
    let min_t = positives.iter().min_by(|i,j| i.t.cmp(&j.t));
    match min_t {
      Some(min_t) => Some(*min_t),
      None => None
    }
  }
  pub fn hitc(&self) -> Option<Intersection> {
    let positives: Vec<&Intersection> = self.intersections.iter()
      .filter(|i| i.t >= OrderedFloat(0.0f32)).collect();
    let min_t = positives.iter().min_by(|i,j| i.t.cmp(&j.t));
    match min_t {
      Some(min_t) => {
        let i1 = IntersectionBuilder::default()
          .t(min_t.t)
          .object(min_t.object.clone())
          .build()
          .unwrap();
        Some(i1)
      },
      None => None
    }
  }
}

