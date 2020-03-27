//use wasm_bindgen::prelude::*;
use super::Vec3;

pub trait Camera {
  fn get_position(&self) -> Vec3 {
    Vec3 { x: 0.0, y: 0.0, z: 0.0}
  }
}

mod perspective_camera;
pub use perspective_camera::PerspectiveCamera;
