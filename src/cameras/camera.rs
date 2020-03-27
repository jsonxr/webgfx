//use wasm_bindgen::prelude::*;
use super::super::Vec3;

pub trait Camera {
  fn get_position(&self) -> Vec3 {
    Vec3 { x: 0.0, y: 0.0, z: 0.0}
  }
}
