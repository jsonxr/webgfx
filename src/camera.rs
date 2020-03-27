use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Camera {
  pub id: u32,
}

#[wasm_bindgen]
impl Camera {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Camera {
    Camera { id: 1 }
  }
}
