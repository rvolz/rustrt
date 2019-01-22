#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::intersection::{Intersections,Intersection,intersections,intersection};
use rustrt::shape::{Shape};
use rustrt::sphere::{Sphere,sphere};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  xs2: Intersections,
  i: Intersection,
  i1: Intersection,
  i2: Intersection,
  i3: Intersection,
  i4: Intersection,
  s: Sphere,
  oi: Option<Intersection>,
  shape: Shape
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    // This function is called every time a new scenario is started
    MyWorld {
        xs2: intersections(vec!()),
        i: intersection(0.0,Shape::default()),
        i1: intersection(0.0,Shape::default()),
        i2: intersection(0.0,Shape::default()),
        i3: intersection(0.0,Shape::default()),
        i4: intersection(0.0,Shape::default()),
        oi: Some(intersection(0.0,Shape::default())),
        s: sphere(),
        shape: Shape::default(),
    }
  }
}

mod intersections_steps {
  use super::*;
  use rustrt::intersection::{intersection,intersections};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given "s ← sphere()" |world, _step| {
      world.s = sphere();
    };
    given "i ← intersection(4, shape)" |world,_steps| {
      world.i = intersection(4.0, world.shape.clone());
    };
    given regex "i1 ← intersection\\((\\-?\\d), s\\)" (f32) |world,value,_steps| {
      world.i1 = intersection(value, world.shape.clone());
    };
    given regex "i2 ← intersection\\((\\-?\\d), s\\)" (f32) |world,value,_steps| {
      world.i2 = intersection(value, world.shape.clone());
    };
    given regex "i3 ← intersection\\((\\-?\\d), s\\)" (f32) |world,value,_steps| {
      world.i3 = intersection(value, world.shape.clone());
    };
    given regex "i4 ← intersection\\((\\-?\\d), s\\)" (f32) |world,value,_steps| {
      world.i4 = intersection(value, world.shape.clone());
    };
    given "xs ← intersections(i2, i1)" |world,_step| {
      world.xs2 = intersections(vec!(world.i2.clone(),world.i1.clone()));
    };
    given "xs ← intersections(i1, i2, i3, i4)" |world,_step| {
      world.xs2 = intersections(vec!(world.i1.clone(),world.i2.clone(),world.i3.clone(),world.i4.clone()));
    };
    when "i ← intersection(3.5, s)" |world,_step| {
      world.i = intersection(3.5, world.shape.clone());
    };
    when "xs ← intersections(i1, i2)" |world,_step| {
      world.xs2 = intersections(vec!(world.i1.clone(),world.i2.clone()));
    };

    when "i ← hit(xs)" |world,_step| {
      world.oi = world.xs2.hitc();
    };
    then "i.t = 3.5" |world,_step| {
      assert_eq!(*world.i.t(), 3.5);
    };
    then "i.object = s" |world,_step| {
      assert_eq!(*world.i.object(), world.shape);
    };
    then "xs.count = 2" |world,_step| {
      assert_eq!(*world.xs2.count(), 2);
    };
    then "xs[0].t = 1" |world,_step| {
      assert_eq!(*world.xs2.intersections()[0].t(), 1.0);
    };
    then "xs[1].t = 2" |world,_step| {
      assert_eq!(*world.xs2.intersections()[1].t(), 2.0);
    };
    then "i = i1" |world,_step| {
      assert_eq!(&world.oi, &Some(world.i1.clone()));
    };
    then "i = i2" |world,_step| {
      assert_eq!(&world.oi, &Some(world.i2.clone()));
    };
    then "i is nothing" |world,_step| {
      assert_eq!(&world.oi, &None);
    };
    then "i = i4" |world,_step| {
      assert_eq!(&world.oi, &Some(world.i4.clone()));
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
  features: "./features/intersections", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      intersections_steps::steps,
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ],
  after: &[
      an_after_fn // Optional; called after each scenario
  ]
}
