use wasm_bindgen::prelude::*;
use super::Geometry;

#[wasm_bindgen( js_name="createBoxGeometry")]
pub fn create_box_geometry(_width: f32, _height: f32, _depth: f32, _options: &JsValue) -> Geometry {
  //let vertices = vec![-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
  Geometry {
    id: 1,
  }
}
