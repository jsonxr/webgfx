use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Geometry {
}

// Rust implementation
impl Geometry {
  pub fn new() -> Geometry {
    Geometry {
    }
  }
}

#[wasm_bindgen]
impl Geometry {
  #[wasm_bindgen(constructor)]
  pub fn wasm_new() -> Geometry {
    Geometry {
    }
  }
}
