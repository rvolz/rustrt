#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple, tuple};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  a: Tuple,
  a1: Tuple,
  a2: Tuple,
  b: Tuple,
  p: Tuple,
  p1: Tuple,
  p2: Tuple,
  v: Tuple,
  v1: Tuple,
  v2: Tuple,
  zero: Tuple,
  norm: Tuple
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    // This function is called every time a new scenario is started
    MyWorld { 
        a: tuple(0.0,0.0,0.0,0.0),
        a1: tuple(0.0,0.0,0.0,0.0),
        a2: tuple(0.0,0.0,0.0,0.0),
        b: tuple(0.0,0.0,0.0,0.0),
        p: tuple(0.0,0.0,0.0,0.0),
        p1: tuple(0.0,0.0,0.0,0.0),
        p2: tuple(0.0,0.0,0.0,0.0),
        v: tuple(0.0,0.0,0.0,0.0),
        v1: tuple(0.0,0.0,0.0,0.0),
        v2: tuple(0.0,0.0,0.0,0.0),
        zero: tuple(0.0,0.0,0.0,0.0),
        norm: tuple(0.0,0.0,0.0,0.0),
    }
  }
}

mod tuple_steps {
  use super::*;
  use rustrt::tuple::{tuple,point,vector};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
      given regex "a ← tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64,f64) |world, n1, n2, n3, n4, _step| {
        world.a = tuple(n1, n2, n3, n4);
      };

      given regex "a1 ← tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64,f64) |world, n1, n2, n3, n4, _step| {
        world.a1 = tuple(n1, n2, n3, n4);
      };
      given regex "a2 ← tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64,f64) |world, n1, n2, n3, n4, _step| {
        world.a2 = tuple(n1, n2, n3, n4);
      };

      given regex "p ← point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.p = point(n1, n2, n3);
      };

      given regex "p1 ← point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.p1 = point(n1, n2, n3);
      };
      given regex "p2 ← point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.p2 = point(n1, n2, n3);
      };

      given regex "a ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.a = vector(n1, n2, n3);
      };

      given regex "b ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.b = vector(n1, n2, n3);
      };

      given regex "v ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.v = vector(n1, n2, n3);
      };

      given regex "v1 ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.v1 = vector(n1, n2, n3);
      };
      given regex "v2 ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.v2 = vector(n1, n2, n3);
      };

      given regex "zero ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        world.v = vector(n1, n2, n3);
      };

      when "norm ← normalize(v)" |world, _step| {
        world.norm = world.v.normalize();
      };

      then regex "a.x = ([-+]?[0-9]*\\.?[0-9]+)" (f64) |world, number, _step| {
          assert_eq!(world.a.get_x(), &number);
      };

      then regex "a.y = ([-+]?[0-9]*\\.?[0-9]+)" (f64) |world, number, _step| {
          assert_eq!(world.a.get_y(), &number);
      };

      then regex "a.z = ([-+]?[0-9]*\\.?[0-9]+)" (f64) |world, number, _step| {
          assert_eq!(world.a.get_z(), &number);
      };

      then regex "a.w = ([-+]?[0-9]*\\.?[0-9]+)" (f64) |world, number, _step| {
          assert_eq!(world.a.get_w(), &number);
      };

      then "a is a point" |world, _step| {
        assert!(world.a.is_point());
      };

      then "a is not a point" |world, _step| {
        assert!(!world.a.is_point());
      };

      then "a is a vector" |world, _step| {
        assert!(world.a.is_vector());
      };

      then "a is not a vector" |world, _step| {
        assert!(!world.a.is_vector());
      };

      then regex "-a = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64,f64) |world, n1, n2, n3, n4, _step| {
        assert_eq!(-world.a, tuple(n1, n2, n3, n4));
      };

      then regex "p = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64,f64) |world, n1, n2, n3, n4, _step| {
        assert_eq!(world.p, tuple(n1, n2, n3, n4));
      };

      then regex "v = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64,f64) |world, n1, n2, n3, n4, _step| {
        assert_eq!(world.v, tuple(n1, n2, n3, n4));
      };

      then regex "a1 \\+ a2 = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64,f64) |world, n1, n2, n3, n4, _step| {
        assert_eq!(world.a1 + world.a2, tuple(n1, n2, n3, n4));
      };

      then regex "p1 \\- p2 = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        assert_eq!(world.p1 - world.p2, vector(n1, n2, n3));
      };

      then regex "p \\- v = point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        assert_eq!(world.p - world.v, point(n1, n2, n3));
      };

      then regex "v1 \\- v2 = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        assert_eq!(world.v1 - world.v2, vector(n1, n2, n3));
      };

      then regex "zero \\- v = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64,f64,f64) |world, n1, n2, n3, _step| {
        assert_eq!(world.zero - world.v, vector(n1, n2, n3));
      };

      then regex "a \\* ([-+]?[0-9]*\\.?[0-9]+) = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64, f64,f64,f64,f64) |world, scalar, n1, n2, n3, n4, _step| {
        assert_eq!(world.a * scalar, tuple(n1, n2, n3, n4));
      };

      then regex "a / ([-+]?[0-9]*\\.?[0-9]+) = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f64, f64,f64,f64,f64) |world, scalar, n1, n2, n3, n4, _step| {
        assert_eq!(world.a / scalar, tuple(n1, n2, n3, n4));
      };

      then regex "magnitude\\(v\\) = ([-+]?[0-9]*\\.?[0-9]+)" (f64) |world, number, _step| {
        assert_eq!(world.v.magnitude(), number);
      };

      then regex "magnitude\\(norm\\) = ([-+]?[0-9]*\\.?[0-9]+)" (f64) |world, number, _step| {
        assert_eq!(world.norm.magnitude(), number);
      };

      then "magnitude(v) = √14" |world, _step| {
        assert_eq!(world.v.magnitude(), 14f64.sqrt());
      };

      then "normalize(v) = vector(1, 0, 0)" |world, _step| {
        assert_eq!(world.v.normalize(), vector(1f64,0f64,0f64));
      };

      then "normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)" |world, _step| {
        assert_eq!(world.v.normalize(), vector(world.v.get_x()/14f64.sqrt(), world.v.get_y()/14f64.sqrt(), world.v.get_z()/14f64.sqrt()));
      };

      then regex "dot\\(a, b\\) = ([-+]?[0-9]*\\.?[0-9]+)" (f64) |world, number, _step| {
        assert_eq!(world.a.dot(world.b), number);
      };
      
      then regex "cross\\(([a-z]), ([a-z])\\) = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (String, String, f64,f64,f64) |world, a1, _b1, n1, n2, n3, _step| {
        if a1 == "a" {
          assert_eq!(world.a.cross(world.b), vector(n1,n2,n3));
        } else {
          assert_eq!(world.b.cross(world.a), vector(n1,n2,n3));
        }
      };
  });
}
// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {
    
}

cucumber! {
  features: "./features", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      tuple_steps::steps
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ], 
  after: &[
      an_after_fn // Optional; called after each scenario
  ] 
}