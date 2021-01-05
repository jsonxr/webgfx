use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct MeshBasicMaterial {
  pub id: u32,
}

#[wasm_bindgen]
impl MeshBasicMaterial {
  #[wasm_bindgen(constructor)]
  pub fn new() -> MeshBasicMaterial {
    MeshBasicMaterial { id: 1 }
  }
}
