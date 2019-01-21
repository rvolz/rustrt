#[macro_use]
extern crate cucumber_rust;
extern crate rustrt;
use rustrt::tuple::{Tuple, tuple, color};
use rustrt::canvas::{Canvas, canvas};
use rustrt::matrix::{Matrix, matrix, identity};
use rustrt::ray::{Ray};
use rustrt::sphere::{sphere,Sphere};
use rustrt::shape::{Shape};
use rustrt::intersection::{Intersection,intersection,Intersections,intersections};
#[allow(unused_imports)] // Import is required, though
use float_cmp::{ApproxEq};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  a: Tuple,
  a1: Tuple,
  a2: Tuple,
  b: Tuple,
  c: Tuple,
  c1: Tuple,
  c2: Tuple,
  c3: Tuple,
  direction: Tuple,
  p: Tuple,
  p1: Tuple,
  p2: Tuple,
  p3: Tuple,
  p4: Tuple,
  v: Tuple,
  v1: Tuple,
  v2: Tuple,
  zero: Tuple,
  norm: Tuple,
  cc: Canvas,
  red: Tuple,
  ppm: String,
  m: Matrix,
  ma: Matrix,
  mb: Matrix,
  mc: Matrix,
  mt: Matrix,
  origin: Tuple,
  r: Ray,
  r2: Ray,
  transform: Matrix,
  inv: Matrix,
  half_quarter: Matrix,
  full_quarter: Matrix,
  s: Sphere,
  xs: Option<(f32,f32)>,
  xs2: Intersections,
  i: Intersection,
  i1: Intersection,
  i2: Intersection,
  i3: Intersection,
  i4: Intersection,
  shape: Shape,
  oi: Option<Intersection>,
  oi1: Option<Intersection>,
  oi2: Option<Intersection>,
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    use rustrt::ray::{ray};
    // This function is called every time a new scenario is started
    MyWorld {
        a: tuple(0.0,0.0,0.0,0.0),
        a1: tuple(0.0,0.0,0.0,0.0),
        a2: tuple(0.0,0.0,0.0,0.0),
        b: tuple(0.0,0.0,0.0,0.0),
        c: tuple(0.0,0.0,0.0,0.0),
        c1: tuple(0.0,0.0,0.0,0.0),
        c2: tuple(0.0,0.0,0.0,0.0),
        c3: tuple(0.0,0.0,0.0,0.0),
        direction: tuple(0.0,0.0,0.0,0.0),
        p: tuple(0.0,0.0,0.0,0.0),
        p1: tuple(0.0,0.0,0.0,0.0),
        p2: tuple(0.0,0.0,0.0,0.0),
        p3: tuple(0.0,0.0,0.0,0.0),
        p4: tuple(0.0,0.0,0.0,0.0),
        v: tuple(0.0,0.0,0.0,0.0),
        v1: tuple(0.0,0.0,0.0,0.0),
        v2: tuple(0.0,0.0,0.0,0.0),
        zero: tuple(0.0,0.0,0.0,0.0),
        norm: tuple(0.0,0.0,0.0,0.0),
        cc: canvas(0,0),
        red: color(1.0,0.0,0.0),
        ppm: String::new(),
        m: matrix(0,0),
        ma: matrix(0,0),
        mb: matrix(0,0),
        mc: matrix(0,0),
        mt: matrix(0,0),
        origin: tuple(0.0,0.0,0.0,0.0),
        r: ray(tuple(0.0,0.0,0.0,0.0),tuple(0.0,0.0,0.0,0.0)),
        r2: ray(tuple(0.0,0.0,0.0,0.0),tuple(0.0,0.0,0.0,0.0)),
        transform: matrix(0,0),
        inv: matrix(0,0),
        half_quarter: matrix(0,0),
        full_quarter: matrix(0,0),
        s: sphere(),
        xs: None,
        xs2: intersections(vec!()),
        i: intersection(0.0,Shape::Sphere(sphere())),
        i1: intersection(0.0,Shape::Sphere(sphere())),
        i2: intersection(0.0,Shape::Sphere(sphere())),
        i3: intersection(0.0,Shape::Sphere(sphere())),
        i4: intersection(0.0,Shape::Sphere(sphere())),
        shape: Shape::Sphere(sphere()),
        oi: Some(intersection(0.0,Shape::Sphere(sphere()))),
        oi1: Some(intersection(0.0,Shape::Sphere(sphere()))),
        oi2: Some(intersection(0.0,Shape::Sphere(sphere()))),
    }
  }
}

mod tuple_steps {
  use super::*;
  use rustrt::tuple::{tuple,point,vector,color};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
      given regex "a ← tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
        world.a = tuple(n1, n2, n3, n4);
      };

      given regex "a1 ← tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
        world.a1 = tuple(n1, n2, n3, n4);
      };
      given regex "a2 ← tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
        world.a2 = tuple(n1, n2, n3, n4);
      };

      given regex "p ← point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.p = point(n1, n2, n3);
      };

      given regex "p1 ← point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.p1 = point(n1, n2, n3);
      };
      given regex "p2 ← point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.p2 = point(n1, n2, n3);
      };

      given regex "a ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.a = vector(n1, n2, n3);
      };

      given regex "b ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.b = vector(n1, n2, n3);
      };

      given regex "v ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.v = vector(n1, n2, n3);
      };

      given regex "v1 ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.v1 = vector(n1, n2, n3);
      };
      given regex "v2 ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.v2 = vector(n1, n2, n3);
      };

      given regex "zero ← vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.v = vector(n1, n2, n3);
      };

      given regex "c ← color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.c = color(n1, n2, n3);
      };
      given regex "c1 ← color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.c1 = color(n1, n2, n3);
      };
      given regex "c2 ← color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        world.c2 = color(n1, n2, n3);
      };

      when "norm ← normalize(v)" |world, _step| {
        world.norm = world.v.normalize();
      };

      then regex "a.x = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world, number, _step| {
          assert_eq!(world.a.get_x(), &number);
      };

      then regex "a.y = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world, number, _step| {
          assert_eq!(world.a.get_y(), &number);
      };

      then regex "a.z = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world, number, _step| {
          assert_eq!(world.a.get_z(), &number);
      };

      then regex "a.w = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world, number, _step| {
          assert_eq!(world.a.get_w(), &number);
      };

      then "a is a point" |world, _step| {
        assert!(world.a.is_point());
      };

      then "a is not a point" |world, _step| {
        assert!(!world.a.is_point());
      };

      then "a is a vector" |world, _step| {
        assert!(world.a.is_vector());
      };

      then "a is not a vector" |world, _step| {
        assert!(!world.a.is_vector());
      };

      then regex "-a = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
        assert_eq!(-world.a, tuple(n1, n2, n3, n4));
      };

      then regex "p = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
        assert_eq!(world.p, tuple(n1, n2, n3, n4));
      };

      then regex "v = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
        assert_eq!(world.v, tuple(n1, n2, n3, n4));
      };

      then regex "a1 \\+ a2 = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
        assert_eq!(world.a1 + world.a2, tuple(n1, n2, n3, n4));
      };

      then regex "p1 \\- p2 = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        assert_eq!(world.p1 - world.p2, vector(n1, n2, n3));
      };

      then regex "p \\- v = point\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        assert_eq!(world.p - world.v, point(n1, n2, n3));
      };

      then regex "v1 \\- v2 = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        assert_eq!(world.v1 - world.v2, vector(n1, n2, n3));
      };

      then regex "zero \\- v = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        assert_eq!(world.zero - world.v, vector(n1, n2, n3));
      };

      then regex "a \\* ([-+]?[0-9]*\\.?[0-9]+) = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32, f32,f32,f32,f32) |world, scalar, n1, n2, n3, n4, _step| {
        assert_eq!(world.a * scalar, tuple(n1, n2, n3, n4));
      };

      then regex "a / ([-+]?[0-9]*\\.?[0-9]+) = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32, f32,f32,f32,f32) |world, scalar, n1, n2, n3, n4, _step| {
        assert_eq!(world.a / scalar, tuple(n1, n2, n3, n4));
      };

      then regex "magnitude\\(v\\) = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world, number, _step| {
        assert_eq!(world.v.magnitude(), number);
      };

      then regex "magnitude\\(norm\\) = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world, number, _step| {
        assert!(world.norm.magnitude().approx_eq(&number, 2.0 * ::std::f32::EPSILON, 10));
      };

      then "magnitude(v) = √14" |world, _step| {
        assert_eq!(world.v.magnitude(), 14f32.sqrt());
      };

      then "normalize(v) = vector(1, 0, 0)" |world, _step| {
        assert_eq!(world.v.normalize(), vector(1f32,0f32,0f32));
      };

      then "normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)" |world, _step| {
        assert_eq!(world.v.normalize(), vector(world.v.get_x()/14f32.sqrt(), world.v.get_y()/14f32.sqrt(), world.v.get_z()/14f32.sqrt()));
      };

      then regex "dot\\(a, b\\) = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world, number, _step| {
        assert_eq!(world.a.dot(world.b), number);
      };

      then regex "cross\\(([a-z]), ([a-z])\\) = vector\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (String, String, f32,f32,f32) |world, a1, _b1, n1, n2, n3, _step| {
        if a1 == "a" {
          assert_eq!(world.a.cross(world.b), vector(n1,n2,n3));
        } else {
          assert_eq!(world.b.cross(world.a), vector(n1,n2,n3));
        }
      };

      then regex "c.([a-z]+) = ([-+]?[0-9]*\\.?[0-9]+)" (String,f32) |world, color, number, _step| {
        let mut result = &0.0f32;
        match color.as_ref() {
          "red" => result = world.c.red(),
          "green" => result = world.c.green(),
          "blue" => result = world.c.blue(),
          _ => (),
        }
        assert_eq!(result, &number);
      };

      then regex "c1 \\+ c2 = color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        assert_eq!(world.c1 + world.c2, color(n1, n2, n3));
      };

      then regex "c1 \\- c2 = color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        // due to rounding problems
        let c: Tuple = world.c1 - world.c2;
        let p = tuple(
          (c.get_x() * 10.0).round() / 10.0,
          (c.get_y() * 10.0).round() / 10.0,
          (c.get_z() * 10.0).round() / 10.0,
          0.0
        );
        assert_eq!(p, color(n1, n2, n3));
      };

      then regex "c \\* ([-+]?[0-9]*\\.?[0-9]+) = color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32, f32,f32,f32) |world, scalar, n1, n2, n3, _step| {
        assert_eq!(world.c * scalar, color(n1, n2, n3));
      };

      then regex "c1 \\* c2 = color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
        // due to rounding problems
        let c: Tuple = world.c1 * world.c2;
        let p = tuple(
          (c.get_x() * 10.0).round() / 10.0,
          (c.get_y() * 10.0).round() / 10.0,
          (c.get_z() * 100.0).round() / 100.0,
          0.0
        );
        assert_eq!(p, color(n1, n2, n3));
      };

  });
}

mod canvas_steps {
  use super::*;
  use rustrt::tuple::{color};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given regex "c ← canvas\\(([0-9]+), ([0-9]+)\\)" (i32,i32) |world, w, h, _step| {
      world.cc = canvas(w,h);
    };

    given "red ← color(1, 0, 0)" |world, _step| {
     world.red = color(1.0,0.0,0.0);
    };

    given regex "c3 ← color\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32) |world, n1, n2, n3, _step| {
      world.c3 = color(n1, n2, n3);
    };


    when regex "write_pixel\\(c, ([0-9]+), ([0-9]+), ([a-z0-9]+)\\)" (i32, i32, String) |world, x, y, c, _step| {
      let color;
      match c.as_ref() {
        "red" => color = world.red,
        "c1" => color = world.c1,
        "c2" => color = world.c2,
        "c3" => color = world.c3,
        _ => panic!("Unkonw color!")
      }
      world.cc.write_pixel(x,y,color);
    };

    when "ppm ← canvas_to_ppm(c)" |world, _step| {
      world.ppm = world.cc.canvas_to_ppm();
    };

    when "every pixel of c is set to color(1, 0.8, 0.6)" |world, _step| {
      let c = color(1.0,0.8,0.6);
      world.cc.clear_to_color(c);
    };

    then "c.width = 10"  |world, _step| {
      assert_eq!(world.cc.width(), &10);
    };

    then "c.height = 20" |world, _step| {
      assert_eq!(world.cc.height(), &20);
    };

    then "every pixel of c is color(0, 0, 0)" |world, _step| {
      let black = color(0.0,0.0,0.0);
      for pixel in world.cc.pixels() {
        assert_eq!(pixel, &black);
      }
    };

    then "pixel_at(c, 2, 3) = red" |world, _step| {
      assert_eq!(world.cc.pixel_at(2,3), world.red);
    };

    then "lines 1-3 of ppm are" |world, step| {
      let txt = step.docstring().unwrap();
      let header:Vec<&str> = world.ppm.split('\n').collect();
      assert_eq!(&header[0..3].join("\n"), txt);
    };

    then "lines 4-6 of ppm are" |world, step| {
      let txt = step.docstring().unwrap();
      let header:Vec<&str> = world.ppm.split('\n').collect();
      assert_eq!(&header[3..6].join("\n"), txt);
    };

    then "lines 4-7 of ppm are" |world, step| {
      let txt = step.docstring().unwrap();
      let header:Vec<&str> = world.ppm.split('\n').collect();
      assert_eq!(&header[3..7].join("\n"), txt);
    };

    then "the last character of ppm is a newline" |world, _step| {
      assert!(world.ppm.ends_with('\n'));
    };
  });
}

mod matrix_steps {
  use super::*;
  use rustrt::matrix::{matrix};
  steps!(MyWorld => {
    given regex "the following (\\d)x(\\d) matrix (\\w):" (usize,usize,char) |world,rows,columns,mname,step| {
      let table = step.table().unwrap();
      let mw;
      if mname == 'A' {
        world.ma = matrix(rows as u32,columns as u32);
        mw = &mut world.ma;
      } else if mname == 'B' {
        world.mb = matrix(rows as u32,columns as u32);
        mw = &mut world.mb;
      } else {
        world.m = matrix(rows as u32,columns as u32);
        mw = &mut world.m;
      }
      for row in 0..rows {
        for column in 0..columns {
          mw[(row as u32,column as u32)] = table.rows[row][column].parse::<f32>().unwrap();
        }
      }
    };
    given regex "the following matrix (\\w):" (char) |world, mname, step| {
      let table = step.table().unwrap();
      let rows = table.rows.len();
      let columns = table.rows.len();
      let mw;
      if mname == 'A' {
        world.ma = matrix(rows as u32,columns as u32);
        mw = &mut world.ma;
      } else {
        world.mb = matrix(rows as u32,columns as u32);
        mw = &mut world.mb;
      }
      for row in 0..rows {
        for column in 0..columns {
          mw[(row as u32,column as u32)] = table.rows[row][column].parse::<f32>().unwrap();
        }
      }
    };
    given regex "b ← tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
      world.b = tuple(n1, n2, n3, n4);
    };
    given "A ← transpose(identity_matrix)" |world,_step| {
      world.ma = identity().transpose();
    };
    given "B ← submatrix(A, 1, 0)" |world,_step| {
      world.mb = world.ma.submatrix(1,0);
    };
    given "B ← inverse(A)" |world,_step| {
      world.mb = world.ma.inverse();
    };
    given "C ← A * B" |world,_step| {
      world.mc = &world.ma * &world.mb;
    };
    then regex "M\\[(\\d+),(\\d+)\\] = ([-+]?[0-9]*\\.?[0-9]+)" (u32,u32,f32) |world,x,y,value,_step| {
      assert_eq!(world.m[(x,y)], value);
    };
    then "A = B" |world, _step| {
      assert_eq!(world.ma, world.mb);
    };
    then "A != B" |world, _step| {
      assert_ne!(world.ma, world.mb);
    };
    then regex "A \\* B is the following (\\d)x(\\d) matrix:" (usize,usize) |world,rows,columns,step| {
      let table = step.table().unwrap();
      world.m = matrix(rows as u32,columns as u32);
      for row in 0..rows {
        for column in 0..columns {
          world.m[(row as u32,column as u32)] = table.rows[row][column].parse::<f32>().unwrap();
        }
      }
      assert_eq!(&world.ma * &world.mb, world.m);
    };
    then regex "A \\* b = tuple\\(([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+), ([-+]?[0-9]*\\.?[0-9]+)\\)" (f32,f32,f32,f32) |world, n1, n2, n3, n4, _step| {
      assert_eq!(&world.ma * &world.b, tuple(n1, n2, n3, n4));
    };
    then "A * identity_matrix = A" |world,_step| {
      assert_eq!(&world.ma * &identity(), world.ma);
    };
    then "identity_matrix * a = a" |world,_step| {
      assert_eq!(&identity() * &world.a, world.a);
    };
    then "transpose(A) is the following matrix:" |world,step| {
      let table = step.table().unwrap();
      world.m = matrix(4,4);
      for row in 0..4 {
        for column in 0..4 {
          world.m[(row as u32,column as u32)] = table.rows[row][column].parse::<f32>().unwrap();
        }
      }
      assert_eq!(world.ma.transpose(), world.m);
    };
    then "A = identity_matrix" |world,_step| {
      assert_eq!(world.ma, identity());
    };
    then regex "determinant\\((\\w)\\) = (([-+]?[0-9]*\\.?[0-9]+))" (char,f32) |world,mname,result,_step| {
      let mw;
      if mname == 'A' {
        mw = &mut world.ma;
      } else {
        mw = &mut world.mb;
      }
      assert_eq!(mw.determinant(), result);
    };
    then regex "submatrix\\((\\w), (\\d), (\\d)\\) is the following (\\d)x(\\d) matrix:" (char,usize,usize,usize,usize) |world,mname,row,column,rows,columns,step| {
      let table = step.table().unwrap();
      world.m = matrix(rows as u32,columns as u32);
      for row in 0..rows {
        for column in 0..columns {
          world.m[(row as u32,column as u32)] = table.rows[row][column].parse::<f32>().unwrap();
        }
      }
      let mw;
      if mname == 'A' {
        mw = &mut world.ma;
      } else {
        mw = &mut world.mb;
      }
      assert_eq!(mw.submatrix(row,column), world.m);
    };
    then regex "minor\\((\\w), (\\d), (\\d)\\) = (([-+]?[0-9]*\\.?[0-9]+))" (char,usize,usize,f32) |world,mname,row,col,result,_step| {
      let mw;
      if mname == 'A' {
        mw = &mut world.ma;
      } else {
        mw = &mut world.mb;
      }
      assert_eq!(mw.minor(row,col), result);
    };
    then regex "cofactor\\((\\w), (\\d), (\\d)\\) = (([-+]?[0-9]*\\.?[0-9]+))" (char,usize,usize,f32) |world,mname,row,col,result,_step| {
      let mw;
      if mname == 'A' {
        mw = &mut world.ma;
      } else {
        mw = &mut world.mb;
      }
      assert_eq!(mw.cofactor(row,col), result);
    };
    then "A is invertible" |world, _step| {
      assert!(world.ma.is_invertible())
    };
    then "A is not invertible" |world, _step| {
      assert!(!world.ma.is_invertible())
    };
    then regex "B\\[(\\d),(\\d)\\] = (-?\\d+)/(-?\\d+)" (usize,usize,f32,f32) |world,row,col,x,y,_step| {
      assert_eq!(world.mb[(row as u32,col as u32)], x/y);
    };
    then "B is the following 4x4 matrix:" |world,step| {
      let table = step.table().unwrap();
      world.m = matrix(4,4);
      for row in 0..4 {
        for column in 0..4 {
          world.m[(row as u32,column as u32)] = table.rows[row][column].parse::<f32>().unwrap();
        }
      }
      // due to 3-digit precision differences we have to use ULPS 1000 for testing
      assert!(world.mb.approx_eq(&world.m,2.0 * ::std::f32::EPSILON, 1000));
    };
    then "inverse(A) is the following 4x4 matrix:" |world,step| {
      let table = step.table().unwrap();
      world.m = matrix(4,4);
      for row in 0..4 {
        for column in 0..4 {
          world.m[(row as u32,column as u32)] = table.rows[row][column].parse::<f32>().unwrap();
        }
      }
      // due to 3-digit precision differences we have to use ULPS 1000-2000 for testing
      assert!(world.ma.inverse().approx_eq(&world.m,2.0 * ::std::f32::EPSILON, 2000));
      //assert_eq!(world.ma.inverse(), world.m);
    };

    then "C * inverse(B) = A" |world,_step| {
      assert_eq!(&world.mc * &world.mb.inverse(), world.ma);
    };
  });
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
      world.r2 = world.r.transform(&world.m);
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

mod spheres_steps {
  use super::*;
  use rustrt::body::Body;
  use rustrt::tuple::{point};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given regex "origin ← point\\((-?\\d+), (-?\\d+), (-?\\d+)\\)" (f32,f32,f32) |world,x,y,z,_step| {
      world.origin = point(x,y,z);
    };
    given "s ← sphere()" |world, _step| {
      world.s = sphere();
    };
    when "xs ← intersect(s, r)" |world,_step| {
      world.xs = world.s.intersect(world.r);
    };
    then "xs.count = 0" |world,_step| {
      assert!(world.xs.is_none());
    };
    then "xs.count = 2" |world,_step| {
      assert!(world.xs.is_some());
    };
    then regex "xs\\[0\\] = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,value,_step| {
      assert_eq!(world.xs.unwrap().0,value);
    };
    then regex "xs\\[1\\] = ([-+]?[0-9]*\\.?[0-9]+)" (f32) |world,value,_step| {
      assert_eq!(world.xs.unwrap().1,value);
    };

  });
}

mod intersections_steps {
  use super::*;
  use rustrt::intersection::{intersection,intersections};
  // Any type that implements cucumber_rust::World + Default can be the world
  steps!(MyWorld => {
    given "shape ← sphere()" |world,_steps| {
      world.shape = Shape::Sphere(sphere());
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
  features: "./features", // Path to our feature files
  world: MyWorld, // The world needs to be the same for steps and the main cucumber call
  steps: &[
      tuple_steps::steps,
      canvas_steps::steps,
      matrix_steps::steps,
      transformations_steps::steps,
      rays_steps::steps,
      spheres_steps::steps,
      intersections_steps::steps
  ],
  setup: setup, // Optional; called once before everything
  before: &[
      a_before_fn // Optional; called before each scenario
  ],
  after: &[
      an_after_fn // Optional; called after each scenario
  ]
}
