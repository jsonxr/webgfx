
use wasm_bindgen::prelude::*;

use super::BoxGeometry;
use super::MeshBasicMaterial;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
  pub id: u32,
  pub geometry: BoxGeometry,
  pub material: MeshBasicMaterial,
}

#[wasm_bindgen]
impl Mesh {
  #[wasm_bindgen(constructor)]
  pub fn new(geometry: BoxGeometry, material: MeshBasicMaterial) -> Mesh {
    Mesh { 
      id: 1,
      geometry: geometry,
      material: material
    }
  }
}
