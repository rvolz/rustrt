#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple};
use rustrt::matrix::{Matrix};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  ma: Matrix,
  mb: Matrix,
  mc: Matrix,
  mt: Matrix,
  p: Tuple,
  p2: Tuple,
  p3: Tuple,
  p4: Tuple,
  transform: Matrix,
  v: Tuple,
  inv: Matrix,
  half_quarter: Matrix,
  full_quarter: Matrix,
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    // This function is called every time a new scenario is started
    MyWorld {
        ma: Matrix::default(),
        mb: Matrix::default(),
        mc: Matrix::default(),
        mt: Matrix::default(),
        p: Tuple::default(),
        p2: Tuple::default(),
        p3: Tuple::default(),
        p4: Tuple::default(),
        transform: Matrix::default(),
        v: Tuple::default(),
        inv: Matrix::default(),
        half_quarter: Matrix::default(),
        full_quarter: Matrix::default(),
    }
  }
}

mod transformations_steps {
  use super::*;
  use rustrt::tuple::{point,vector};
  use rustrt::matrix::{translation,scaling,rotation_x,rotation_y,rotation_z,shearing};
  use core::f32::consts::{FRAC_PI_2,FRAC_PI_4,SQRT_2};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given regex "transform ← translation\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.transform = translation(x,y,z);
    };
    given regex "transform ← scaling\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.transform = scaling(x,y,z);
    };
    given regex "transform ← shearing\\((-?\\d+), (-?\\d+), (-?\\d+), (-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32,f32,f32,f32) |world,xy,xz,yx,yz,zx,zy,_step| {
      world.transform = shearing(xy,xz,yx,yz,zx,zy);
    };
    given regex "p ← point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
      world.p = point(n1, n2, n3);
    };
    given regex "v ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
      world.v = vector(n1, n2, n3);
    };
    given "inv ← inverse(transform)" |world, _step| {
      world.inv = world.transform.inverse();
    };
    given "half_quarter ← rotation_x(π / 4)" |world, _step| {
      world.half_quarter = rotation_x(FRAC_PI_4);
    };
    given "full_quarter ← rotation_x(π / 2)" |world, _step| {
      world.full_quarter = rotation_x(FRAC_PI_2);
    };
    given "half_quarter ← rotation_y(π / 4)" |world, _step| {
      world.half_quarter = rotation_y(FRAC_PI_4);
    };
    given "full_quarter ← rotation_y(π / 2)" |world, _step| {
      world.full_quarter = rotation_y(FRAC_PI_2);
    };
    given "half_quarter ← rotation_z(π / 4)" |world, _step| {
      world.half_quarter = rotation_z(FRAC_PI_4);
    };
    given "full_quarter ← rotation_z(π / 2)" |world, _step| {
      world.full_quarter = rotation_z(FRAC_PI_2);
    };
    given "inv ← inverse(half_quarter)" |world, _step| {
      world.inv = world.half_quarter.inverse();
    };
    given "A ← rotation_x(π / 2)" |world,_step| {
      world.ma = rotation_x(FRAC_PI_2);
    };
    given "B ← scaling(5, 5, 5)" |world,_step| {
      world.mb = scaling(5.0,5.0,5.0);
    };
    given "C ← translation(10, 5, 7)" |world,_step| {
      world.mc = translation(10.0,5.0,7.0);
    };
    when "p2 ← A * p" |world,_step| {
      world.p2 = &world.ma * &world.p;
    };
    when "p3 ← B * p2" |world,_step| {
      world.p3 = &world.mb * &world.p2;
    };
    when "p4 ← C * p3" |world,_step| {
      world.p4 = &world.mc * &world.p3;
    };
    when "T ← C * B * A" |world,_step| {
      let w1 = &world.mc * &world.mb;
      world.mt =  &w1 * &world.ma;
    };
    then regex "transform \\* p = point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(&world.transform * &world.p, point(x,y,z));
    };
    then regex "transform \\* v = vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(&world.transform * &world.v, vector(x,y,z));
    };
    then regex "inv \\* p = point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(&world.inv * &world.p, point(x,y,z));
    };
    then regex "inv \\* v = vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(&world.inv * &world.v, vector(x,y,z));
    };
    then "transform * v = v" |world,_step| {
      assert_eq!(&world.transform * &world.v, world.v);
    };
    then "half_quarter * p = point(0, √2/2, √2/2)" |world,_step| {
      assert_eq!(&world.half_quarter * &world.p, point(0.0,SQRT_2/2.0,SQRT_2/2.0));
    };
    then regex "full_quarter \\* p = point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(&world.full_quarter * &world.p, point(x,y,z));
    };
    then "inv * p = point(0, √2/2, -√2/2)" |world,_step| {
      assert_eq!(&world.inv * &world.p, point(0.0,SQRT_2/2.0,-SQRT_2/2.0));
    };
    then "half_quarter * p = point(√2/2, 0, √2/2)" |world,_step| {
      assert_eq!(&world.half_quarter * &world.p, point(SQRT_2/2.0,0.0,SQRT_2/2.0));
    };
    then "half_quarter * p = point(-√2/2, √2/2, 0)" |world,_step| {
      assert_eq!(&world.half_quarter * &world.p, point(-SQRT_2/2.0,SQRT_2/2.0,0.0));
    };
    then "p2 = point(1, -1, 0)" |world,_step| {
      assert_eq!(world.p2,point(1.0,-1.0,0.0));
    };
    then "p3 = point(5, -5, 0)" |world,_step| {
      assert_eq!(world.p3,point(5.0,-5.0,0.0));
    };
    then "p4 = point(15, 0, 7)" |world,_step| {
      assert_eq!(world.p4,point(15.0,0.0,7.0));
    };
    then "T * p = point(15, 0, 7)" |world,_step| {
      assert_eq!(&world.mt * &world.p,point(15.0,0.0,7.0));
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
  features: "./features/transformations", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      transformations_steps::steps,
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ],
  after: &[
      an_after_fn // Optional; called after each scenario
  ]
}
