use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct BoxGeometry {
  pub width: f32,
  pub height: f32,
  pub depth: f32,
}

//width : Float, height : Float, depth : Float, widthSegments : Integer, heightSegments : Integer, depthSegments : Integer


#[wasm_bindgen]
impl BoxGeometry {
  #[wasm_bindgen(constructor)]
  pub fn new(width: f32, height: f32, depth: f32) -> BoxGeometry {
    BoxGeometry {
      width,
      height,
      depth
    }
  }
}
