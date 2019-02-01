#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple};
use rustrt::matrix::{Matrix, identity};
use rustrt::ray::{Ray,ray};
use rustrt::intersection::{Intersections,intersections};
use rustrt::sphere::{Sphere,sphere};
use rustrt::shape::{Shape};
use rustrt::material::{Material,material};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  origin: Tuple,
  m: Matrix,
  m2: Material,
  n: Tuple,
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
        m: Matrix::default(),
        m2: Material::default(),
        n: Tuple::default(),
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
  use rustrt::matrix::{translation,scaling,rotation_z};
  use core::f32::consts::PI;
  use round::round;
  use std::f64;
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
    given "m ← scaling(1, 0.5, 1) * rotation_z(π/5)" |world,_step| {
      world.m = scaling(1.0,0.5,1.0) * rotation_z(PI/5.0);
    };
    given regex "set_transform\\(s, translation\\((-?\\d+), (-?\\d+), (-?\\d+)\\)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.s.set_transform(translation(x,y,z));
    };
    given "set_transform(s, m)" |world,_step| {
      world.s.set_transform(world.m.clone());
    };
    given "m ← material()" |world,_step| {
      world.m2 = material();
    };
    given "m.ambient ← 1" |world,_step| {
      world.m2.set_ambient(1.0);
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
      world.s.set_transform(translation(x,y,z));
    };
    when regex "n ← normal_at\\(s, point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.n = world.s.normal_at(point(x,y,z));
    };
    when "n ← normal_at(s, point(√3/3, √3/3, √3/3))" |world,_step| {
      let t = 3f32.sqrt()/3f32;
      world.n = world.s.normal_at(point(t,t,t));
    };
    when "n ← normal_at(s, point(0, √2/2, -√2/2))" |world,_step| {
      let t1 = 2f32.sqrt()/2f32;
      let t2 = 2f32.sqrt()/-2f32;
      world.n = world.s.normal_at(point(0.0,t1,t2));
    };
    when "m ← s.material" |world,_step| {
      world.m2 = world.s.material().clone();
    };
    when "s.material ← m" |world,_step| {
      world.s.set_material(world.m2.clone());
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
    then regex "n = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      let x1 = round(*world.n.get_x() as f64,1) as f32;
      let y1 = round(*world.n.get_y() as f64,5) as f32;
      let z1 = round(*world.n.get_z() as f64,5) as f32;
      assert_eq!(vector(x1,y1,z1), vector(x,y,z));
    };
    then "n = vector(√3/3, √3/3, √3/3)" |world,_step| {
      let t = 3f32.sqrt()/3f32;
      assert_eq!(world.n, vector(t,t,t));
    };
    then "n = normalize(n)" |world,_step| {
      assert_eq!(world.n, world.n.normalize());
    };
    then "m = material()" |world,_step| {
      assert_eq!(world.m2, material());
    };
    then "s.material = m" |world,_step| {
      assert_eq!(world.s.material(),&world.m2);
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
