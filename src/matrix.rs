use float_cmp::{ApproxEq};
use std::ops::{Index, IndexMut, Mul};
use std::f32;
use crate::tuple::{Tuple,tuple};

#[derive(Debug, Clone)]
pub struct Matrix {
  columns: u32,
  rows: u32,
  data: Vec<f32>
}

pub fn matrix(columns:u32, rows:u32) -> Matrix {  
  let csize = (columns * rows) as usize;
  let data = vec![0.0; csize];
  Matrix {columns,rows,data}
}

/// Identity matrix for 4x4
pub fn id4() -> Matrix {
  let mut m = matrix(4,4);
  m[(0,0)] = 1.0;
  m[(1,1)] = 1.0;
  m[(2,2)] = 1.0;
  m[(3,3)] = 1.0;
  m
}

/// Rotation matrix for the x axis
/// r = radians
pub fn rotation_x(r:f32) -> Matrix {
  let mut m = id4();
  m[(1,1)] = r.cos();
  m[(1,2)] = -r.sin();
  m[(2,1)] = r.sin();
  m[(2,2)] = r.cos();
  m
}

/// Rotation matrix for the y axis
/// r = radians
pub fn rotation_y(r:f32) -> Matrix {
  let mut m = id4();
  m[(0,0)] = r.cos();
  m[(0,2)] = r.sin();
  m[(2,0)] = -r.sin();
  m[(2,2)] = r.cos();
  m
}

/// Rotation matrix for the z axis
/// r = radians
pub fn rotation_z(r:f32) -> Matrix {
  let mut m = id4();
  m[(0,0)] = r.cos();
  m[(0,1)] = -r.sin();
  m[(1,0)] = r.sin();
  m[(1,1)] = r.cos();
  m
}

/// A scaling matrix
pub fn scaling(x:f32, y:f32, z:f32) -> Matrix {
  let mut m = id4();
  m[(0,0)] = x;
  m[(1,1)] = y;
  m[(2,2)] = z;
  m[(3,3)] = 1.0;
  m
}

/// A translation matrix
pub fn translation(x:f32, y:f32, z:f32) -> Matrix {
  let mut m = id4();
  m[(0,3)] = x;
  m[(1,3)] = y;
  m[(2,3)] = z;
  m
}

impl Matrix {
  pub fn columns(&self) -> &u32 {&self.columns}
  pub fn rows(&self) -> &u32 {&self.rows}
  pub fn cofactor(&self,row: usize, col: usize) -> f32 {
    let minor = self.minor(row, col);
    if (row+col)%2 == 0 { minor } else { -minor }
  }
  pub fn determinant(&self) -> f32 {
    if self.rows == 2 {
      self[(0,0)]*self[(1,1)] - self[(0,1)]*self[(1,0)]
    } else {
      (0..self.columns).map(|i| self[(0,i)]*self.cofactor(0,i as usize)).sum()
    }
  }
  pub fn inverse(&self) -> Matrix {
    if !self.is_invertible() {
      panic!("Trying to invert a non-invertible matrix!");
    }
    let mut m = matrix(self.rows, self.columns);
    for row in 0..self.rows {
      for col in 0..self.columns {
        let c = self.cofactor(row as usize,col as usize);
        m[(col,row)] = c / self.determinant();
      }
    }
    m
  }
  pub fn is_invertible(&self) -> bool {
    // TODO check with approx_eq?
    self.determinant() != 0.0
  }
  // determinant of a submatrix
  pub fn minor(&self,row: usize, col: usize) -> f32 {
    let subm = self.submatrix(row,col);
    subm.determinant()
  }
  pub fn submatrix(&self,row: usize, col: usize) -> Matrix {
    if row as u32 >= self.rows || col as u32 >= self.columns {
      panic!("Matrix row/col {:?},{:?} out of range {:?},{:?}", 
        row,col,self.rows(),self.columns());
    }
    let mut m = matrix(self.rows-1, self.columns-1);
    for r in 0..self.rows {
      for c in 0..self.columns {
        // is that something to copy?
        if r != row as u32 && c != col as u32 {
          let new_r = if r > row as u32 { r-1 } else { r };
          let new_c = if c > col as u32 { c-1 } else { c };
          m[(new_r,new_c)] = self[(r,c)];
        }
      }
    }
    m
  }
  pub fn transpose(&self) -> Matrix {
    let mut m = matrix(self.rows, self.columns);
    for row in 0..self.rows {
      for col in 0..self.columns {
        m[(col,row)] = self[(row,col)];
      }
    }
    m
  }
}

impl Index<(u32,u32)> for Matrix {
  type Output = f32;

  fn index(&self, pos:(u32,u32)) -> &f32 {
    let row = pos.0;
    let col = pos.1;
    if row >= self.rows || col >= self.columns {
      panic!("Matrix coordinates {:?},{:?} out of range {:?},{:?}", 
        row,col,self.rows(),self.columns());
    }
    let index = (row*self.columns+col) as usize;
    &(self.data[index])
  }
}
impl IndexMut<(u32,u32)> for Matrix {

  fn index_mut(&mut self, pos:(u32,u32)) -> &mut f32 {
    let row = pos.0;
    let col = pos.1;
    if row >= self.rows || col >= self.columns {
      panic!("Matrix coordinates {:?},{:?} out of range {:?},{:?}", 
        row,col,self.rows(),self.columns());
    }
    let index = (row*self.columns+col) as usize;
    &mut (self.data[index])
  }
}

impl Mul for &Matrix {
  type Output = Matrix;

  fn mul(self, _rhs:&Matrix) -> Matrix {
    if self.rows != _rhs.rows || self.columns != _rhs.columns {
      panic!("Trying to multiply matrices with different sizes: {:?}x{:?} != {:?}x{:?}", 
        self.rows(),self.columns(),_rhs.rows(),_rhs.columns());
    }
    let mut m = matrix(self.rows, self.columns);
    for row in 0..self.rows {
      for col in 0..self.columns {
        m[(row,col)] = (0..self.rows).map(|i| self[(row,i)]*_rhs[(i,col)]).sum();
      }
    }
    m
  }
}

impl Mul<&Tuple> for &Matrix {
  type Output = Tuple;

  fn mul(self, _rhs:&Tuple) -> Tuple {
    if self.rows != 4 || self.columns != 4 {
      panic!("Trying to multiply matrix and tuple with bad size: {:?}x{:?}", 
        self.rows(),self.columns());
    }
    tuple(
      self[(0,0)] * _rhs.get_x() + self[(0,1)] * _rhs.get_y() + self[(0,2)] * _rhs.get_z() + self[(0,3)] * _rhs.get_w(),
      self[(1,0)] * _rhs.get_x() + self[(1,1)] * _rhs.get_y() + self[(1,2)] * _rhs.get_z() + self[(1,3)] * _rhs.get_w(),
      self[(2,0)] * _rhs.get_x() + self[(2,1)] * _rhs.get_y() + self[(2,2)] * _rhs.get_z() + self[(2,3)] * _rhs.get_w(),
      self[(3,0)] * _rhs.get_x() + self[(3,1)] * _rhs.get_y() + self[(3,2)] * _rhs.get_z() + self[(3,3)] * _rhs.get_w()
    )
  }
}

impl PartialEq for Matrix {
  fn eq(&self, other: &Matrix) -> bool {
    if self.rows == other.rows && self.columns == other.columns {
      for row in 0..self.rows {
        for column in 0..self.columns {
          let index = (row*self.columns+column) as usize;
          if !self.data[index].approx_eq(&other.data[index], 2.0 * ::std::f32::EPSILON, 10) {
            return false
          }
        }
      }
      true
    } else {
      false
    }
  }
}

impl ApproxEq for Matrix {
  type Flt = f32;

  fn approx_eq(&self, other: &Matrix, epsilon: f32, ulps: i32) -> bool {
    if self.rows == other.rows && self.columns == other.columns {
      for row in 0..self.rows {
        for column in 0..self.columns {
          let index = (row*self.columns+column) as usize;
          if !self.data[index].approx_eq(&other.data[index], epsilon, ulps) {
            return false
          }
        }
      }
      true
    } else {
      false
    }
  }
}


#[cfg(test)]
mod tests {   
  use super::*; 
  #[test]
  fn fp_comparison1() {
    let mut a = matrix(2,2);
    a[(0,0)] = 123.1000000000001;
    a[(0,1)] = 123.1;
    let mut b = matrix(2,2);
    b[(0,0)] = 123.1;
    b[(0,1)] = 123.1000000000001;
    assert_eq!(a,b);
  }
}
