use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene {
  pub id: u32,
}

#[wasm_bindgen]
impl Scene {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Scene {
    Scene { id: 1 }
  }
}
