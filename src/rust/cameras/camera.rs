//use wasm_bindgen::prelude::*;
//use super::super::Vec3;

pub trait Camera {
  fn get_position(&self) -> [f32;3] {
    [0.0, 0.0, 0.0]
  }
}
