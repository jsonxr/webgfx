use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Geometry {
  pub id: u32,

}

pub struct Geo<'life> {
  pub id: u32,
  pub vertices: &'life [f32],
}

#[wasm_bindgen]
impl Geometry {
  #[wasm_bindgen(constructor)]
  pub fn wasm_new(_vertices: Box<[JsValue]>) -> Geometry {
    Geometry {
      id: 1,
    }
  }
}
