#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple};
use rustrt::material::{Material,material};
use rustrt::light::{Light,point_light};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  eyev: Tuple,
  light: Light,
  m: Material,
  normalv: Tuple,
  position: Tuple,
  result: Tuple
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    // This function is called every time a new scenario is started
    MyWorld {
        eyev: Tuple::default(),
        light: Light::default(),
        m: Material::default(),
        normalv: Tuple::default(),
        position: Tuple::default(),
        result: Tuple::default()
    }
  }
}


mod materials_steps {
  use super::*;
  use rustrt::tuple::{point,color,vector};
  use round::round;
  use std::f64;
  use std::f32;
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given "m ← material()" |world,_step| {
      world.m = material();
    };
    given regex "eyev ← vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.eyev = vector(x,y,z);
    };
    given "eyev ← vector(0, -√2/2, -√2/2)" |world,_step| {
      world.eyev = vector(0.0,-2.0f32.sqrt()/2.0,-2.0f32.sqrt()/2.0);
    };
    given "eyev ← vector(0, √2/2, -√2/2)" |world,_step| {
      world.eyev = vector(0.0,2.0f32.sqrt()/2.0,-2.0f32.sqrt()/2.0);
    };
    given regex "normalv ← vector\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.normalv = vector(x,y,z);
    };
    given regex "position ← point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.position = point(x,y,z);
    };
    given regex "light ← point_light\\(point\\((-?\\d+), (-?\\d+), (-?\\d+)\\), color\\((-?\\d+), (-?\\d+), (-?\\d+)\\)\\)" (f32,f32,f32,f32,f32,f32) |world,x,y,z,c1,c2,c3,_step| {
      world.light = point_light(point(x,y,z), color(c1,c2,c3));
    };
    when "result ← lighting(m, light, position, eyev, normalv)" |world,_step| {
      world.result = world.m.lighting(world.light, world.position, world.eyev, world.normalv);
    };
    then regex "m.color = color\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      assert_eq!(world.m.color(), &color(x,y,z));
    };
    then regex "m.ambient = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,a,_step| {
      assert_eq!(world.m.ambient(), a);
    };
    then regex "m.diffuse = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,a,_step| {
      assert_eq!(world.m.diffuse(), a);
    };
    then regex "m.specular = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,a,_step| {
      assert_eq!(world.m.specular(), a);
    };
    then regex "m.shininess = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,a,_step| {
      assert_eq!(world.m.shininess(), a);
    };
    then regex "result = color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      let x1 = round(*world.result.get_x() as f64,4) as f32;
      let y1 = round(*world.result.get_y() as f64,4) as f32;
      let z1 = round(*world.result.get_z() as f64,4) as f32;
      assert_eq!(color(x1,y1,z1), color(x,y,z));
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
  features: "./features/materials", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      materials_steps::steps,
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ],
  after: &[
      an_after_fn // Optional; called after each scenario
  ]
}
