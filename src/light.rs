use crate::tuple::*;
use derive_builder::Builder;

#[derive(Default, Debug, Builder, Clone, Copy)]
pub struct Light {
  intensity: Tuple,
  position: Tuple
}

pub fn point_light(position: Tuple, intensity: Tuple) ->  Light {
  LightBuilder::default()
    .intensity(intensity)
    .position(position)
    .build()
    .unwrap()
}

impl Light {
  pub fn intensity(&self) -> &Tuple { &self.intensity }
  pub fn position(&self) -> &Tuple { &self.position }
}
