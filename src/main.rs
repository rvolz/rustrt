pub mod tuple;
pub mod canvas;

use std::fs;
use crate::tuple::*;
use crate::canvas::*;

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

fn exc1() {
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

fn scale_position(width: i32, height: i32, position: Tuple) -> (i32, i32) {
  let mx = width as f64 / 100.0;
  let my = height as f64 / 100.0;
  let nx = (position.get_x()*mx) as i32;
  let ny = height - (position.get_y()*my) as i32;
  (nx, ny)
}

fn exc2() -> std::io::Result<()> {
  println!("Chapter 2 exercise");
  let v = tuple::vector(1.0, 1.8, 0.0);
  let mut p = Projectile {
    position: tuple::point(0.0, 1.0, 0.0),
    velocity: v.normalize() * 4.0
  };
  let e = Environment {
    gravity: tuple::vector(0.0, -0.1, 0.0),
    wind: tuple::vector(-0.01, 0.0, 0.0)
  };
  let mut c = canvas(1024, 960);
  let red = color(1.0,0.0,0.0);
  let mut ticks = 0;
  while p.position.get_y() >= &0.0f64 {
    p = tick(&e, &p);
    ticks += 1;
    println!("Ticks {:?}, Position {:?}",ticks, p.position);
    let (nx, ny) = scale_position(*c.width(), *c.height(), p.position);
    println!("Ticks {:?}, Scalled positon {:?},{:?}",ticks, nx, ny);
    c.write_pixel(nx, ny, red);
  }
  fs::write("exc2.ppm", c.canvas_to_ppm())
}

fn main() -> std::io::Result<()> {
  exc1();
  exc2()
}