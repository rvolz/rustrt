#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple};
use rustrt::matrix::{Matrix, identity};
use rustrt::ray::{Ray,ray};
use rustrt::intersection::{Intersections,intersections};
use rustrt::sphere::{Sphere,sphere};
use rustrt::shape::{Shape};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  origin: Tuple,
  r: Ray,
  t: Matrix,
  s: Sphere,
  xs: Option<(f32,f32)>,
  xs2: Intersections,
  shape: Shape,
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    // This function is called every time a new scenario is started
    MyWorld {
        origin: Tuple::default(),
        r: ray(Tuple::default(),Tuple::default()),
        t: Matrix::default(),
        s: sphere(),
        xs: None,
        xs2: intersections(vec!()),
        shape: Shape::Sphere(sphere()),
    }
  }
}


mod spheres_steps {
  use super::*;
  use rustrt::body::Body;
  use rustrt::tuple::{point,vector};
  use rustrt::matrix::{translation,scaling};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given regex "origin ← point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.origin = point(x,y,z);
    };
    given "s ← sphere()" |world, _step| {
      world.s = sphere();
    };
    given regex "r ← ray\\(point\\((-?\\d+), (-?\\d+), (-?\\d+)\\), vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)\\)" (f32,f32,f32,f32,f32,f32) |world,x1,y1,z1,x2,y2,z2,_step| {
      world.r = ray(point(x1,y1,z1),vector(x2,y2,z2));
    };
    given "shape ← sphere()" |world,_steps| {
      world.shape = Shape::Sphere(sphere());
    };
    given regex "t ← translation\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.t = translation(x,y,z);
    };
    when "xs1 ← intersect(s, r)" |world,_step| {
      world.xs = world.s.intersect(world.r);
    };
    when "xs ← intersect(shape, r)" |world,_step| {
      world.xs2 = world.shape.intersect(world.r);
    };
    when "set_transform(s, t)" |world,_step| {
      world.s.set_transform(world.t.clone());
    };
    when regex "set_transform\\(s, scaling\\((-?\\d+), (-?\\d+), (-?\\d+)\\)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.s.set_transform(scaling(x,y,z));
    };
    when regex "set_transform\\(s, translation\\((-?\\d+), (-?\\d+), (-?\\d+)\\)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.s.set_transform(scaling(x,y,z));
    };
    then "xs1.count = 0" |world,_step| {
      assert!(world.xs.is_none());
    };
    then "xs1.count = 2" |world,_step| {
      assert!(world.xs.is_some());
    };
    then regex "xs1\\[0\\] = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,value,_step| {
      assert_eq!(world.xs.unwrap().0,value);
    };
    then regex "xs1\\[1\\] = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,value,_step| {
      assert_eq!(world.xs.unwrap().1,value);
    };
    then "xs.count = 2" |world,_step| {
      assert_eq!(*world.xs2.count(), 2);
    };
    then "xs[0].object = shape" |world,_step| {
      assert_eq!(world.xs2.intersections()[0].object(),&world.shape);
    };
    then "xs[1].object = shape" |world,_step| {
      assert_eq!(world.xs2.intersections()[0].object(),&world.shape);
    };
    then "s.transform = identity_matrix" |world,_step| {
      assert_eq!(world.s.transform(), &identity());
    };
    then "s.transform = t" |world,_step| {
      assert_eq!(world.s.transform(), &world.t);
    };
    then regex "xs1\\[0\\].t = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,value,_step| {
      assert_eq!(world.xs.unwrap().0,value);
    };
    then regex "xs1\\[1\\].t = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,value,_step| {
      assert_eq!(world.xs.unwrap().1,value);
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
  features: "./features/spheres", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      spheres_steps::steps,
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ],
  after: &[
      an_after_fn // Optional; called after each scenario
  ]
}
