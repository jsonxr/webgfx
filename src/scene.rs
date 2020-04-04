use std::vec::Vec;
use wasm_bindgen::prelude::*;
use super::mesh::Mesh;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Scene {
  pub id: u32,

  #[wasm_bindgen(skip)]
  pub meshes: Vec<Mesh>,
}

impl Scene {
  pub fn new() -> Scene {
    Scene { 
      id: 1,
      meshes: Vec::new(),
    }
  }
}

#[wasm_bindgen]
impl Scene {
  #[wasm_bindgen(constructor)]
  pub fn wasm_new() -> Scene {
    return Scene::new();
  }

  #[wasm_bindgen]
  pub fn add(&mut self, mesh: Mesh) {
    self.meshes.push(mesh);
  }

  #[wasm_bindgen]
  pub fn len(&self) -> usize {
    self.meshes.len()
  }
}
