pub mod tuple;
pub mod canvas;
pub mod matrix;
pub mod ray;

use core::f32::consts::{PI};
use std::fs;
use crate::tuple::*;
use crate::canvas::*;
use crate::matrix::*;

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

fn _exc1() {
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
  while p.position.get_y() >= &0.0f32 {
    p = tick(&e, &p);
    ticks += 1;
    println!("Ticks {:?}, Position {:?}",ticks, p.position)
  }
}

fn _scale_position(width: i32, height: i32, position: Tuple) -> (i32, i32) {
  let mx = width as f32 / 100.0;
  let my = height as f32 / 100.0;
  let nx = (position.get_x()*mx) as i32;
  let ny = height - (position.get_y()*my) as i32;
  (nx, ny)
}

fn _exc2() -> std::io::Result<()> {
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
  while p.position.get_y() >= &0.0f32 {
    p = tick(&e, &p);
    ticks += 1;
    println!("Ticks {:?}, Position {:?}",ticks, p.position);
    let (nx, ny) = _scale_position(*c.width(), *c.height(), p.position);
    println!("Ticks {:?}, Scalled positon {:?},{:?}",ticks, nx, ny);
    c.write_pixel(nx, ny, red);
  }
  fs::write("exc2.ppm", c.canvas_to_ppm())
}

fn cconvert(t: &Tuple, c: &Canvas) -> (i32, i32) {
  let mid_x = *c.width() as f32 / 2.0;
  let mid_y = *c.height() as f32 / 2.0;
  let x = mid_x * t.get_x();
  let y =  mid_y * t.get_y();
  ((mid_x + x) as i32, (mid_y+y) as i32)  
}

fn exc3() -> std::io::Result<()> {
  let mut c = canvas(800, 800);
  let red = color(1.0,0.0,0.0);
  let times: Vec<Tuple> = (0..12).map(|_| point(0.0,0.0,0.0)).collect();
  let times2: [f32;12] = [PI/2.0,PI/3.0,PI/6.0,2.0*PI,11.0*PI/6.0,5.0*PI/3.0,3.0*PI/2.0,4.0*PI/3.0,7.0*PI/6.0,PI,5.0*PI/6.0,2.0*PI/3.0];
  for tt in times.iter().zip(times2.iter()) {
    let p = tt.0;
    let w = tt.1;
    let t = &rotation_z(*w) * &translation(0.0,0.9,0.0);
    let t12 = &t * p;
    let s12 = cconvert(&t12, &c);
    println!("Positon {:?}, scaled {:?}", t12, s12);
    c.write_pixel(s12.0, s12.1, red);
  }
  fs::write("exc3.ppm", c.canvas_to_ppm())
}

fn main() -> std::io::Result<()> {
  //exc1();
  //exc2();
  exc3()
}
