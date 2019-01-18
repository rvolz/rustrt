use ordered_float::*;
use crate::shape::Shape;

/// Intersection of a ray with a 3d body
/// Contains the location (t) and the body
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug)]
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
  pub fn hit<'a>(&'a self) -> Option<&'a Intersection> {
    let positives: Vec<&Intersection> = self.intersections.iter().filter(|i| i.t >= OrderedFloat(0.0f32)).collect();
    if positives.len() == 0 {
      None
    } else {
      let min_t = positives.iter().min_by(|i,j| i.t.cmp(&j.t));
      Some(min_t.unwrap())
    }
  }
  pub fn hitc(&self) -> Option<Intersection> {
    let positives: Vec<&Intersection> = self.intersections.iter().filter(|i| i.t >= OrderedFloat(0.0f32)).collect();
    if positives.len() == 0 {
      None
    } else {
      let min_t = positives.iter().min_by(|i,j| i.t.cmp(&j.t));
      let min_tc = min_t.unwrap().clone();
      Some(*min_tc)
    }
  }
}
