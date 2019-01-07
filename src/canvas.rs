use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub struct Canvas {
  width: i32,
  height: i32,
  pixels: Vec<Tuple>
}

pub fn canvas(width:i32, height:i32) -> Canvas {  
  let csize = (width * height) as usize;
  let pixels = vec![color(0.0,0.0,0.0); csize];
  Canvas {width,height,pixels}
}

pub fn canvas_with_color(width:i32, height:i32, color: Tuple) -> Canvas {  
  let csize = (width * height) as usize;
  let pixels = vec![color; csize];
  Canvas {width,height,pixels}
}

impl Canvas {
  pub fn height(&self) -> &i32 {&self.height}
  pub fn width(&self) -> &i32 {&self.width}
  pub fn pixels(&self) -> &Vec<Tuple> {&self.pixels}

  // Clear the canvas by setting all pixels to black
  pub fn clear(&mut self) -> () {
    let black = color(0.0,0.0,0.0);
    for y in 0..self.height {
      for x in 0..self.width {
        self.write_pixel(x,y,black);
      }
    }  
  }

  // Clear the canvas by setting all pixels to a color
  pub fn clear_to_color(&mut self, color: Tuple) -> () {
    for y in 0..self.height {
      for x in 0..self.width {
        self.write_pixel(x,y,color);
      }
    }  
  }

  /// Get the color of a pixel at position x,y
  pub fn pixel_at(&self, x: i32, y: i32) -> Tuple {
    let index = (y*self.width+x) as usize;
    self.pixels[index]
  }

  fn scale_color_256(color: f32) -> u8 {
    if color < 0.0 {0}
    else if color >= 1.0 {255}
    else {
      (256.0 * color) as u8
    }
  }

  fn fix_line(line: &String) -> String {
    let v:Vec<&str> = line.split_terminator(' ').collect();
    let mut pctr = 0;
    let mut result = String::with_capacity(70);
    for p in v.iter() {
      result.push_str(p);
      pctr += 1;
      if p.ends_with('\n') {
        pctr = 0;
      } else if (pctr+1) * 4 >= 70 {
        result.push('\n');
        pctr = 0;
      } else {
        result.push(' ');
      }
    }
    result
  } 

  /// Return a string representation of the canvas in PPM format
  pub fn canvas_to_ppm(&self) -> String {
    let capacity = self.width * self.height * 4;
    let header = format!("P3\n{} {}\n255\n",self.width, self.height);
    let mut body = String::with_capacity(capacity as usize);
    let mut line = String::with_capacity(70);
    for y in 0..self.height {
      for x in 0..self.width {
        let pixel = self.pixel_at(x,y);
        let terminator = if x == (self.width-1) { "\n"} else { " " };
        let pixelvals = format!("{} {} {}{}",
          Canvas::scale_color_256(*pixel.red()), 
          Canvas::scale_color_256(*pixel.green()), 
          Canvas::scale_color_256(*pixel.blue()), 
          terminator);
        line.push_str(&pixelvals);
      }
      let pline = Canvas::fix_line(&line);
      body.push_str(&pline);
      line.clear();
    }
    header + &body
  }

  /// Set the color of a pixel at position x,y
  pub fn write_pixel(&mut self, x: i32, y: i32, color: Tuple) {
    if x < 0 || x > self.width || y < 0 || y > self.height {
      // TODO log error?
      return;
    }
    let index = (y*self.width+x) as usize;
    self.pixels[index] = color;
  }
}