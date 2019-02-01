use crate::tuple::*;
use derive_builder::Builder;

#[derive(Default, Debug, Builder, Clone, Copy)]
pub struct Material {
  color: Tuple,
  ambient: f32,
  diffuse: f32,
  specular: f32,
  shininess: f32
}

/// Defaut contructor with color white etc
pub fn material() -> Material {
  full_material(color(1.0,1.0,1.0),0.1,0.9,0.9,200.0)
}

/// Constructor with full parameter set
pub fn full_material(color: Tuple, ambient: f32, diffuse: f32, specular: f32, shininess: f32) ->  Material {
  MaterialBuilder::default()
    .color(color)
    .ambient(ambient)
    .diffuse(diffuse)
    .specular(specular)
    .shininess(shininess)
    .build()
    .unwrap()
}

impl Material {
  pub fn color(&self) -> &Tuple { &self.color }
  pub fn set_color(&mut self, color: Tuple) -> &Material {
    self.color = color;
    self
  }
  pub fn ambient(&self) -> f32 { self.ambient }
  pub fn set_ambient(&mut self, ambient: f32) -> &Material {
    self.ambient = ambient;
    self
  }
  pub fn diffuse(&self) -> f32 { self.diffuse }
  pub fn set_diffuse(&mut self, diffuse: f32) -> &Material {
    self.diffuse = diffuse;
    self
  }
  pub fn specular(&self) -> f32 { self.specular }
  pub fn set_specular(&mut self, specular: f32) -> &Material {
    self.specular = specular;
    self
  }
  pub fn shininess(&self) -> f32 { self.shininess }
  pub fn set_shininess(&mut self, shininess: f32) -> &Material {
    self.shininess = shininess;
    self
  }
}

impl PartialEq for Material {
  fn eq(&self, other: &Material) -> bool {
    self.color == other.color &&
    self.ambient == other.ambient &&
    self.diffuse == other.diffuse &&
    self.specular == other.specular &&
    self.shininess == other.shininess
  }
}