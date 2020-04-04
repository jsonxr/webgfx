use wasm_bindgen::prelude::*;

pub use super::super::Vec3;
pub use super::super::Vector3;

#[wasm_bindgen(inspectable)]
#[derive(Debug)]
pub struct PerspectiveCamera {
  #[wasm_bindgen(readonly)]
  pub id: u32,

  #[wasm_bindgen(skip)]
  pub position: [f32;3],
}

#[wasm_bindgen]
impl PerspectiveCamera {
  #[wasm_bindgen(constructor)]
  pub fn new() -> PerspectiveCamera {
    PerspectiveCamera {
      id: 1,
      position: [0.0, 0.0, 0.0],
    }
  }
}
use super::Camera;

impl Camera for PerspectiveCamera {
  fn get_position(&self) -> [f32;3] {
    self.position
  }
}
