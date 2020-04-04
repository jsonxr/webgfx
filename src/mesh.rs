
use wasm_bindgen::prelude::*;
use super::Geometry;
use super::MeshBasicMaterial;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Mesh {
  pub id: u32,

  #[wasm_bindgen(skip)]
  pub geometry: Geometry,

  #[wasm_bindgen(skip)]
  pub material: MeshBasicMaterial,
}

#[wasm_bindgen]
impl Mesh {
  #[wasm_bindgen(constructor)]
  pub fn wasm_new(geometry: Geometry, material: MeshBasicMaterial) -> Mesh {
    Mesh { 
      id: 1,
      geometry: geometry,
      material: material,
    }
  }
}
