use wasm_bindgen::prelude::*;
use super::Geometry;


#[wasm_bindgen( js_name="createBoxGeometry")]
pub fn create_box_geometry(width: f32, height: f32, depth: f32, options: &JsValue) -> Geometry {
  Geometry {
  }
}
