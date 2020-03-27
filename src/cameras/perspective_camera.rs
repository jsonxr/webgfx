use wasm_bindgen::prelude::*;

pub use super::super::Vec3;
pub use super::super::Vector3;

#[wasm_bindgen(inspectable)]
#[derive(Debug, Copy, Clone)]
pub struct PerspectiveCamera {
  #[wasm_bindgen(readonly)]
  pub id: u32,

  #[wasm_bindgen(readonly)]
  pub position: Vec3,
}

#[wasm_bindgen]
impl PerspectiveCamera {
  #[wasm_bindgen(constructor)]
  pub fn new() -> PerspectiveCamera {
    PerspectiveCamera {
      id: 1,
      position: Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
  }
}
use super::Camera;

impl Camera for PerspectiveCamera {
  fn get_position(&self) -> Vec3 {
    self.position
  }
}
