#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple};
use rustrt::matrix::{Matrix};
use rustrt::ray::{Ray,ray};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  direction: Tuple,
  m: Matrix,
  origin: Tuple,
  r: Ray,
  r2: Ray
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    // This function is called every time a new scenario is started
    MyWorld {
        direction: Tuple::default(),
        m: Matrix::default(),
        origin: Tuple::default(),
        r: ray(Tuple::default(),Tuple::default()),
        r2: ray(Tuple::default(),Tuple::default())
    }
  }
}


mod rays_steps {
  use super::*;
  use rustrt::tuple::{point,vector};
  use rustrt::matrix::{scaling,translation};
  use rustrt::ray::{ray};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given regex "origin ← point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.origin = point(x,y,z);
    };
    given regex "direction ← vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.direction = vector(x,y,z);
    };
    given regex "r ← ray\\(point\\((-?\\d+), (-?\\d+), (-?\\d+)\\), vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)\\)" (f32,f32,f32,f32,f32,f32) |world,x1,y1,z1,x2,y2,z2,_step| {
      world.r = ray(point(x1,y1,z1),vector(x2,y2,z2));
    };
    given regex "m ← translation\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.m = translation(x,y,z);
    };
    given regex "m ← scaling\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.m = scaling(x,y,z);
    };
    when "r ← ray(origin, direction)" |world, _step| {
      world.r = ray(world.origin, world.direction);
    };
    when "r2 ← transform(r, m)" |world, _step| {
      world.r2 = world.r.set_transform(&world.m);
    };
    then "r.origin = origin" |world, _step| {
      assert_eq!(world.r.origin(), &world.origin);
    };
    then "r.direction = direction" |world, _step| {
      assert_eq!(world.r.direction(), &world.direction);
    };
    then regex "r2.origin = point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(world.r2.origin(), &point(x,y,z));
    };
    then regex "r2.direction = vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(world.r2.direction(), &vector(x,y,z));
    };
    then regex "position\\(r, ([-+]?[0-9]*\\.?[0-9]+)\\) = point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world,t,x,y,z,_step| {
      assert_eq!(world.r.position(t), point(x,y,z));
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
  features: "./features/rays", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      rays_steps::steps,
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ],
  after: &[
      an_after_fn // Optional; called after each scenario
  ]
}
