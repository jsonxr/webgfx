use wasm_bindgen::prelude::*;


//[wasm_bindgen]
pub type Vector3 = [f32; 3];


#[wasm_bindgen(inspectable)]
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}
