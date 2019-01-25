#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple};
use rustrt::light::{Light,light};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  intensity: Tuple,
  position: Tuple,
  light: Light
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    // This function is called every time a new scenario is started
    MyWorld {
        intensity: Tuple::default(),
        position: Tuple::default(),
        light: Light::default(),
    }
  }
}


mod lights_steps {
  use super::*;
  use rustrt::tuple::{point,color};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given regex "intensity ← color\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.intensity = color(x,y,z);
    };
    given regex "position ← point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.position = point(x,y,z);
    };
    when "light ← point_light(position, intensity)" |world, _step| {
      world.light = light(world.intensity, world.position);
    };
    then "light.position = position" |world, _step| {
      assert_eq!(world.light.position(), &world.position);
    };
    then "light.intensity = intensity" |world, _step| {
      assert_eq!(world.light.intensity(), &world.intensity);
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
  features: "./features/lights", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      lights_steps::steps,
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ],
  after: &[
      an_after_fn // Optional; called after each scenario
  ]
}
