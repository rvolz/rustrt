pub mod tuple;

use crate::tuple::*;
pub struct Projectile {
  position: Tuple,
  velocity: Tuple
}
pub struct Environment {
  gravity: Tuple,
  wind: Tuple
}
pub fn tick(env: &Environment, proj: &Projectile) -> Projectile {
  let position = proj.position + proj.velocity;
  let velocity = proj.velocity + env.gravity + env.wind;
  Projectile {
    position: position,
    velocity: velocity
  }
}


fn main() {
  println!("Chapter 1 exercise");
  let v = tuple::vector(1.0, 1.0, 0.0);
  let mut p = Projectile {
    position: tuple::point(0.0, 1.0, 0.0),
    velocity: v.normalize()
  };
  let e = Environment {
    gravity: tuple::vector(0.0, -0.1, 0.0),
    wind: tuple::vector(-0.01, 0.0, 0.0)
  };
  let mut ticks = 0;
  while p.position.get_y() >= &0.0f64 {
    p = tick(&e, &p);
    ticks += 1;
    println!("Ticks {:?}, Position {:?}",ticks, p.position)
  }
}
