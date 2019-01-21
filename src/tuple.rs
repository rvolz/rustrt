use std::ops;
use std::f32;
use float_cmp::{ApproxEq};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
  x: f32,
  y: f32,
  z: f32,
  w: f32
}

pub fn color(x:f32, y:f32, z:f32) -> Tuple {
  Tuple {x,y,z,w:0.0}
}

pub fn point(x:f32, y:f32, z:f32) -> Tuple {
  Tuple {x,y,z,w:1.0}
}

pub fn tuple(x:f32, y:f32, z:f32, w:f32) -> Tuple {
  Tuple {x,y,z,w}
}

pub fn vector(x:f32, y:f32, z:f32) -> Tuple {
  Tuple {x,y,z,w:0.0}
}

/// Determines if a tuple is a point
pub fn is_point(t: &Tuple) -> bool {
  (t.w == 1.0)
}

/// Determines if a tuple is a vector
fn is_vector(t: &Tuple) -> bool {
  (t.w == 0.0)
}

impl Tuple {
  pub fn get_x(&self) -> &f32 { &self.x }
  pub fn get_y(&self) -> &f32 { &self.y }
  pub fn get_z(&self) -> &f32 { &self.z }
  pub fn get_w(&self) -> &f32 { &self.w }
  pub fn red(&self) -> &f32 { &self.x }
  pub fn green(&self) -> &f32 { &self.y }
  pub fn blue(&self) -> &f32 { &self.z }

  /// Computes the cross product of two vectors. Panics with points
  pub fn cross(&self, rhs: Tuple) -> Tuple {
    if is_point(&self) || is_point(&rhs) {
      panic!("Dot product is only defined for vectors! left `{:?}`, right `{:?}`", &self, &rhs);
    }
    let x = &self.y * &rhs.z - &self.z * &rhs.y;
    let y = &self.z * &rhs.x - &self.x * &rhs.z;
    let z = &self.x * &rhs.y - &self.y * &rhs.x;
    vector(x, y, z)
  }

  /// Computes the dot product of two vectors. Panics with points
  pub fn dot(&self, rhs: Tuple) -> f32 {
    if is_point(&self) || is_point(&rhs) {
      panic!("Dot product is only defined for vectors! left `{:?}`, right `{:?}`", &self, &rhs);
    }
    &self.x * &rhs.x + &self.y * &rhs.y + &self.z * &rhs.z + &self.w * &rhs.w
  }

  /// Determines if a tuple is a point
  pub fn is_point(&self) -> bool {
    (&self.w == &1.0)
  }
  /// Determines if a tuple is a vector
  pub fn is_vector(&self) -> bool {
    (&self.w == &0.0)
  }
  /// Determine the magnitude of a vector. Panics if operationg on a point.
  pub fn magnitude(&self) -> f32 {
    if !is_vector(&self) {
      panic!("Trying to get the magnitude of a non-vector!");
    }
    let x = self.x.powf(2f32) + self.y.powf(2f32) + self.z.powf(2f32);
    x.sqrt()
  }

  pub fn normalize(&self) -> Tuple {
    if !is_vector(&self) {
      panic!("Trying to normalize a non-vector!");
    }
    let m = self.magnitude();
    Tuple {
      x: self.x/m,
      y: self.y/m,
      z: self.z/m,
      w: self.w/m
    }
  }
}

impl ops::Add for Tuple {
  type Output = Tuple;

  /// Adds two tuples and returns the resulting tuple.
  /// Adding tuples and vectors, or two vectors is allowed.
  /// Adding two points makes no sense and causes an error.
  fn add(self, _rhs: Tuple) -> Tuple {
    if is_point(&self) && is_point(&_rhs) {
      panic!("Adding two points is not allowed!");
    }
    Tuple {
      x: self.x + _rhs.x,
      y: self.y + _rhs.y,
      z: self.z + _rhs.z,
      w: self.w + _rhs.w,
    }
  }
}

impl ops::Div<f32> for Tuple {
  type Output = Tuple;

  /// Divides a tuple by a scalar value only
  fn div(self, _rhs: f32) -> Tuple {
    Tuple {
      x: self.x / _rhs,
      y: self.y / _rhs,
      z: self.z / _rhs,
      w: self.w / _rhs,
    }
  }
}

impl ops::Mul<f32> for Tuple {
  type Output = Tuple;

  /// Multplies a tuple with a scalar value only
  fn mul(self, _rhs: f32) -> Tuple {
    Tuple {
      x: self.x * _rhs,
      y: self.y * _rhs,
      z: self.z * _rhs,
      w: self.w * _rhs,
    }
  }
}

impl ops::Mul for Tuple {
  type Output = Tuple;

  fn mul(self, _rhs: Tuple) -> Tuple {
    if is_point(&self) || is_point(&_rhs) {
      panic!("Multiplying points not defined: {:?} * {:?}",self,_rhs);
    }
    Tuple {
      x: self.x * _rhs.x,
      y: self.y * _rhs.y,
      z: self.z * _rhs.z,
      w: self.w * _rhs.w,
    }
  }
}

impl ops::Neg for Tuple {
  type Output = Tuple;

  // Negation of the whole tuple, including the point/vector identifier (w)
  fn neg(self) -> Tuple {
    Tuple {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    }
  }
}

impl ops::Sub for Tuple {
  type Output = Tuple;

  /// Subtracts two tuples and returns the resulting tuple.
  /// Subtracting a point from a vector makes no sense and causes an error.
  fn sub(self, _rhs: Tuple) -> Tuple {
    if is_vector(&self) && is_point(&_rhs) {
      panic!("Subtracting a point from a vector is not allowed!");
    }
    Tuple {
      x: self.x - _rhs.x,
      y: self.y - _rhs.y,
      z: self.z - _rhs.z,
      w: self.w - _rhs.w,
    }
  }
}

impl PartialEq for Tuple {
  fn eq(&self, other: &Tuple) -> bool {
    self.approx_eq(other, 2.0 * ::std::f32::EPSILON, 10)
  }
}

impl ApproxEq for Tuple {
  type Flt = f32;

  fn approx_eq(&self, other: &Tuple, epsilon: f32, ulps: i32) -> bool {
    self.x.approx_eq(&other.x, epsilon, ulps) &&
    self.y.approx_eq(&other.y, epsilon, ulps) &&
    self.z.approx_eq(&other.z, epsilon, ulps) &&
    self.w.approx_eq(&other.w, epsilon, ulps)
  }
}

impl Default for Tuple {
    fn default() -> Self { tuple(0.0,0.0,0.0,0.0) }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn tuple_as_a_point() {
      assert_eq!(Tuple {x:4.3, y:-4.2, z:3.1, w:1.0}, tuple(4.3, -4.2, 3.1, 1.0));
      assert!(is_point(&tuple(4.3, -4.2, 3.1, 1.0)), "should be a point");
      assert!(!is_vector(&tuple(4.3, -4.2, 3.1, 1.0)), "should not be a vector");
  }

  #[test]
  fn tuple_as_a_vector() {
      assert_eq!(Tuple {x:4.3, y:-4.2, z:3.1, w:0.0}, tuple(4.3, -4.2, 3.1, 0.0));
      assert!(!is_point(&tuple(4.3, -4.2, 3.1, 0.0)), "should not be a point");
      assert!(is_vector(&tuple(4.3, -4.2, 3.1, 0.0)), "should be a vector");
  }

  #[test]
  fn create_point() {
    let p = point(4.0,4.0,3.0);
    assert_eq!(p,tuple(4.0,4.0,3.0,1.0));
  }

  #[test]
  fn create_vector() {
    let v = vector(4.0,4.0,3.0);
    assert_eq!(v,tuple(4.0,4.0,3.0,0.0));
  }

  #[test]
  fn add_tuples() {
    let p = point(3.0,-2.0,5.0);
    let v = vector(-2.0,3.0,1.0);
    assert_eq!(tuple(1.0,1.0,6.0,1.0), p+v);
  }

  #[test]
  #[should_panic(expected = "Adding two points is not allowed!")]
  #[allow(unused_must_use)]
  fn adding_points_causes_error() {
    let p = point(3.0,-2.0,5.0);
    let v = point(-2.0,3.0,1.0);
    p+v;
  }

  /// Subtracting two points yields the vector pointing from p2 to p1
  #[test]
  fn subtract_points_to_get_a_vector() {
    let p1 = point(3.0,2.0,1.0);
    let p2 = point(5.0,6.0,7.0);
    assert_eq!(vector(-2.0,-4.0,-6.0), p1-p2);
  }

  /// Subtracting a vector from a point yields the point moved backwards
  /// by the vector
  #[test]
  fn subtract_vector_from_point() {
    let p = point(3.0,2.0,1.0);
    let v = vector(5.0,6.0,7.0);
    assert_eq!(point(-2.0,-4.0,-6.0), p-v);
  }

  /// Subtracting two vectors yields another vector, representing the change in direction
  #[test]
  fn subtract_vectors() {
    let v1 = vector(3.0,2.0,1.0);
    let v2 = vector(5.0,6.0,7.0);
    assert_eq!(vector(-2.0,-4.0,-6.0), v1-v2);
  }

  #[test]
  #[should_panic(expected = "Subtracting a point from a vector is not allowed!")]
  #[allow(unused_must_use)]
  fn subtracting_point_from_vector_causes_error() {
    let p = point(3.0,-2.0,5.0);
    let v = vector(-2.0,3.0,1.0);
    v-p;
  }
}
